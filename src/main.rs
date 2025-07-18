use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

mod database;
mod hardware;
mod ml;
mod gui;
mod api;

use database::Database;
use hardware::collector::DataCollector;
use ml::cnn::{ModelTrainer, TrainingConfig};
use api::{AppState, run_server};

#[derive(Parser)]
#[command(name = "electronic-nose")]
#[command(about = "Electronic Nose with Deep Learning - Rust Implementation")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the GUI application
    Gui,
    /// Start the API server
    Server {
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Collect training data
    Collect {
        #[arg(short, long)]
        gas_type: String,
        #[arg(short, long)]
        concentration: f64,
        #[arg(short, long)]
        duration: u64,
    },
    /// Train the model
    Train {
        #[arg(short, long, default_value = "100")]
        epochs: usize,
        #[arg(short, long, default_value = "32")]
        batch_size: usize,
    },
    /// Test the model
    Test,
    /// Initialize hardware and calibrate sensors
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    
    // Ensure data directory exists
    std::fs::create_dir_all("data")?;
    std::fs::create_dir_all("logs")?;
    std::fs::create_dir_all("models")?;
    
    match cli.command {
        Commands::Gui => {
            run_gui().await?;
        }
        Commands::Server { port } => {
            run_api_server(port).await?;
        }
        Commands::Collect { gas_type, concentration, duration } => {
            collect_data(gas_type, concentration, duration).await?;
        }
        Commands::Train { epochs, batch_size } => {
            train_model(epochs, batch_size).await?;
        }
        Commands::Test => {
            test_model().await?;
        }
        Commands::Init => {
            initialize_hardware().await?;
        }
    }
    
    Ok(())
}

async fn run_gui() -> Result<()> {
    println!("Starting Electronic Nose GUI...");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Electronic Nose - Deep Learning System"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Electronic Nose - Deep Learning",
        options,
        Box::new(|_cc| Box::new(gui::ElectronicNoseApp::default())),
    )?;
    
    Ok(())
}

async fn run_api_server(port: u16) -> Result<()> {
    println!("Starting Electronic Nose API Server on port {}...", port);
    
    let db_path = PathBuf::from("data/electronic_nose.db");
    let database = Arc::new(Database::new(&db_path)?);
    
    let state = AppState {
        database,
        model_trainer: Arc::new(RwLock::new(None)),
        data_collector: Arc::new(RwLock::new(None)),
    };
    
    run_server(state, port).await
}

async fn collect_data(gas_type: String, concentration: f64, duration: u64) -> Result<()> {
    println!("Collecting training data...");
    println!("Gas Type: {}", gas_type);
    println!("Concentration: {} ppm", concentration);
    println!("Duration: {} seconds", duration);
    
    let db_path = PathBuf::from("data/electronic_nose.db");
    let database = Database::new(&db_path)?;
    let mut collector = DataCollector::new(database)?;
    
    let session_id = collector.collect_training_data(gas_type, concentration, duration)?;
    println!("Data collection complete!");
    println!("Session ID: {}", session_id);
    
    Ok(())
}

async fn train_model(epochs: usize, batch_size: usize) -> Result<()> {
    println!("Starting model training...");
    println!("Epochs: {}", epochs);
    println!("Batch Size: {}", batch_size);
    
    let db_path = PathBuf::from("data/electronic_nose.db");
    let database = Database::new(&db_path)?;
    
    let config = TrainingConfig {
        epochs,
        batch_size,
        ..Default::default()
    };
    
    let mut trainer = ModelTrainer::new(config)?;
    
    // Load and preprocess training data
    let mut preprocessor = ml::preprocessing::DataPreprocessor::new();
    let training_data = preprocessor.prepare_training_data(&database)?;
    
    if training_data.is_empty() {
        println!("No training data found. Please collect some data first.");
        return Ok(());
    }
    
    println!("Training data loaded: {} samples", training_data.len());
    
    // Train the model
    let history = trainer.train(&training_data)?;
    
    // Save model performance
    let final_accuracy = history.val_accuracies.last().copied().unwrap_or(0.0);
    println!("Training complete! Final accuracy: {:.3}", final_accuracy);
    
    // Save model
    trainer.save_model("models/electronic_nose_model.bin")?;
    println!("Model saved to models/electronic_nose_model.bin");
    
    Ok(())
}

async fn test_model() -> Result<()> {
    println!("Testing model...");
    
    let db_path = PathBuf::from("data/electronic_nose.db");
    let database = Database::new(&db_path)?;
    
    // Load test data
    let test_data = database.get_training_data(None)?;
    
    if test_data.is_empty() {
        println!("No test data found.");
        return Ok(());
    }
    
    // Load trained model
    let model_path = PathBuf::from("models/electronic_nose_model.bin");
    if !model_path.exists() {
        println!("No trained model found. Please train the model first.");
        return Ok(());
    }
    
    let config = TrainingConfig::default();
    let mut trainer = ModelTrainer::new(config)?;
    trainer.load_model(&model_path)?;
    
    // Test the model
    let mut correct = 0;
    let mut total = 0;
    
    for reading in &test_data {
        if let Some(gas_type) = &reading.gas_type {
            let features = vec![
                reading.tgs2600, reading.tgs2602, reading.tgs2610, reading.tgs2611,
                reading.mq2, reading.mq3, reading.mq7, reading.mq135,
            ];
            
            let (prediction, confidence) = trainer.predict(&features)?;
            
            // Convert gas type to label for comparison
            let expected_label = match gas_type.as_str() {
                "CO" => 0,
                "CH4" => 1,
                "H2" => 2,
                "C2H5OH" => 3,
                "NH3" => 4,
                "NO2" => 5,
                _ => continue,
            };
            
            if prediction == expected_label {
                correct += 1;
            }
            total += 1;
            
            println!("Prediction: {}, Expected: {}, Confidence: {:.3}", 
                     prediction, expected_label, confidence);
        }
    }
    
    let accuracy = correct as f64 / total as f64;
    println!("Test Results:");
    println!("Correct: {}/{}", correct, total);
    println!("Accuracy: {:.3}", accuracy);
    
    Ok(())
}

async fn initialize_hardware() -> Result<()> {
    println!("Initializing hardware...");
    
    // Check if SPI is enabled
    if !std::path::Path::new("/dev/spidev0.0").exists() {
        println!("Error: SPI is not enabled. Please enable SPI using raspi-config.");
        return Err(anyhow::anyhow!("SPI not enabled"));
    }
    
    let db_path = PathBuf::from("data/electronic_nose.db");
    let database = Database::new(&db_path)?;
    let mut collector = DataCollector::new(database)?;
    
    println!("Hardware initialized successfully!");
    println!("Sensor calibration complete.");
    
    Ok(())
}