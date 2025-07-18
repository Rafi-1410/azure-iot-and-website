# Electronic Nose Project Structure

## Complete File Organization

This document outlines the complete structure of the Electronic Nose project with all source code files.

```
electronic_nose_rust/
├── Cargo.toml                          # Project configuration and dependencies
├── README.md                           # Project documentation
├── WIRING_TUTORIAL.md                  # Complete hardware wiring guide
├── PROJECT_STRUCTURE.md               # This file
├── build.sh                           # Build script
├── deploy.sh                          # Deployment script
├── .gitignore                         # Git ignore file
├── src/
│   ├── main.rs                        # Main application entry point
│   ├── database/
│   │   └── mod.rs                     # Database operations and models
│   ├── hardware/
│   │   ├── mod.rs                     # Hardware module exports
│   │   ├── sensors.rs                 # Sensor reading and SPI communication
│   │   └── collector.rs               # Data collection and management
│   ├── ml/
│   │   ├── mod.rs                     # Machine learning module exports
│   │   ├── cnn.rs                     # 1D-CNN model implementation
│   │   ├── preprocessing.rs           # Data preprocessing and augmentation
│   │   └── evaluation.rs              # Model evaluation and metrics
│   ├── gui/
│   │   ├── mod.rs                     # GUI module exports
│   │   ├── app.rs                     # Main GUI application
│   │   ├── plots.rs                   # Plotting and visualization
│   │   └── widgets.rs                 # Custom GUI widgets
│   └── api/
│       ├── mod.rs                     # API module exports
│       ├── server.rs                  # REST API server
│       └── handlers.rs                # API request handlers
├── config/
│   ├── settings.toml                  # Application configuration
│   └── schema.sql                     # Database schema
├── data/                              # Data directory (created at runtime)
│   ├── raw/                          # Raw sensor data
│   ├── processed/                    # Processed training data
│   └── electronic_nose.db           # SQLite database
├── models/                           # Trained models (created at runtime)
│   └── electronic_nose_model.bin    # Trained CNN model
├── logs/                             # Log files (created at runtime)
├── tests/
│   ├── integration_tests.rs          # Integration tests
│   ├── hardware_tests.rs             # Hardware testing
│   └── ml_tests.rs                   # ML model tests
└── examples/
    ├── basic_usage.rs                # Basic usage examples
    ├── data_collection.rs            # Data collection examples
    └── model_training.rs             # Model training examples
```

## File Descriptions

### Core Application Files

#### `src/main.rs`
- **Purpose**: Main application entry point with CLI interface
- **Features**: Command-line argument parsing, application initialization
- **Commands**: GUI, Server, Collect, Train, Test, Init

#### `Cargo.toml`
- **Purpose**: Project configuration and dependency management
- **Dependencies**: Hardware (rppal, spidev), ML (candle), GUI (egui), API (axum)
- **Build**: Optimized release configuration

### Database Module (`src/database/`)

#### `src/database/mod.rs`
- **Purpose**: Database operations and data models
- **Features**: SQLite integration, sensor reading storage, training session management
- **Models**: SensorReading, TrainingSession, ModelPerformance

### Hardware Module (`src/hardware/`)

#### `src/hardware/mod.rs`
- **Purpose**: Hardware module exports
- **Exports**: sensors, collector modules

#### `src/hardware/sensors.rs`
- **Purpose**: Low-level sensor communication
- **Features**: SPI communication with MCP3008, sensor calibration, data validation
- **Sensors**: 4 TGS sensors, 4 MQ sensors

#### `src/hardware/collector.rs`
- **Purpose**: High-level data collection management
- **Features**: Training data collection, continuous monitoring, data export
- **Statistics**: Sensor statistics, data validation

### Machine Learning Module (`src/ml/`)

#### `src/ml/mod.rs`
- **Purpose**: ML module exports
- **Exports**: cnn, preprocessing, evaluation modules

#### `src/ml/cnn.rs`
- **Purpose**: 1D-CNN model implementation
- **Features**: Deep learning model, training pipeline, prediction
- **Architecture**: 3-layer CNN with batch normalization and dropout

#### `src/ml/preprocessing.rs`
- **Purpose**: Data preprocessing and augmentation
- **Features**: Normalization, data augmentation, feature extraction
- **Techniques**: Z-score normalization, noise injection, scaling

#### `src/ml/evaluation.rs`
- **Purpose**: Model evaluation and metrics
- **Features**: Confusion matrix, precision, recall, F1-score
- **Metrics**: Classification performance analysis

### GUI Module (`src/gui/`)

#### `src/gui/mod.rs`
- **Purpose**: GUI module exports
- **Exports**: app, plots, widgets modules

#### `src/gui/app.rs`
- **Purpose**: Main GUI application
- **Features**: Real-time sensor display, training interface, results visualization
- **Framework**: egui with real-time updates

#### `src/gui/plots.rs`
- **Purpose**: Data visualization and plotting
- **Features**: Performance metrics plots, sensor readings, confusion matrix
- **Plots**: Line plots, bar charts, heatmaps

#### `src/gui/widgets.rs`
- **Purpose**: Custom GUI widgets
- **Features**: Sensor display widgets, control panels, status indicators
- **Widgets**: Reusable UI components

### API Module (`src/api/`)

#### `src/api/mod.rs`
- **Purpose**: API module exports
- **Exports**: server, handlers modules

#### `src/api/server.rs`
- **Purpose**: REST API server implementation
- **Features**: HTTP server, CORS support, state management
- **Framework**: Axum web framework

#### `src/api/handlers.rs`
- **Purpose**: API request handlers
- **Features**: Sensor data endpoints, training control, model predictions
- **Endpoints**: RESTful API for remote control

### Configuration Files

#### `config/settings.toml`
- **Purpose**: Application configuration
- **Settings**: Hardware parameters, training config, database settings
- **Categories**: hardware, sensors, training, database, api

#### `config/schema.sql`
- **Purpose**: Database schema definition
- **Tables**: sensor_readings, training_sessions, model_performance
- **Indexes**: Optimized for query performance

### Build and Deployment

#### `build.sh`
- **Purpose**: Build script for the project
- **Features**: Cargo build, permission setup, configuration copy
- **Output**: Optimized release binary

#### `deploy.sh`
- **Purpose**: Deployment script for Raspberry Pi
- **Features**: System service setup, dependency installation
- **Service**: Systemd service configuration

### Testing

#### `tests/integration_tests.rs`
- **Purpose**: Integration testing
- **Tests**: Database operations, end-to-end workflows
- **Coverage**: Complete system testing

#### `tests/hardware_tests.rs`
- **Purpose**: Hardware testing
- **Tests**: Sensor communication, SPI interface, calibration
- **Validation**: Hardware functionality

#### `tests/ml_tests.rs`
- **Purpose**: ML model testing
- **Tests**: Model training, prediction accuracy, data preprocessing
- **Validation**: ML pipeline correctness

### Examples

#### `examples/basic_usage.rs`
- **Purpose**: Basic usage demonstration
- **Features**: Simple sensor reading, basic operations
- **Usage**: Getting started guide

#### `examples/data_collection.rs`
- **Purpose**: Data collection examples
- **Features**: Training data collection, session management
- **Usage**: Data gathering workflows

#### `examples/model_training.rs`
- **Purpose**: Model training examples
- **Features**: Training pipeline, model evaluation
- **Usage**: ML workflow demonstration

## Key Features Implemented

### ✅ Hardware Integration
- **SPI Communication**: Complete MCP3008 interface
- **Sensor Support**: 8 gas sensors (4 TGS + 4 MQ)
- **Calibration**: Automatic sensor calibration
- **Data Validation**: Real-time data quality checks

### ✅ Machine Learning
- **1D-CNN Model**: Deep learning for gas classification
- **Data Preprocessing**: Normalization and augmentation
- **Training Pipeline**: Complete training workflow
- **Model Evaluation**: Comprehensive metrics

### ✅ Data Management
- **SQLite Database**: Persistent data storage
- **Session Management**: Training session tracking
- **Data Export**: CSV export functionality
- **Statistics**: Real-time data statistics

### ✅ User Interface
- **Real-time GUI**: Live sensor monitoring
- **Training Interface**: Interactive training control
- **Visualization**: Performance metrics and plots
- **Control Panel**: System configuration

### ✅ API Server
- **REST API**: Remote system control
- **Real-time Data**: Live sensor data streaming
- **Training Control**: Remote training management
- **Model Prediction**: API-based predictions

### ✅ System Integration
- **CLI Interface**: Command-line operation
- **Service Deployment**: Systemd service
- **Configuration**: Flexible configuration system
- **Logging**: Comprehensive logging

## Usage Commands

### Basic Operations
```bash
# Build the project
./build.sh

# Initialize hardware
./electronic-nose init

# Start GUI
./electronic-nose gui

# Start API server
./electronic-nose server

# Collect training data
./electronic-nose collect --gas-type CO --concentration 100 --duration 60

# Train model
./electronic-nose train --epochs 100 --batch-size 32

# Test model
./electronic-nose test
```

### Development Commands
```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=info ./electronic-nose gui

# Build for release
cargo build --release

# Run specific example
cargo run --example basic_usage
```

## Performance Expectations

### Hardware Performance
- **Sampling Rate**: 10Hz (configurable)
- **Sensor Warm-up**: 2-3 minutes
- **Calibration Time**: 10 seconds
- **Response Time**: <100ms per reading

### Machine Learning Performance
- **Training Time**: 5-10 minutes (100 epochs)
- **Inference Time**: <10ms per prediction
- **Memory Usage**: <500MB RAM
- **Model Size**: <50MB

### System Performance
- **CPU Usage**: <20% during operation
- **Memory Usage**: <1GB total
- **Storage**: <100MB for models and data
- **Network**: <1MB/s for API operations

This project structure provides a complete, production-ready electronic nose system with deep learning capabilities, real-time GUI, and remote API access, all implemented in Rust for optimal performance and reliability.