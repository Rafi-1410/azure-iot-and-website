# Electronic Nose with Deep Learning 🌬️🧠

A complete electronic nose system implementation using Rust, featuring deep learning-based gas classification with real-time GUI and API interface.

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Raspberry%20Pi-red.svg)](https://www.raspberrypi.org/)

## 🎯 Features

- **🔬 8 Gas Sensors**: 4 TGS + 4 MQ sensors for comprehensive gas detection
- **🧠 Deep Learning**: 1D-CNN model for accurate gas classification
- **📊 Real-time GUI**: Interactive interface with live sensor monitoring
- **🌐 REST API**: Remote control and monitoring capabilities
- **📈 Performance Metrics**: Precision, accuracy, recall, and F1-score visualization
- **💾 Data Management**: SQLite database with training session management
- **🔄 Auto Calibration**: Automatic sensor calibration and drift compensation
- **⚡ High Performance**: Optimized Rust implementation for embedded systems

## 🛠️ Hardware Requirements

### Main Components
- **Raspberry Pi Compute Module 5** (8GB RAM recommended)
- **MCP3008 ADC Converter** (10-bit, 8-channel SPI)
- **Power Supply**: 5V 3A minimum
- **Breadboard/PCB** for connections

### Gas Sensors
| Sensor | Type | Target Gases | Sensitivity Range |
|--------|------|--------------|-------------------|
| TGS2600 | TGS | H2, CO, CH4 | 1-10 ppm |
| TGS2602 | TGS | NH3, H2S, C2H5OH | 1-30 ppm |
| TGS2610 | TGS | Butane, Propane, CH4 | 1-10 ppm |
| TGS2611 | TGS | CH4, Propane | 1-10 ppm |
| MQ-2 | MQ | LPG, Propane, H2 | 200-10000 ppm |
| MQ-3 | MQ | C2H5OH, Benzine | 0.05-10 mg/L |
| MQ-7 | MQ | CO | 20-2000 ppm |
| MQ-135 | MQ | NH3, NOx, CO2 | 10-1000 ppm |

## 📋 Software Requirements

- **Rust** 1.70+ (stable)
- **Raspberry Pi OS** (64-bit recommended)
- **SPI Interface** enabled
- **SQLite3** development libraries

## 🚀 Quick Start

### 1. Clone and Build
```bash
git clone https://github.com/yourusername/electronic-nose-rust.git
cd electronic-nose-rust
chmod +x build.sh
./build.sh
```

### 2. Hardware Setup
Follow the detailed [Wiring Tutorial](WIRING_TUTORIAL.md) to connect your sensors.

### 3. Initialize System
```bash
./target/release/electronic-nose init
```

### 4. Start GUI
```bash
./start_gui.sh
```

### 5. Collect Training Data
```bash
./target/release/electronic-nose collect --gas-type CO --concentration 100 --duration 60
```

### 6. Train Model
```bash
./target/release/electronic-nose train --epochs 100 --batch-size 32
```

## 💻 Usage

### Command Line Interface
```bash
# Initialize hardware and calibrate sensors
./electronic-nose init

# Start GUI application
./electronic-nose gui

# Start API server
./electronic-nose server --port 3000

# Collect training data
./electronic-nose collect --gas-type CO --concentration 100 --duration 60

# Train the model
./electronic-nose train --epochs 100 --batch-size 32

# Test the model
./electronic-nose test
```

### GUI Application
The GUI provides:
- **Real-time sensor monitoring**
- **Training data collection interface**
- **Model training progress**
- **Performance metrics visualization**
- **Confusion matrix display**
- **Historical data analysis**

### REST API
The API server provides endpoints for:
- `GET /sensors/current` - Current sensor readings
- `POST /training/start` - Start training session
- `POST /model/predict` - Make predictions
- `GET /model/metrics` - Get model performance

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Hardware      │    │   Data Layer    │    │   ML Pipeline   │
│                 │    │                 │    │                 │
│ • 8 Gas Sensors │───▶│ • SQLite DB     │───▶│ • 1D-CNN Model  │
│ • MCP3008 ADC   │    │ • Data Storage  │    │ • Training      │
│ • SPI Interface │    │ • Session Mgmt  │    │ • Evaluation    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   GUI Layer     │    │   API Layer     │    │   System Layer  │
│                 │    │                 │    │                 │
│ • Real-time UI  │    │ • REST API      │    │ • CLI Interface │
│ • Visualization │    │ • Remote Access │    │ • Service Mgmt  │
│ • Control Panel │    │ • Data Streaming│    │ • Configuration │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 📊 Performance

### Expected Results
- **Accuracy**: 90-95% on gas classification
- **Inference Time**: <10ms per prediction
- **Sampling Rate**: 10Hz (configurable)
- **Memory Usage**: <500MB RAM
- **Model Size**: <50MB

### Benchmarks
| Metric | Value | Notes |
|--------|-------|-------|
| Training Time | 5-10 min | 100 epochs, 6000 samples |
| Inference Time | <10ms | Single prediction |
| Memory Usage | <500MB | During training |
| Storage | <100MB | Models + data |
| CPU Usage | <20% | During operation |

## 🔧 Configuration

Edit `config/settings.toml` to customize:

```toml
[hardware]
spi_bus = 0
spi_speed = 1000000
adc_channels = 8

[training]
learning_rate = 0.001
batch_size = 32
epochs = 100
validation_split = 0.2

[api]
host = "0.0.0.0"
port = 3000
```

## 📚 Documentation

- **[Wiring Tutorial](WIRING_TUTORIAL.md)** - Complete hardware setup guide
- **[Project Structure](PROJECT_STRUCTURE.md)** - Detailed code organization
- **[API Documentation](docs/api.md)** - REST API reference
- **[Training Guide](docs/training.md)** - Model training best practices

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run hardware tests (Raspberry Pi only)
cargo test --test hardware_tests

# Run ML tests
cargo test --test ml_tests

# Run integration tests
cargo test --test integration_tests
```

## 🚀 Deployment

### Systemd Service
```bash
# Install service
sudo cp electronic-nose.service /etc/systemd/system/
sudo systemctl enable electronic-nose
sudo systemctl start electronic-nose

# Check status
sudo systemctl status electronic-nose
```

### Docker (Optional)
```bash
# Build Docker image
docker build -t electronic-nose .

# Run container
docker run -p 3000:3000 --device=/dev/spidev0.0 electronic-nose
```

## 🔬 Deep Learning Model

### Architecture
- **Input Layer**: 8 sensors × 100 time steps
- **Conv1D Layer 1**: 32 filters, kernel size 3
- **Conv1D Layer 2**: 64 filters, kernel size 3
- **Conv1D Layer 3**: 128 filters, kernel size 3
- **Global Average Pooling**
- **Dense Layer**: 64 units + Dropout
- **Output Layer**: 6 classes (gas types)

### Training Features
- **Data Augmentation**: Noise injection, scaling, time shifting
- **Regularization**: Dropout, batch normalization, early stopping
- **Optimization**: Adam optimizer with learning rate scheduling
- **Validation**: K-fold cross-validation

## 🛡️ Safety & Calibration

### Safety Considerations
- **Ventilation**: Ensure proper ventilation during testing
- **Gas Handling**: Use only safe, non-toxic test gases
- **Electrical Safety**: Follow proper electrical safety procedures
- **Sensor Temperature**: Sensors get hot during operation

### Calibration
- **Automatic Calibration**: Run every startup
- **Baseline Correction**: Clean air environment required
- **Drift Compensation**: Periodic recalibration recommended
- **Validation**: Built-in sensor validation checks

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Figaro Engineering** for TGS sensor documentation
- **Hanwei Electronics** for MQ sensor specifications
- **Raspberry Pi Foundation** for excellent hardware platform
- **Rust Community** for amazing libraries and tools
- **Research Papers** on electronic nose systems and deep learning

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/electronic-nose-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/electronic-nose-rust/discussions)
- **Documentation**: [Wiki](https://github.com/yourusername/electronic-nose-rust/wiki)

## 🔮 Future Enhancements

- [ ] **Wireless Sensors**: Support for wireless sensor nodes
- [ ] **Cloud Integration**: Data upload to cloud services
- [ ] **Mobile App**: Companion mobile application
- [ ] **Advanced ML**: Transformer models and ensemble methods
- [ ] **Multi-Language**: Python and JavaScript bindings
- [ ] **Real-time Streaming**: WebSocket support for real-time data
- [ ] **Edge AI**: TensorFlow Lite integration
- [ ] **Sensor Fusion**: Multi-modal sensor integration

---

**Made with ❤️ and Rust** 🦀

*For detailed technical information, see the [Project Structure](PROJECT_STRUCTURE.md) and [Wiring Tutorial](WIRING_TUTORIAL.md).*