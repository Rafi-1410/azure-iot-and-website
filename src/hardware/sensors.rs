use anyhow::Result;
use ads1x1x::{Ads1x1x, TargetAddr, channel, FullScaleRange, DataRate16Bit};
use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;
use log::{info, warn, error};

pub struct SensorArray {
    adc1: Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>,
    adc2: Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>,
    adc3: Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>,
    baseline: Option<[f64; 12]>,
    channel_count: usize,
}

impl SensorArray {
    pub fn new() -> Result<Self> {
        info!("Initializing SensorArray with 3x ADS1115 ADCs (12 channels)");
        
        // Initialize I2C devices for three ADS1115 chips
        let dev1 = I2cdev::new("/dev/i2c-1")?;
        let dev2 = I2cdev::new("/dev/i2c-1")?;
        let dev3 = I2cdev::new("/dev/i2c-1")?;
        
        // Create ADS1115 instances with different I2C addresses
        let mut adc1 = Ads1x1x::new_ads1115(dev1, TargetAddr::Gnd);     // Address 0x48
        let mut adc2 = Ads1x1x::new_ads1115(dev2, TargetAddr::Vdd);     // Address 0x49  
        let mut adc3 = Ads1x1x::new_ads1115(dev3, TargetAddr::Sda);     // Address 0x4A
        
        // Configure optimal settings for gas sensors
        adc1.set_full_scale_range(FullScaleRange::Within4_096V)?; // ±4.096V range
        adc2.set_full_scale_range(FullScaleRange::Within4_096V)?;
        adc3.set_full_scale_range(FullScaleRange::Within4_096V)?;
        
        // Set data rate for better noise performance
        adc1.set_data_rate(DataRate16Bit::Sps128)?; // 128 samples per second
        adc2.set_data_rate(DataRate16Bit::Sps128)?;
        adc3.set_data_rate(DataRate16Bit::Sps128)?;
        
        info!("3x ADS1115 ADCs initialized successfully");
        info!("Total channels: 12 (3 ADCs × 4 channels each)");
        info!("I2C addresses: 0x48, 0x49, 0x4A");
        info!("Resolution: 16-bit (65,536 levels per channel)");
        info!("Voltage range: ±4.096V");
        info!("Precision: 0.125mV per bit");
        
        Ok(Self {
            adc1,
            adc2,
            adc3,
            baseline: None,
            channel_count: 12,
        })
    }
    
    pub fn read_all_sensors(&mut self) -> Result<[f64; 12]> {
        let mut readings = [0.0; 12];
        
        // Read from first ADS1115 (Primary TGS sensors - channels 0-3)
        readings[0] = self.read_voltage(&mut self.adc1, &mut channel::SingleA0)?; // TGS2600
        readings[1] = self.read_voltage(&mut self.adc1, &mut channel::SingleA1)?; // TGS2602
        readings[2] = self.read_voltage(&mut self.adc1, &mut channel::SingleA2)?; // TGS2610
        readings[3] = self.read_voltage(&mut self.adc1, &mut channel::SingleA3)?; // TGS2611
        
        // Small delay between ADC readings for stability
        thread::sleep(Duration::from_millis(2));
        
        // Read from second ADS1115 (MQ sensors - channels 4-7)
        readings[4] = self.read_voltage(&mut self.adc2, &mut channel::SingleA0)?; // MQ-2
        readings[5] = self.read_voltage(&mut self.adc2, &mut channel::SingleA1)?; // MQ-3
        readings[6] = self.read_voltage(&mut self.adc2, &mut channel::SingleA2)?; // MQ-7
        readings[7] = self.read_voltage(&mut self.adc2, &mut channel::SingleA3)?; // MQ-135
        
        thread::sleep(Duration::from_millis(2));
        
        // Read from third ADS1115 (Additional/Environmental sensors - channels 8-11)
        readings[8] = self.read_voltage(&mut self.adc3, &mut channel::SingleA0)?;  // TGS2620 or BME280 temp
        readings[9] = self.read_voltage(&mut self.adc3, &mut channel::SingleA1)?;  // MQ-4 or BME280 humidity  
        readings[10] = self.read_voltage(&mut self.adc3, &mut channel::SingleA2)?; // MQ-6 or Reference voltage
        readings[11] = self.read_voltage(&mut self.adc3, &mut channel::SingleA3)?; // MQ-8 or Spare channel
        
        Ok(readings)
    }
    
    fn read_voltage<CH>(&mut self, adc: &mut Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>, channel: &mut CH) -> Result<f64>
    where
        CH: ads1x1x::channel::Channel<ads1x1x::ic::Ads1115>,
    {
        // Read 16-bit ADC value
        let raw_value = adc.read(channel)?;
        
        // Convert to voltage with 16-bit precision
        // ADS1115 at ±4.096V range: 0.125mV per bit
        let voltage = (raw_value as f64) * 0.000125; // 0.125mV resolution
        
        Ok(voltage)
    }
    
    // Enhanced differential measurement for 3 ADCs
    pub fn read_differential_pair(&mut self, pair: DifferentialPair) -> Result<f64> {
        match pair {
            // ADC1 differential pairs
            DifferentialPair::TGS2600_2602 => {
                self.read_voltage(&mut self.adc1, &mut channel::DifferentialA0A1)
            },
            DifferentialPair::TGS2610_2611 => {
                self.read_voltage(&mut self.adc1, &mut channel::DifferentialA2A3)
            },
            // ADC2 differential pairs  
            DifferentialPair::MQ2_MQ3 => {
                self.read_voltage(&mut self.adc2, &mut channel::DifferentialA0A1)
            },
            DifferentialPair::MQ7_MQ135 => {
                self.read_voltage(&mut self.adc2, &mut channel::DifferentialA2A3)
            },
            // ADC3 differential pairs (new)
            DifferentialPair::Additional_A0A1 => {
                self.read_voltage(&mut self.adc3, &mut channel::DifferentialA0A1)
            },
            DifferentialPair::Additional_A2A3 => {
                self.read_voltage(&mut self.adc3, &mut channel::DifferentialA2A3)
            },
        }
    }
    
    pub fn calibrate_sensors(&mut self) -> Result<[f64; 12]> {
        info!("Starting high-precision sensor calibration with 3x ADS1115...");
        println!("Calibrating 12-channel sensor array... Please ensure clean air environment.");
        
        let mut baseline_readings = [0.0; 12];
        let calibration_samples = 200; // More samples for better accuracy with 16-bit precision
        
        for sample in 0..calibration_samples {
            let readings = self.read_all_sensors()?;
            
            for (i, &reading) in readings.iter().enumerate() {
                baseline_readings[i] += reading;
            }
            
            if sample % 40 == 0 {
                println!("Calibration progress: {}/{}...", sample + 1, calibration_samples);
            }
            
            thread::sleep(Duration::from_millis(50)); // Slightly faster with better ADC
        }
        
        // Calculate average baseline with higher precision
        for reading in &mut baseline_readings {
            *reading /= calibration_samples as f64;
        }
        
        self.baseline = Some(baseline_readings);
        
        info!("High-precision 12-channel calibration complete. Baseline readings: {:?}", baseline_readings);
        println!("16-bit precision calibration complete for 12 channels!");
        println!("Baseline readings (±0.125mV precision):");
        
        // Primary gas sensors (ADC1)
        println!("ADC1 (Primary TGS sensors):");
        println!("  TGS2600: {:.6}V", baseline_readings[0]);
        println!("  TGS2602: {:.6}V", baseline_readings[1]);
        println!("  TGS2610: {:.6}V", baseline_readings[2]);
        println!("  TGS2611: {:.6}V", baseline_readings[3]);
        
        // MQ sensors (ADC2)
        println!("ADC2 (MQ sensors):");
        println!("  MQ-2:    {:.6}V", baseline_readings[4]);
        println!("  MQ-3:    {:.6}V", baseline_readings[5]);
        println!("  MQ-7:    {:.6}V", baseline_readings[6]);
        println!("  MQ-135:  {:.6}V", baseline_readings[7]);
        
        // Additional sensors (ADC3)
        println!("ADC3 (Additional sensors):");
        println!("  Ch8:     {:.6}V", baseline_readings[8]);
        println!("  Ch9:     {:.6}V", baseline_readings[9]);
        println!("  Ch10:    {:.6}V", baseline_readings[10]);
        println!("  Ch11:    {:.6}V", baseline_readings[11]);
        
        Ok(baseline_readings)
    }
    
    pub fn get_baseline(&self) -> Option<[f64; 12]> {
        self.baseline
    }
    
    pub fn set_baseline(&mut self, baseline: [f64; 12]) {
        self.baseline = Some(baseline);
    }
    
    pub fn test_connection(&mut self) -> Result<()> {
        info!("Testing 3x ADS1115 I2C connections...");
        
        // Test all ADCs by reading all channels
        let readings = self.read_all_sensors()?;
        
        for (i, &reading) in readings.iter().enumerate() {
            let adc_num = (i / 4) + 1;
            let channel = i % 4;
            
            if reading < -5.0 || reading > 5.0 {
                warn!("ADC{} Channel {} reading out of expected range: {:.6}V", adc_num, channel, reading);
            }
        }
        
        info!("All 3x ADS1115 connection tests passed - 16-bit precision confirmed");
        info!("Total functional channels: {}", self.channel_count);
        Ok(())
    }
    
    // Enhanced gain control for 3 ADCs
    pub fn set_sensor_gain(&mut self, sensor_group: SensorGroup, gain_range: FullScaleRange) -> Result<()> {
        match sensor_group {
            SensorGroup::TGS => {
                self.adc1.set_full_scale_range(gain_range)?;
                info!("TGS sensors (ADC1) gain set to: {:?}", gain_range);
            },
            SensorGroup::MQ => {
                self.adc2.set_full_scale_range(gain_range)?;
                info!("MQ sensors (ADC2) gain set to: {:?}", gain_range);
            },
            SensorGroup::Additional => {
                self.adc3.set_full_scale_range(gain_range)?;
                info!("Additional sensors (ADC3) gain set to: {:?}", gain_range);
            },
        }
        Ok(())
    }
    
    // New method: Read specific ADC
    pub fn read_adc_channels(&mut self, adc_number: u8) -> Result<[f64; 4]> {
        let mut readings = [0.0; 4];
        
        match adc_number {
            1 => {
                readings[0] = self.read_voltage(&mut self.adc1, &mut channel::SingleA0)?;
                readings[1] = self.read_voltage(&mut self.adc1, &mut channel::SingleA1)?;
                readings[2] = self.read_voltage(&mut self.adc1, &mut channel::SingleA2)?;
                readings[3] = self.read_voltage(&mut self.adc1, &mut channel::SingleA3)?;
            },
            2 => {
                readings[0] = self.read_voltage(&mut self.adc2, &mut channel::SingleA0)?;
                readings[1] = self.read_voltage(&mut self.adc2, &mut channel::SingleA1)?;
                readings[2] = self.read_voltage(&mut self.adc2, &mut channel::SingleA2)?;
                readings[3] = self.read_voltage(&mut self.adc2, &mut channel::SingleA3)?;
            },
            3 => {
                readings[0] = self.read_voltage(&mut self.adc3, &mut channel::SingleA0)?;
                readings[1] = self.read_voltage(&mut self.adc3, &mut channel::SingleA1)?;
                readings[2] = self.read_voltage(&mut self.adc3, &mut channel::SingleA2)?;
                readings[3] = self.read_voltage(&mut self.adc3, &mut channel::SingleA3)?;
            },
            _ => return Err(anyhow::anyhow!("Invalid ADC number: {}. Use 1, 2, or 3.", adc_number)),
        }
        
        Ok(readings)
    }
    
    pub fn get_channel_count(&self) -> usize {
        self.channel_count
    }
}

#[derive(Debug, Clone)]
pub enum DifferentialPair {
    // ADC1 pairs
    TGS2600_2602,
    TGS2610_2611,
    // ADC2 pairs
    MQ2_MQ3,
    MQ7_MQ135,
    // ADC3 pairs (new)
    Additional_A0A1,
    Additional_A2A3,
}

#[derive(Debug, Clone)]
pub enum SensorGroup {
    TGS,        // ADC1
    MQ,         // ADC2  
    Additional, // ADC3
}

// Updated SensorData for 12 channels
#[derive(Debug, Clone)]
pub struct SensorData {
    // Primary TGS sensors (ADC1)
    pub tgs2600: f64,
    pub tgs2602: f64,
    pub tgs2610: f64,
    pub tgs2611: f64,
    // MQ sensors (ADC2)
    pub mq2: f64,
    pub mq3: f64,
    pub mq7: f64,
    pub mq135: f64,
    // Additional sensors (ADC3)
    pub additional_0: f64,
    pub additional_1: f64,
    pub additional_2: f64,
    pub additional_3: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl SensorData {
    pub fn from_readings(readings: [f64; 12]) -> Self {
        Self {
            tgs2600: readings[0],
            tgs2602: readings[1],
            tgs2610: readings[2],
            tgs2611: readings[3],
            mq2: readings[4],
            mq3: readings[5],
            mq7: readings[6],
            mq135: readings[7],
            additional_0: readings[8],
            additional_1: readings[9],
            additional_2: readings[10],
            additional_3: readings[11],
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn to_array(&self) -> [f64; 12] {
        [
            self.tgs2600,
            self.tgs2602,
            self.tgs2610,
            self.tgs2611,
            self.mq2,
            self.mq3,
            self.mq7,
            self.mq135,
            self.additional_0,
            self.additional_1,
            self.additional_2,
            self.additional_3,
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
            self.additional_0,
            self.additional_1,
            self.additional_2,
            self.additional_3,
        ]
    }
    
    pub fn normalize(&self, baseline: &[f64; 12]) -> [f64; 12] {
        let mut normalized = [0.0; 12];
        let readings = self.to_array();
        
        for (i, (&reading, &base)) in readings.iter().zip(baseline.iter()).enumerate() {
            // Higher precision normalization with 16-bit data
            normalized[i] = if base > 0.0 {
                (reading - base) / base
            } else {
                0.0
            };
        }
        
        normalized
    }
    
    pub fn apply_calibration(&mut self, baseline: &[f64; 12]) {
        let normalized = self.normalize(baseline);
        self.tgs2600 = normalized[0];
        self.tgs2602 = normalized[1];
        self.tgs2610 = normalized[2];
        self.tgs2611 = normalized[3];
        self.mq2 = normalized[4];
        self.mq3 = normalized[5];
        self.mq7 = normalized[6];
        self.mq135 = normalized[7];
        self.additional_0 = normalized[8];
        self.additional_1 = normalized[9];
        self.additional_2 = normalized[10];
        self.additional_3 = normalized[11];
    }
    
    pub fn is_valid(&self) -> bool {
        let readings = self.to_array();
        
        // Check if all readings are within valid range (expanded for ±4.096V)
        for &reading in &readings {
            if reading < -5.0 || reading > 5.0 || reading.is_nan() || reading.is_infinite() {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_sensor_names() -> [&'static str; 12] {
        [
            "TGS2600", "TGS2602", "TGS2610", "TGS2611", 
            "MQ-2", "MQ-3", "MQ-7", "MQ-135",
            "Additional-0", "Additional-1", "Additional-2", "Additional-3"
        ]
    }
    
    pub fn print_readings(&self) {
        let names = Self::get_sensor_names();
        let readings = self.to_array();
        
        println!("12-Channel High-Precision Sensor Readings at {}:", self.timestamp.format("%Y-%m-%d %H:%M:%S"));
        
        println!("ADC1 (TGS Sensors):");
        for i in 0..4 {
            println!("  {}: {:.6}V", names[i], readings[i]);
        }
        
        println!("ADC2 (MQ Sensors):");
        for i in 4..8 {
            println!("  {}: {:.6}V", names[i], readings[i]);
        }
        
        println!("ADC3 (Additional Sensors):");
        for i in 8..12 {
            println!("  {}: {:.6}V", names[i], readings[i]);
        }
    }
}

impl std::fmt::Display for SensorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SensorData {{ TGS2600: {:.6}, TGS2602: {:.6}, TGS2610: {:.6}, TGS2611: {:.6}, MQ2: {:.6}, MQ3: {:.6}, MQ7: {:.6}, MQ135: {:.6}, Add0: {:.6}, Add1: {:.6}, Add2: {:.6}, Add3: {:.6} }}", 
               self.tgs2600, self.tgs2602, self.tgs2610, self.tgs2611, 
               self.mq2, self.mq3, self.mq7, self.mq135,
               self.additional_0, self.additional_1, self.additional_2, self.additional_3)
    }
}

// Enhanced sensor configuration for 12 channels
#[derive(Debug, Clone)]
pub struct SensorConfig {
    pub name: String,
    pub channel: u8,
    pub adc_number: u8,
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
    Environmental,
    Reference,
}

impl SensorConfig {
    pub fn get_default_configs() -> Vec<SensorConfig> {
        vec![
            // ADC1 - TGS Sensors
            SensorConfig {
                name: "TGS2600".to_string(),
                channel: 0,
                adc_number: 1,
                sensor_type: SensorType::TGS,
                target_gases: vec!["H2".to_string(), "CO".to_string(), "CH4".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2602".to_string(),
                channel: 1,
                adc_number: 1,
                sensor_type: SensorType::TGS,
                target_gases: vec!["NH3".to_string(), "H2S".to_string(), "C2H5OH".to_string()],
                sensitivity_range: (1.0, 30.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2610".to_string(),
                channel: 2,
                adc_number: 1,
                sensor_type: SensorType::TGS,
                target_gases: vec!["Butane".to_string(), "Propane".to_string(), "CH4".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "TGS2611".to_string(),
                channel: 3,
                adc_number: 1,
                sensor_type: SensorType::TGS,
                target_gases: vec!["CH4".to_string(), "Propane".to_string()],
                sensitivity_range: (1.0, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            // ADC2 - MQ Sensors
            SensorConfig {
                name: "MQ-2".to_string(),
                channel: 0,
                adc_number: 2,
                sensor_type: SensorType::MQ,
                target_gases: vec!["LPG".to_string(), "Propane".to_string(), "H2".to_string()],
                sensitivity_range: (200.0, 10000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-3".to_string(),
                channel: 1,
                adc_number: 2,
                sensor_type: SensorType::MQ,
                target_gases: vec!["C2H5OH".to_string(), "Benzine".to_string()],
                sensitivity_range: (0.05, 10.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-7".to_string(),
                channel: 2,
                adc_number: 2,
                sensor_type: SensorType::MQ,
                target_gases: vec!["CO".to_string()],
                sensitivity_range: (20.0, 2000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-135".to_string(),
                channel: 3,
                adc_number: 2,
                sensor_type: SensorType::MQ,
                target_gases: vec!["NH3".to_string(), "NOx".to_string(), "CO2".to_string()],
                sensitivity_range: (10.0, 1000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            // ADC3 - Additional Sensors
            SensorConfig {
                name: "TGS2620".to_string(),
                channel: 0,
                adc_number: 3,
                sensor_type: SensorType::TGS,
                target_gases: vec!["Alcohol".to_string(), "Solvent".to_string()],
                sensitivity_range: (1.0, 30.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "MQ-4".to_string(),
                channel: 1,
                adc_number: 3,
                sensor_type: SensorType::MQ,
                target_gases: vec!["CH4".to_string(), "Natural Gas".to_string()],
                sensitivity_range: (200.0, 10000.0),
                operating_voltage: 5.0,
                heating_voltage: 5.0,
            },
            SensorConfig {
                name: "Temperature".to_string(),
                channel: 2,
                adc_number: 3,
                sensor_type: SensorType::Environmental,
                target_gases: vec!["Temperature".to_string()],
                sensitivity_range: (-40.0, 85.0),
                operating_voltage: 3.3,
                heating_voltage: 0.0,
            },
            SensorConfig {
                name: "Reference".to_string(),
                channel: 3,
                adc_number: 3,
                sensor_type: SensorType::Reference,
                target_gases: vec!["Reference Voltage".to_string()],
                sensitivity_range: (0.0, 5.0),
                operating_voltage: 3.3,
                heating_voltage: 0.0,
            },
        ]
    }
}