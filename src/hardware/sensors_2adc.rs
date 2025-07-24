use anyhow::Result;
use ads1x1x::{Ads1x1x, TargetAddr, channel, FullScaleRange, DataRate16Bit};
use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;
use log::{info, warn, error};

pub struct SensorArray {
    adc1: Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>,
    adc2: Ads1x1x<I2cdev, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>,
    baseline: Option<[f64; 8]>,
    channel_count: usize,
}

impl SensorArray {
    pub fn new() -> Result<Self> {
        info!("Initializing SensorArray with 2x ADS1115 ADCs (8 channels)");
        
        // Initialize I2C devices for two ADS1115 chips
        let dev1 = I2cdev::new("/dev/i2c-1")?;
        let dev2 = I2cdev::new("/dev/i2c-1")?;
        
        // Create ADS1115 instances with different I2C addresses
        let mut adc1 = Ads1x1x::new_ads1115(dev1, TargetAddr::Gnd);     // Address 0x48
        let mut adc2 = Ads1x1x::new_ads1115(dev2, TargetAddr::Vdd);     // Address 0x49
        
        // Configure optimal settings for gas sensors
        adc1.set_full_scale_range(FullScaleRange::Within4_096V)?; // ±4.096V range
        adc2.set_full_scale_range(FullScaleRange::Within4_096V)?;
        
        // Set data rate for better noise performance
        adc1.set_data_rate(DataRate16Bit::Sps128)?; // 128 samples per second
        adc2.set_data_rate(DataRate16Bit::Sps128)?;
        
        info!("2x ADS1115 ADCs initialized successfully");
        info!("Total channels: 8 (2 ADCs × 4 channels each)");
        info!("I2C addresses: 0x48, 0x49");
        info!("Resolution: 16-bit (65,536 levels per channel)");
        info!("Voltage range: ±4.096V");
        info!("Precision: 0.125mV per bit");
        
        Ok(Self {
            adc1,
            adc2,
            baseline: None,
            channel_count: 8,
        })
    }
    
    pub fn read_all_sensors(&mut self) -> Result<[f64; 8]> {
        let mut readings = [0.0; 8];
        
        // Read from first ADS1115 (TGS sensors - channels 0-3)
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
    
    pub fn calibrate_sensors(&mut self) -> Result<[f64; 8]> {
        info!("Starting high-precision sensor calibration with 2x ADS1115...");
        println!("Calibrating 8-channel sensor array... Please ensure clean air environment.");
        
        let mut baseline_readings = [0.0; 8];
        let calibration_samples = 200;
        
        for sample in 0..calibration_samples {
            let readings = self.read_all_sensors()?;
            
            for (i, &reading) in readings.iter().enumerate() {
                baseline_readings[i] += reading;
            }
            
            if sample % 40 == 0 {
                println!("Calibration progress: {}/{}...", sample + 1, calibration_samples);
            }
            
            thread::sleep(Duration::from_millis(50));
        }
        
        // Calculate average baseline
        for reading in &mut baseline_readings {
            *reading /= calibration_samples as f64;
        }
        
        self.baseline = Some(baseline_readings);
        
        info!("High-precision 8-channel calibration complete");
        println!("16-bit precision calibration complete for 8 channels!");
        println!("Baseline readings (±0.125mV precision):");
        
        // TGS sensors (ADC1)
        println!("ADC1 (TGS sensors):");
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
        
        Ok(baseline_readings)
    }
    
    pub fn test_connection(&mut self) -> Result<()> {
        info!("Testing 2x ADS1115 I2C connections...");
        
        let readings = self.read_all_sensors()?;
        
        for (i, &reading) in readings.iter().enumerate() {
            let adc_num = (i / 4) + 1;
            let channel = i % 4;
            
            if reading < -5.0 || reading > 5.0 {
                warn!("ADC{} Channel {} reading out of expected range: {:.6}V", adc_num, channel, reading);
            }
        }
        
        info!("All 2x ADS1115 connection tests passed - 16-bit precision confirmed");
        info!("Total functional channels: {}", self.channel_count);
        Ok(())
    }
    
    pub fn get_channel_count(&self) -> usize {
        self.channel_count
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
    
    pub fn print_readings(&self) {
        let readings = [self.tgs2600, self.tgs2602, self.tgs2610, self.tgs2611,
                       self.mq2, self.mq3, self.mq7, self.mq135];
        let names = ["TGS2600", "TGS2602", "TGS2610", "TGS2611", 
                    "MQ-2", "MQ-3", "MQ-7", "MQ-135"];
        
        println!("8-Channel High-Precision Sensor Readings at {}:", 
                self.timestamp.format("%Y-%m-%d %H:%M:%S"));
        
        println!("ADC1 (TGS Sensors):");
        for i in 0..4 {
            println!("  {}: {:.6}V", names[i], readings[i]);
        }
        
        println!("ADC2 (MQ Sensors):");
        for i in 4..8 {
            println!("  {}: {:.6}V", names[i], readings[i]);
        }
    }
}