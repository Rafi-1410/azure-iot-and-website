use anyhow::Result;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread;
use std::time::Duration;
use log::{info, warn, error};

pub struct SensorArray {
    spi: Spi,
    channels: [u8; 8],
    baseline: Option<[f64; 8]>,
}

impl SensorArray {
    pub fn new() -> Result<Self> {
        info!("Initializing SensorArray with SPI communication");
        
        // Initialize SPI with 1MHz clock speed
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
        
        Ok(Self {
            spi,
            channels: [0, 1, 2, 3, 4, 5, 6, 7], // MCP3008 channels 0-7
            baseline: None,
        })
    }
    
    pub fn read_all_sensors(&mut self) -> Result<[f64; 8]> {
        let mut readings = [0.0; 8];
        
        for (i, &channel) in self.channels.iter().enumerate() {
            readings[i] = self.read_channel(channel)?;
            thread::sleep(Duration::from_millis(10)); // Small delay between readings
        }
        
        Ok(readings)
    }
    
    fn read_channel(&mut self, channel: u8) -> Result<f64> {
        if channel > 7 {
            return Err(anyhow::anyhow!("Invalid channel: {}", channel));
        }
        
        // MCP3008 command structure: start bit + single/diff + channel + don't care
        let command = [
            0x01,                           // Start bit
            (0x08 | channel) << 4,          // Single-ended + channel
            0x00,                           // Don't care
        ];
        
        let mut response = [0u8; 3];
        self.spi.transfer(&mut response, &command)?;
        
        // Extract 10-bit value from response
        let raw_value = (((response[1] & 0x03) as u16) << 8) | (response[2] as u16);
        
        // Convert to voltage (0-3.3V range)
        let voltage = (raw_value as f64 / 1024.0) * 3.3;
        
        Ok(voltage)
    }
    
    pub fn calibrate_sensors(&mut self) -> Result<[f64; 8]> {
        info!("Starting sensor calibration...");
        println!("Calibrating sensors... Please ensure clean air environment.");
        
        let mut baseline_readings = [0.0; 8];
        let calibration_samples = 100;
        
        for sample in 0..calibration_samples {
            let readings = self.read_all_sensors()?;
            
            for (i, &reading) in readings.iter().enumerate() {
                baseline_readings[i] += reading;
            }
            
            if sample % 20 == 0 {
                println!("Calibration progress: {}/{}...", sample + 1, calibration_samples);
            }
            
            thread::sleep(Duration::from_millis(100));
        }
        
        // Calculate average baseline
        for reading in &mut baseline_readings {
            *reading /= calibration_samples as f64;
        }
        
        self.baseline = Some(baseline_readings);
        
        info!("Calibration complete. Baseline readings: {:?}", baseline_readings);
        println!("Calibration complete!");
        println!("Baseline readings:");
        println!("  TGS2600: {:.3}V", baseline_readings[0]);
        println!("  TGS2602: {:.3}V", baseline_readings[1]);
        println!("  TGS2610: {:.3}V", baseline_readings[2]);
        println!("  TGS2611: {:.3}V", baseline_readings[3]);
        println!("  MQ-2:    {:.3}V", baseline_readings[4]);
        println!("  MQ-3:    {:.3}V", baseline_readings[5]);
        println!("  MQ-7:    {:.3}V", baseline_readings[6]);
        println!("  MQ-135:  {:.3}V", baseline_readings[7]);
        
        Ok(baseline_readings)
    }
    
    pub fn get_baseline(&self) -> Option<[f64; 8]> {
        self.baseline
    }
    
    pub fn set_baseline(&mut self, baseline: [f64; 8]) {
        self.baseline = Some(baseline);
    }
    
    pub fn test_connection(&mut self) -> Result<()> {
        info!("Testing SPI connection...");
        
        // Try to read from all channels
        for channel in 0..8 {
            let reading = self.read_channel(channel)?;
            if reading < 0.0 || reading > 3.3 {
                warn!("Channel {} reading out of range: {:.3}V", channel, reading);
            }
        }
        
        info!("SPI connection test passed");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SensorData {
    pub tgs2600: f64,
    pub tgs2602: f64,
    pub tgs2610: f64,
    pub tgs2611: f64,
    pub mq2: f64,
    pub mq3: f64,
    pub mq7: f64,
    pub mq135: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl SensorData {
    pub fn from_readings(readings: [f64; 8]) -> Self {
        Self {
            tgs2600: readings[0],
            tgs2602: readings[1],
            tgs2610: readings[2],
            tgs2611: readings[3],
            mq2: readings[4],
            mq3: readings[5],
            mq7: readings[6],
            mq135: readings[7],
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn to_array(&self) -> [f64; 8] {
        [
            self.tgs2600,
            self.tgs2602,
            self.tgs2610,
            self.tgs2611,
            self.mq2,
            self.mq3,
            self.mq7,
            self.mq135,
        ]
    }
    
    pub fn to_vec(&self) -> Vec<f64> {
        vec![
            self.tgs2600,
            self.tgs2602,
            self.tgs2610,
            self.tgs2611,
            self.mq2,
            self.mq3,
            self.mq7,
            self.mq135,
        ]
    }
    
    pub fn normalize(&self, baseline: &[f64; 8]) -> [f64; 8] {
        let mut normalized = [0.0; 8];
        let readings = self.to_array();
        
        for (i, (&reading, &base)) in readings.iter().zip(baseline.iter()).enumerate() {
            // Normalize using (reading - baseline) / baseline
            normalized[i] = if base > 0.0 {
                (reading - base) / base
            } else {
                0.0
            };
        }
        
        normalized
    }
    
    pub fn apply_calibration(&mut self, baseline: &[f64; 8]) {
        let normalized = self.normalize(baseline);
        self.tgs2600 = normalized[0];
        self.tgs2602 = normalized[1];
        self.tgs2610 = normalized[2];
        self.tgs2611 = normalized[3];
        self.mq2 = normalized[4];
        self.mq3 = normalized[5];
        self.mq7 = normalized[6];
        self.mq135 = normalized[7];
    }
    
    pub fn is_valid(&self) -> bool {
        let readings = self.to_array();
        
        // Check if all readings are within valid range
        for &reading in &readings {
            if reading < 0.0 || reading > 5.0 || reading.is_nan() || reading.is_infinite() {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_sensor_names() -> [&'static str; 8] {
        ["TGS2600", "TGS2602", "TGS2610", "TGS2611", "MQ-2", "MQ-3", "MQ-7", "MQ-135"]
    }
    
    pub fn print_readings(&self) {
        let names = Self::get_sensor_names();
        let readings = self.to_array();
        
        println!("Sensor Readings at {}:", self.timestamp.format("%Y-%m-%d %H:%M:%S"));
        for (name, reading) in names.iter().zip(readings.iter()) {
            println!("  {}: {:.3}V", name, reading);
        }
    }
}

impl std::fmt::Display for SensorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SensorData {{ TGS2600: {:.3}, TGS2602: {:.3}, TGS2610: {:.3}, TGS2611: {:.3}, MQ2: {:.3}, MQ3: {:.3}, MQ7: {:.3}, MQ135: {:.3} }}", 
               self.tgs2600, self.tgs2602, self.tgs2610, self.tgs2611, 
               self.mq2, self.mq3, self.mq7, self.mq135)
    }
}

// Sensor configuration and metadata
#[derive(Debug, Clone)]
pub struct SensorConfig {
    pub name: String,
    pub channel: u8,
    pub sensor_type: SensorType,
    pub target_gases: Vec<String>,
    pub sensitivity_range: (f64, f64),
    pub operating_voltage: f64,
    pub heating_voltage: f64,
}

#[derive(Debug, Clone)]
pub enum SensorType {
    TGS,
    MQ,
}

impl SensorConfig {
    pub fn get_default_configs() -> Vec<SensorConfig> {
        vec![
            SensorConfig {
                name: "TGS2600".to_string(),
                channel: 0,
                sensor_type: SensorType::TGS,
                target_gases: vec!["H2".to_string(), "CO".to_string(), "CH4".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2602".to_string(),
                channel: 1,
                sensor_type: SensorType::TGS,
                target_gases: vec!["NH3".to_string(), "H2S".to_string(), "C2H5OH".to_string()],
                sensitivity_range: (1.0, 30.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2610".to_string(),
                channel: 2,
                sensor_type: SensorType::TGS,
                target_gases: vec!["Butane".to_string(), "Propane".to_string(), "CH4".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2611".to_string(),
                channel: 3,
                sensor_type: SensorType::TGS,
                target_gases: vec!["CH4".to_string(), "Propane".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-2".to_string(),
                channel: 4,
                sensor_type: SensorType::MQ,
                target_gases: vec!["LPG".to_string(), "Propane".to_string(), "H2".to_string()],
                sensitivity_range: (200.0, 10000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-3".to_string(),
                channel: 5,
                sensor_type: SensorType::MQ,
                target_gases: vec!["C2H5OH".to_string(), "Benzine".to_string()],
                sensitivity_range: (0.05, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-7".to_string(),
                channel: 6,
                sensor_type: SensorType::MQ,
                target_gases: vec!["CO".to_string()],
                sensitivity_range: (20.0, 2000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-135".to_string(),
                channel: 7,
                sensor_type: SensorType::MQ,
                target_gases: vec!["NH3".to_string(), "NOx".to_string(), "CO2".to_string()],
                sensitivity_range: (10.0, 1000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
        ]
    }
}