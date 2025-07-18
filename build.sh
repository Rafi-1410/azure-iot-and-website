#!/bin/bash

# Electronic Nose Build Script
# This script builds the complete electronic nose system

set -e

echo "🔧 Building Electronic Nose System..."
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're on Raspberry Pi
if [ -f /proc/device-tree/model ] && grep -q "Raspberry Pi" /proc/device-tree/model; then
    print_status "Detected Raspberry Pi system"
    IS_RPI=true
else
    print_warning "Not running on Raspberry Pi - some features may not work"
    IS_RPI=false
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
print_status "Using Rust version: $RUST_VERSION"

# Create necessary directories
print_status "Creating project directories..."
mkdir -p data/{raw,processed}
mkdir -p logs
mkdir -p models
mkdir -p config

# Create default configuration if it doesn't exist
if [ ! -f config/settings.toml ]; then
    print_status "Creating default configuration..."
    cat > config/settings.toml << EOF
[hardware]
spi_bus = 0
spi_device = 0
spi_speed = 1000000
adc_channels = 8

[sensors]
tgs2600_channel = 0
tgs2602_channel = 1
tgs2610_channel = 2
tgs2611_channel = 3
mq2_channel = 4
mq3_channel = 5
mq7_channel = 6
mq135_channel = 7

[training]
learning_rate = 0.001
batch_size = 32
epochs = 100
validation_split = 0.2
early_stopping_patience = 10

[database]
path = "data/electronic_nose.db"
backup_interval = 3600

[api]
host = "0.0.0.0"
port = 3000
EOF
fi

# Check for required system packages on Raspberry Pi
if [ "$IS_RPI" = true ]; then
    print_status "Checking system dependencies..."
    
    # Check if SPI is enabled
    if [ ! -e /dev/spidev0.0 ]; then
        print_warning "SPI is not enabled. You may need to enable it:"
        echo "sudo raspi-config -> Interface Options -> SPI -> Enable"
    fi
    
    # Check for required packages
    REQUIRED_PACKAGES="build-essential pkg-config libssl-dev libsqlite3-dev"
    for package in $REQUIRED_PACKAGES; do
        if ! dpkg -l | grep -q "^ii  $package "; then
            print_warning "Missing package: $package"
            echo "Install with: sudo apt install $package"
        fi
    done
fi

# Build the project
print_status "Building project in release mode..."
export RUSTFLAGS="-C target-cpu=native"

if cargo build --release; then
    print_success "Build completed successfully!"
else
    print_error "Build failed!"
    exit 1
fi

# Set up permissions for GPIO access (Raspberry Pi only)
if [ "$IS_RPI" = true ]; then
    print_status "Setting up GPIO permissions..."
    
    # Add user to gpio and spi groups
    sudo usermod -a -G gpio,spi $USER || print_warning "Could not add user to gpio/spi groups"
    
    # Set permissions for SPI device
    if [ -e /dev/spidev0.0 ]; then
        sudo chmod 666 /dev/spidev0.0 || print_warning "Could not set SPI permissions"
    fi
fi

# Copy configuration files to target directory
print_status "Copying configuration files..."
cp -r config target/release/ 2>/dev/null || print_warning "Could not copy config files"

# Create desktop shortcut (if running on desktop environment)
if [ -n "$DISPLAY" ] && [ "$IS_RPI" = true ]; then
    print_status "Creating desktop shortcut..."
    cat > ~/Desktop/electronic-nose.desktop << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Electronic Nose
Comment=Electronic Nose Deep Learning System
Exec=$(pwd)/target/release/electronic-nose gui
Icon=applications-science
Terminal=false
Categories=Science;Education;
EOF
    chmod +x ~/Desktop/electronic-nose.desktop
fi

# Create quick start script
print_status "Creating quick start scripts..."
cat > start_gui.sh << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
RUST_LOG=info ./target/release/electronic-nose gui
EOF
chmod +x start_gui.sh

cat > start_server.sh << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
RUST_LOG=info ./target/release/electronic-nose server
EOF
chmod +x start_server.sh

# Create systemd service file
print_status "Creating systemd service file..."
cat > electronic-nose.service << EOF
[Unit]
Description=Electronic Nose API Server
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/electronic-nose server
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

# Check binary size
BINARY_SIZE=$(du -h target/release/electronic-nose | cut -f1)
print_status "Binary size: $BINARY_SIZE"

# Run basic tests if possible
if [ "$IS_RPI" = true ]; then
    print_status "Running basic functionality tests..."
    
    # Test if binary can run
    if ./target/release/electronic-nose --help > /dev/null 2>&1; then
        print_success "Binary execution test passed"
    else
        print_warning "Binary execution test failed"
    fi
else
    print_status "Skipping hardware tests (not on Raspberry Pi)"
fi

# Print build summary
echo ""
echo "🎉 Build Summary"
echo "================"
print_success "Electronic Nose system built successfully!"
echo ""
echo "📁 Project structure:"
echo "  • Binary: target/release/electronic-nose"
echo "  • Config: config/settings.toml"
echo "  • Data:   data/ (will be created on first run)"
echo "  • Models: models/ (will be created during training)"
echo "  • Logs:   logs/ (will be created on first run)"
echo ""
echo "🚀 Quick start commands:"
echo "  • GUI:           ./start_gui.sh"
echo "  • API Server:    ./start_server.sh"
echo "  • Initialize:    ./target/release/electronic-nose init"
echo "  • Collect data:  ./target/release/electronic-nose collect --gas-type CO --concentration 100 --duration 60"
echo "  • Train model:   ./target/release/electronic-nose train"
echo ""
echo "🔧 System service:"
echo "  • Install:       sudo cp electronic-nose.service /etc/systemd/system/"
echo "  • Enable:        sudo systemctl enable electronic-nose"
echo "  • Start:         sudo systemctl start electronic-nose"
echo ""

if [ "$IS_RPI" = true ]; then
    echo "📋 Next steps for Raspberry Pi:"
    echo "  1. Enable SPI if not already enabled: sudo raspi-config"
    echo "  2. Reboot to apply group changes: sudo reboot"
    echo "  3. Connect your sensors according to WIRING_TUTORIAL.md"
    echo "  4. Initialize hardware: ./target/release/electronic-nose init"
    echo "  5. Start collecting data!"
else
    echo "📋 Next steps for development:"
    echo "  1. Review the wiring tutorial: WIRING_TUTORIAL.md"
    echo "  2. Set up your Raspberry Pi hardware"
    echo "  3. Transfer this build to your Raspberry Pi"
    echo "  4. Run the deployment script on the Pi"
fi

echo ""
print_success "Build completed! 🎉"
echo ""
echo "For detailed wiring instructions, see: WIRING_TUTORIAL.md"
echo "For project structure, see: PROJECT_STRUCTURE.md"
echo ""
echo "Happy gas sensing! 🌬️"