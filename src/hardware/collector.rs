use super::sensors::{SensorArray, SensorData};
use crate::database::{Database, SensorReading, TrainingSession};
use anyhow::Result;
use chrono::Utc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use log::{info, warn, error};

pub struct DataCollector {
    sensor_array: SensorArray,
    database: Database,
    baseline: [f64; 8],
    is_collecting: bool,
    collection_rate_hz: u64,
}

impl DataCollector {
    pub fn new(database: Database) -> Result<Self> {
        info!("Initializing DataCollector");
        
        let mut sensor_array = SensorArray::new()?;
        
        // Test connection first
        sensor_array.test_connection()?;
        
        // Calibrate sensors
        let baseline = sensor_array.calibrate_sensors()?;
        
        Ok(Self {
            sensor_array,
            database,
            baseline,
            is_collecting: false,
            collection_rate_hz: 10, // 10Hz default sampling rate
        })
    }
    
    pub fn set_collection_rate(&mut self, rate_hz: u64) {
        self.collection_rate_hz = rate_hz;
        info!("Collection rate set to {} Hz", rate_hz);
    }
    
    pub fn start_collection(&mut self, session_id: String, gas_type: Option<String>) -> Result<()> {
        if self.is_collecting {
            return Err(anyhow::anyhow!("Collection already in progress"));
        }
        
        self.is_collecting = true;
        info!("Starting data collection for session: {}", session_id);
        
        // Create training session record
        if let Some(ref gas) = gas_type {
            let session = TrainingSession {
                id: None,
                session_id: session_id.clone(),
                gas_type: gas.clone(),
                concentration: 0.0, // Will be set later
                start_time: Utc::now(),
                end_time: None,
                sample_count: 0,
                notes: None,
            };
            
            self.database.create_training_session(&session)?;
        }
        
        Ok(())
    }
    
    pub fn collect_training_data(
        &mut self,
        gas_type: String,
        concentration: f64,
        duration_seconds: u64,
    ) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        info!("Starting training data collection");
        println!("Starting training data collection for {} at {}ppm", gas_type, concentration);
        println!("Session ID: {}", session_id);
        println!("Duration: {} seconds", duration_seconds);
        
        // Create training session
        let session = TrainingSession {
            id: None,
            session_id: session_id.clone(),
            gas_type: gas_type.clone(),
            concentration,
            start_time: Utc::now(),
            end_time: None,
            sample_count: 0,
            notes: Some("Automated training data collection".to_string()),
        };
        
        self.database.create_training_session(&session)?;
        
        let start_time = Utc::now();
        let total_samples = duration_seconds * self.collection_rate_hz;
        let sample_interval = Duration::from_millis(1000 / self.collection_rate_hz);
        
        println!("Collecting {} samples at {} Hz...", total_samples, self.collection_rate_hz);
        
        let mut samples_collected = 0;
        let mut valid_samples = 0;
        
        for i in 0..total_samples {
            let readings = self.sensor_array.read_all_sensors()?;
            let mut sensor_data = SensorData::from_readings(readings);
            
            // Apply calibration
            sensor_data.apply_calibration(&self.baseline);
            
            // Validate sensor data
            if !sensor_data.is_valid() {
                warn!("Invalid sensor reading at sample {}", i + 1);
                continue;
            }
            
            let sensor_reading = SensorReading {
                id: None,
                timestamp: sensor_data.timestamp,
                session_id: session_id.clone(),
                tgs2600: sensor_data.tgs2600,
                tgs2602: sensor_data.tgs2602,
                tgs2610: sensor_data.tgs2610,
                tgs2611: sensor_data.tgs2611,
                mq2: sensor_data.mq2,
                mq3: sensor_data.mq3,
                mq7: sensor_data.mq7,
                mq135: sensor_data.mq135,
                gas_type: Some(gas_type.clone()),
                concentration: Some(concentration),
                temperature: None,
                humidity: None,
            };
            
            self.database.insert_sensor_reading(&sensor_reading)?;
            
            samples_collected += 1;
            valid_samples += 1;
            
            // Progress reporting
            if samples_collected % (total_samples / 10) == 0 {
                let progress = (samples_collected as f64 / total_samples as f64) * 100.0;
                println!("Progress: {:.1}% ({}/{} samples)", progress, samples_collected, total_samples);
            }
            
            thread::sleep(sample_interval);
        }
        
        let end_time = Utc::now();
        
        // Update training session
        self.database.update_training_session(&session_id, end_time, valid_samples as i32)?;
        
        println!("Training data collection complete!");
        println!("Total samples collected: {}", valid_samples);
        println!("Session duration: {:.1} seconds", 
                 (end_time - start_time).num_milliseconds() as f64 / 1000.0);
        
        info!("Training data collection completed. Session: {}, Samples: {}", 
              session_id, valid_samples);
        
        Ok(session_id)
    }
    
    pub fn collect_continuous(&mut self, tx: mpsc::Sender<SensorData>) -> Result<()> {
        if self.is_collecting {
            return Err(anyhow::anyhow!("Collection already in progress"));
        }
        
        self.is_collecting = true;
        info!("Starting continuous data collection");
        
        let sample_interval = Duration::from_millis(1000 / self.collection_rate_hz);
        
        while self.is_collecting {
            let readings = self.sensor_array.read_all_sensors()?;
            let mut sensor_data = SensorData::from_readings(readings);
            
            // Apply calibration
            sensor_data.apply_calibration(&self.baseline);
            
            // Send data through channel
            if let Err(e) = tx.send(sensor_data) {
                error!("Failed to send sensor data: {}", e);
                break;
            }
            
            thread::sleep(sample_interval);
        }
        
        info!("Continuous data collection stopped");
        Ok(())
    }
    
    pub fn stop_collection(&mut self) {
        self.is_collecting = false;
        info!("Data collection stopped");
    }
    
    pub fn is_collecting(&self) -> bool {
        self.is_collecting
    }
    
    pub fn get_baseline(&self) -> [f64; 8] {
        self.baseline
    }
    
    pub fn recalibrate(&mut self) -> Result<[f64; 8]> {
        info!("Recalibrating sensors");
        
        if self.is_collecting {
            return Err(anyhow::anyhow!("Cannot recalibrate while collecting data"));
        }
        
        self.baseline = self.sensor_array.calibrate_sensors()?;
        
        info!("Recalibration complete");
        Ok(self.baseline)
    }
    
    pub fn read_single_sample(&mut self) -> Result<SensorData> {
        let readings = self.sensor_array.read_all_sensors()?;
        let mut sensor_data = SensorData::from_readings(readings);
        
        // Apply calibration
        sensor_data.apply_calibration(&self.baseline);
        
        Ok(sensor_data)
    }
    
    pub fn test_sensors(&mut self) -> Result<()> {
        info!("Testing all sensors");
        println!("Testing sensor connections...");
        
        // Test SPI connection
        self.sensor_array.test_connection()?;
        
        // Read from all sensors
        let readings = self.sensor_array.read_all_sensors()?;
        let sensor_data = SensorData::from_readings(readings);
        
        // Display readings
        sensor_data.print_readings();
        
        // Validate readings
        if !sensor_data.is_valid() {
            warn!("Some sensor readings are out of range");
            return Err(anyhow::anyhow!("Sensor validation failed"));
        }
        
        println!("All sensors are working properly!");
        info!("Sensor test completed successfully");
        
        Ok(())
    }
    
    pub fn get_sensor_statistics(&self) -> Result<SensorStatistics> {
        let recent_readings = self.database.get_recent_readings(1000)?;
        
        if recent_readings.is_empty() {
            return Ok(SensorStatistics::default());
        }
        
        let mut stats = SensorStatistics::default();
        
        // Calculate statistics for each sensor
        for reading in &recent_readings {
            let values = [
                reading.tgs2600, reading.tgs2602, reading.tgs2610, reading.tgs2611,
                reading.mq2, reading.mq3, reading.mq7, reading.mq135,
            ];
            
            for (i, &value) in values.iter().enumerate() {
                stats.means[i] += value;
                stats.mins[i] = stats.mins[i].min(value);
                stats.maxs[i] = stats.maxs[i].max(value);
            }
        }
        
        // Calculate means
        let count = recent_readings.len() as f64;
        for i in 0..8 {
            stats.means[i] /= count;
        }
        
        // Calculate standard deviations
        for reading in &recent_readings {
            let values = [
                reading.tgs2600, reading.tgs2602, reading.tgs2610, reading.tgs2611,
                reading.mq2, reading.mq3, reading.mq7, reading.mq135,
            ];
            
            for (i, &value) in values.iter().enumerate() {
                let diff = value - stats.means[i];
                stats.stds[i] += diff * diff;
            }
        }
        
        for i in 0..8 {
            stats.stds[i] = (stats.stds[i] / count).sqrt();
        }
        
        stats.sample_count = recent_readings.len();
        
        Ok(stats)
    }
    
    pub fn export_data(&self, session_id: &str, filename: &str) -> Result<()> {
        info!("Exporting data for session: {}", session_id);
        
        let readings = self.database.get_training_data(None)?;
        let session_readings: Vec<_> = readings.into_iter()
            .filter(|r| r.session_id == session_id)
            .collect();
        
        if session_readings.is_empty() {
            return Err(anyhow::anyhow!("No data found for session: {}", session_id));
        }
        
        // Export to CSV
        let mut csv_content = String::new();
        csv_content.push_str("timestamp,tgs2600,tgs2602,tgs2610,tgs2611,mq2,mq3,mq7,mq135,gas_type,concentration\n");
        
        for reading in session_readings {
            csv_content.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                reading.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                reading.tgs2600,
                reading.tgs2602,
                reading.tgs2610,
                reading.tgs2611,
                reading.mq2,
                reading.mq3,
                reading.mq7,
                reading.mq135,
                reading.gas_type.unwrap_or_default(),
                reading.concentration.unwrap_or_default()
            ));
        }
        
        std::fs::write(filename, csv_content)?;
        
        info!("Data exported to: {}", filename);
        println!("Data exported to: {}", filename);
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SensorStatistics {
    pub means: [f64; 8],
    pub stds: [f64; 8],
    pub mins: [f64; 8],
    pub maxs: [f64; 8],
    pub sample_count: usize,
}

impl Default for SensorStatistics {
    fn default() -> Self {
        Self {
            means: [0.0; 8],
            stds: [0.0; 8],
            mins: [f64::INFINITY; 8],
            maxs: [f64::NEG_INFINITY; 8],
            sample_count: 0,
        }
    }
}

impl SensorStatistics {
    pub fn print(&self) {
        let sensor_names = SensorData::get_sensor_names();
        
        println!("Sensor Statistics (last {} samples):", self.sample_count);
        println!("{:<10} {:<8} {:<8} {:<8} {:<8}", "Sensor", "Mean", "Std", "Min", "Max");
        println!("{:-<50}", "");
        
        for (i, name) in sensor_names.iter().enumerate() {
            println!("{:<10} {:<8.3} {:<8.3} {:<8.3} {:<8.3}", 
                     name, self.means[i], self.stds[i], self.mins[i], self.maxs[i]);
        }
    }
}

// Background data collection service
pub struct DataCollectionService {
    collector: DataCollector,
    tx: Option<mpsc::Sender<SensorData>>,
    rx: Option<mpsc::Receiver<SensorData>>,
    is_running: bool,
}

impl DataCollectionService {
    pub fn new(database: Database) -> Result<Self> {
        let collector = DataCollector::new(database)?;
        let (tx, rx) = mpsc::channel();
        
        Ok(Self {
            collector,
            tx: Some(tx),
            rx: Some(rx),
            is_running: false,
        })
    }
    
    pub fn start(&mut self) -> Result<()> {
        if self.is_running {
            return Err(anyhow::anyhow!("Service already running"));
        }
        
        self.is_running = true;
        info!("Starting data collection service");
        
        let tx = self.tx.take().unwrap();
        
        // Start collection in background thread
        thread::spawn(move || {
            // This would need proper error handling in a real implementation
            // For now, we'll just log errors
            if let Err(e) = self.collector.collect_continuous(tx) {
                error!("Data collection service error: {}", e);
            }
        });
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        if !self.is_running {
            return;
        }
        
        self.collector.stop_collection();
        self.is_running = false;
        info!("Data collection service stopped");
    }
    
    pub fn get_latest_data(&mut self) -> Option<SensorData> {
        if let Some(ref rx) = self.rx {
            rx.try_recv().ok()
        } else {
            None
        }
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}