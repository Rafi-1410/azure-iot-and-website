# Electronic Nose Wiring Tutorial

## Complete Hardware Setup Guide

This tutorial provides detailed wiring instructions for connecting 8 gas sensors (4 TGS + 4 MQ) to a Raspberry Pi Compute Module 5 using an MCP3008 ADC converter.

---

## 📋 Components List

### Main Components
- **Raspberry Pi Compute Module 5** (8GB RAM recommended)
- **MCP3008 ADC Converter** (10-bit, 8-channel SPI ADC)
- **Breadboard or PCB** (830-point breadboard recommended)
- **Jumper Wires** (Male-to-Male, Male-to-Female)

### Gas Sensors (8 total)
#### TGS Sensors (4 pieces):
- **TGS2600** - Air Quality, Hydrogen, Carbon Monoxide
- **TGS2602** - Ammonia, Hydrogen Sulfide, Ethanol
- **TGS2610** - Butane, Propane, Methane
- **TGS2611** - Methane, Propane (Natural Gas)

#### MQ Sensors (4 pieces):
- **MQ-2** - LPG, Propane, Hydrogen, Smoke
- **MQ-3** - Ethanol, Alcohol, Benzine
- **MQ-7** - Carbon Monoxide
- **MQ-135** - Ammonia, Nitrogen Oxides, CO2, Smoke

### Electronic Components
- **Resistors**: 8x 10kΩ (pull-up resistors)
- **Capacitors**: 8x 100nF ceramic (noise filtering)
- **Power Supply**: 5V 3A (for sensors), 5V 2A (for Pi)
- **Voltage Regulator**: 3.3V regulator (if needed)

---

## 🔌 Raspberry Pi CM5 GPIO Pinout

```
Raspberry Pi Compute Module 5 GPIO Pinout:
┌─────────────────────────────────────────────────────────────┐
│                    Raspberry Pi CM5                        │
│                                                             │
│  3V3  (1) ● ● (2)  5V     │  3V3  (17) ● ● (18) GPIO24   │
│  GPIO2(3) ● ● (4)  5V     │  GPIO10(19) ● ● (20) GND     │
│  GPIO3(5) ● ● (6)  GND    │  GPIO9 (21) ● ● (22) GPIO25  │
│  GPIO4(7) ● ● (8)  GPIO14 │  GPIO11(23) ● ● (24) GPIO8   │
│  GND  (9) ● ● (10) GPIO15 │  GND   (25) ● ● (26) GPIO7   │
│  GPIO17  ● ● (12) GPIO18  │  GPIO0 (27) ● ● (28) GPIO1   │
│  GPIO27  ● ● (14) GND     │  GPIO5 (29) ● ● (30) GND     │
│  GPIO22  ● ● (16) GPIO23  │  GPIO6 (31) ● ● (32) GPIO12  │
└─────────────────────────────────────────────────────────────┘

SPI Pins Used:
- GPIO8  (Pin 24) - SPI0_CE0_N (Chip Select)
- GPIO9  (Pin 21) - SPI0_MISO  (Master In Slave Out)
- GPIO10 (Pin 19) - SPI0_MOSI  (Master Out Slave In)
- GPIO11 (Pin 23) - SPI0_SCLK  (Serial Clock)
```

---

## 🔧 MCP3008 ADC Wiring

### MCP3008 Pinout
```
MCP3008 ADC Converter Pinout:
┌─────────────────────────────────────┐
│ CH0  (1) ● ● (16) VDD (3.3V)        │
│ CH1  (2) ● ● (15) VREF (3.3V)       │
│ CH2  (3) ● ● (14) AGND (GND)        │
│ CH3  (4) ● ● (13) CLK (GPIO11)      │
│ CH4  (5) ● ● (12) DOUT (GPIO9)      │
│ CH5  (6) ● ● (11) DIN (GPIO10)      │
│ CH6  (7) ● ● (10) CS (GPIO8)        │
│ CH7  (8) ● ● (9)  DGND (GND)        │
└─────────────────────────────────────┘
```

### MCP3008 to Raspberry Pi Connections
| MCP3008 Pin | Function | Raspberry Pi Pin | GPIO |
|-------------|----------|------------------|------|
| 16 (VDD)    | Power    | Pin 1            | 3.3V |
| 15 (VREF)   | Reference| Pin 1            | 3.3V |
| 14 (AGND)   | Ground   | Pin 6            | GND  |
| 13 (CLK)    | Clock    | Pin 23           | GPIO11 |
| 12 (DOUT)   | Data Out | Pin 21           | GPIO9  |
| 11 (DIN)    | Data In  | Pin 19           | GPIO10 |
| 10 (CS)     | Chip Sel | Pin 24           | GPIO8  |
| 9 (DGND)    | Ground   | Pin 6            | GND  |

---

## 🌡️ Sensor Wiring Details

### TGS Sensor Wiring (All 4 sensors)

Each TGS sensor has 4 pins:
- **Pin 1**: Heater +5V
- **Pin 2**: Heater GND
- **Pin 3**: Sensor Output (to ADC)
- **Pin 4**: Sensor VCC +5V

```
TGS Sensor Wiring Pattern:
┌─────────────────────────────────────┐
│         TGS Sensor                  │
│  (1) H+ ● ● (4) VCC                 │
│  (2) H- ● ● (3) OUT                 │
└─────────────────────────────────────┘

Connection for each TGS sensor:
Pin 1 (H+)  → 5V Power Supply
Pin 2 (H-)  → Ground
Pin 3 (OUT) → 10kΩ → 3.3V (Pull-up)
Pin 3 (OUT) → 100nF → GND (Filter)
Pin 3 (OUT) → MCP3008 Channel
Pin 4 (VCC) → 5V Power Supply
```

### MQ Sensor Wiring (All 4 sensors)

Each MQ sensor has 4 pins:
- **Pin 1**: Heater +5V
- **Pin 2**: Heater GND  
- **Pin 3**: Analog Output (to ADC)
- **Pin 4**: Digital Output (not used)

```
MQ Sensor Wiring Pattern:
┌─────────────────────────────────────┐
│          MQ Sensor                  │
│  (1) H+ ● ● (4) DOUT                │
│  (2) H- ● ● (3) AOUT                │
└─────────────────────────────────────┘

Connection for each MQ sensor:
Pin 1 (H+)   → 5V Power Supply
Pin 2 (H-)   → Ground
Pin 3 (AOUT) → 10kΩ → 3.3V (Pull-up)
Pin 3 (AOUT) → 100nF → GND (Filter)
Pin 3 (AOUT) → MCP3008 Channel
Pin 4 (DOUT) → Not Connected
```

---

## 🔗 Complete Wiring Connections

### Sensor to ADC Channel Mapping
| Sensor   | Type | ADC Channel | MCP3008 Pin |
|----------|------|-------------|-------------|
| TGS2600  | TGS  | CH0         | Pin 1       |
| TGS2602  | TGS  | CH1         | Pin 2       |
| TGS2610  | TGS  | CH2         | Pin 3       |
| TGS2611  | TGS  | CH3         | Pin 4       |
| MQ-2     | MQ   | CH4         | Pin 5       |
| MQ-3     | MQ   | CH5         | Pin 6       |
| MQ-7     | MQ   | CH6         | Pin 7       |
| MQ-135   | MQ   | CH7         | Pin 8       |

### Power Distribution
```
Power Supply Connections:
┌─────────────────────────────────────┐
│  5V Power Supply (3A minimum)      │
│  ┌─────────────────────────────────┐│
│  │ +5V Bus (Red Wire)              ││
│  │ ├─ All Sensor VCC pins          ││
│  │ ├─ All Sensor Heater + pins     ││
│  │ └─ Raspberry Pi 5V (Pin 2)      ││
│  │                                 ││
│  │ GND Bus (Black Wire)            ││
│  │ ├─ All Sensor GND pins          ││
│  │ ├─ All Sensor Heater - pins     ││
│  │ ├─ MCP3008 GND pins             ││
│  │ └─ Raspberry Pi GND (Pin 6)     ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
```

---

## 📐 Breadboard Layout

### Breadboard Wiring Diagram
```
Breadboard Layout (Top View):
┌─────────────────────────────────────────────────────────────┐
│  + Power Rail (5V)                                         │
│  - Ground Rail (GND)                                       │
│                                                             │
│  [TGS2600] [TGS2602] [TGS2610] [TGS2611]                  │
│     |         |         |         |                       │
│  [10kΩ]   [10kΩ]   [10kΩ]   [10kΩ]   ← Pull-up resistors │
│     |         |         |         |                       │
│  [100nF]  [100nF]  [100nF]  [100nF]  ← Filter capacitors  │
│     |         |         |         |                       │
│    CH0      CH1       CH2       CH3   ← MCP3008 channels   │
│                                                             │
│  [MQ-2]   [MQ-3]    [MQ-7]   [MQ-135]                     │
│     |         |         |         |                       │
│  [10kΩ]   [10kΩ]   [10kΩ]   [10kΩ]   ← Pull-up resistors │
│     |         |         |         |                       │
│  [100nF]  [100nF]  [100nF]  [100nF]  ← Filter capacitors  │
│     |         |         |         |                       │
│    CH4      CH5       CH6       CH7   ← MCP3008 channels   │
│                                                             │
│                    [MCP3008]                               │
│                        |                                   │
│                 [Raspberry Pi CM5]                         │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔧 Step-by-Step Assembly

### Step 1: Prepare the Breadboard
1. **Insert MCP3008** into the center of the breadboard
2. **Connect power rails**:
   - Red wire from 5V supply to positive rail
   - Black wire from GND to negative rail
3. **Connect 3.3V rail** for pull-up resistors

### Step 2: Wire the MCP3008
1. **VDD (Pin 16)** → 3.3V rail
2. **VREF (Pin 15)** → 3.3V rail  
3. **AGND (Pin 14)** → GND rail
4. **DGND (Pin 9)** → GND rail
5. **CLK (Pin 13)** → GPIO11 (Pi Pin 23)
6. **DOUT (Pin 12)** → GPIO9 (Pi Pin 21)
7. **DIN (Pin 11)** → GPIO10 (Pi Pin 19)
8. **CS (Pin 10)** → GPIO8 (Pi Pin 24)

### Step 3: Wire TGS Sensors
For each TGS sensor (TGS2600, TGS2602, TGS2610, TGS2611):

1. **Heater connections**:
   - Pin 1 (H+) → 5V rail
   - Pin 2 (H-) → GND rail
   - Pin 4 (VCC) → 5V rail

2. **Signal conditioning**:
   - Pin 3 (OUT) → 10kΩ resistor → 3.3V rail
   - Pin 3 (OUT) → 100nF capacitor → GND rail
   - Pin 3 (OUT) → MCP3008 channel (CH0-CH3)

### Step 4: Wire MQ Sensors
For each MQ sensor (MQ-2, MQ-3, MQ-7, MQ-135):

1. **Heater connections**:
   - Pin 1 (H+) → 5V rail
   - Pin 2 (H-) → GND rail

2. **Signal conditioning**:
   - Pin 3 (AOUT) → 10kΩ resistor → 3.3V rail
   - Pin 3 (AOUT) → 100nF capacitor → GND rail
   - Pin 3 (AOUT) → MCP3008 channel (CH4-CH7)

### Step 5: Power Connections
1. **Connect 5V supply** to breadboard power rail
2. **Connect GND** to breadboard ground rail
3. **Connect Raspberry Pi**:
   - Pi Pin 2 (5V) → 5V rail (optional, for Pi power)
   - Pi Pin 6 (GND) → GND rail

### Step 6: Final Checks
1. **Verify all connections** with multimeter
2. **Check power supply voltage** (should be 5V ±0.1V)
3. **Test continuity** on all signal paths
4. **Ensure no short circuits** between power and ground

---

## ⚡ Power Requirements

### Power Consumption
| Component | Voltage | Current | Power |
|-----------|---------|---------|-------|
| Raspberry Pi CM5 | 5V | 2A | 10W |
| MCP3008 | 3.3V | 1mA | 3.3mW |
| TGS2600 | 5V | 56mA | 280mW |
| TGS2602 | 5V | 56mA | 280mW |
| TGS2610 | 5V | 56mA | 280mW |
| TGS2611 | 5V | 56mA | 280mW |
| MQ-2 | 5V | 150mA | 750mW |
| MQ-3 | 5V | 150mA | 750mW |
| MQ-7 | 5V | 150mA | 750mW |
| MQ-135 | 5V | 150mA | 750mW |
| **Total** | **5V** | **~3A** | **~15W** |

### Power Supply Recommendations
- **Minimum**: 5V 3A (15W) switching power supply
- **Recommended**: 5V 4A (20W) for safety margin
- **Features**: Short circuit protection, overvoltage protection

---

## 🛠️ Tools Required

### Basic Tools
- **Soldering iron** (25-40W)
- **Solder** (60/40 rosin core)
- **Wire strippers**
- **Multimeter**
- **Breadboard jumper wires**
- **Small screwdriver set**

### Optional Tools
- **Oscilloscope** (for signal analysis)
- **Logic analyzer** (for SPI debugging)
- **Hot air station** (for SMD work)
- **Flux** (for better soldering)

---

## 🔍 Testing and Verification

### Initial Testing
1. **Power-on test**:
   ```bash
   # Check if SPI is enabled
   ls /dev/spidev*
   # Should show: /dev/spidev0.0
   ```

2. **Sensor reading test**:
   ```bash
   # Run hardware initialization
   ./electronic-nose init
   ```

3. **Individual sensor test**:
   ```bash
   # Test each sensor channel
   ./electronic-nose test-sensors
   ```

### Troubleshooting Common Issues

#### No SPI Device
```bash
# Enable SPI interface
sudo raspi-config
# Navigate to: Interface Options → SPI → Enable
sudo reboot
```

#### Sensor Readings Out of Range
- Check power supply voltage (should be 5V ±0.1V)
- Verify pull-up resistor connections
- Check for loose connections
- Ensure proper grounding

#### Unstable Readings
- Add more filtering capacitors
- Check for electromagnetic interference
- Verify sensor heating time (2-3 minutes)
- Use shielded cables for long connections

---

## 📊 Expected Sensor Readings

### Normal Operating Ranges (Clean Air)
| Sensor | Voltage Range | Typical Value |
|--------|---------------|---------------|
| TGS2600 | 0.5V - 2.0V | ~1.2V |
| TGS2602 | 0.3V - 1.5V | ~0.8V |
| TGS2610 | 0.4V - 1.8V | ~1.0V |
| TGS2611 | 0.4V - 1.8V | ~1.0V |
| MQ-2 | 0.2V - 3.0V | ~1.5V |
| MQ-3 | 0.1V - 2.5V | ~1.0V |
| MQ-7 | 0.2V - 3.0V | ~1.8V |
| MQ-135 | 0.3V - 2.8V | ~1.6V |

### Sensor Warm-up Time
- **TGS sensors**: 2-3 minutes
- **MQ sensors**: 3-5 minutes
- **Stabilization**: 10-15 minutes for accurate readings

---

## 🔧 Maintenance and Calibration

### Regular Maintenance
1. **Clean sensors** monthly with compressed air
2. **Check connections** for corrosion or looseness
3. **Verify power supply** voltage stability
4. **Update calibration** every 3-6 months

### Calibration Procedure
1. **Prepare clean air environment**
2. **Warm up sensors** for 15 minutes
3. **Run calibration**:
   ```bash
   ./electronic-nose init
   ```
4. **Verify baseline readings** are stable
5. **Save calibration data** to database

---

## 📚 Additional Resources

### Datasheets
- [TGS2600 Datasheet](https://www.figaro.co.jp/en/product/docs/tgs2600_product_information.pdf)
- [TGS2602 Datasheet](https://www.figaro.co.jp/en/product/docs/tgs2602_product_information.pdf)
- [MQ-2 Datasheet](https://www.pololu.com/file/0J309/MQ2.pdf)
- [MCP3008 Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/21295d.pdf)

### Raspberry Pi Resources
- [Raspberry Pi GPIO Pinout](https://pinout.xyz/)
- [SPI Interface Guide](https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md)

### Software Setup
- [Rust Installation Guide](https://rustup.rs/)
- [RPPAL Library Documentation](https://docs.rs/rppal/)

---

## ⚠️ Safety Considerations

### Electrical Safety
- **Always disconnect power** before making connections
- **Use proper fusing** on power supplies
- **Avoid short circuits** that could damage components
- **Use anti-static precautions** when handling ICs

### Gas Safety
- **Ensure proper ventilation** when testing with gases
- **Use only safe, non-toxic test gases** in low concentrations
- **Have safety equipment** (gas detector, ventilation) available
- **Follow local regulations** for gas handling

### Component Safety
- **Sensors get hot** during operation - avoid touching
- **Allow cool-down time** before handling
- **Use heat sinks** if continuous operation is required
- **Monitor power consumption** to prevent overheating

---

This completes the comprehensive wiring tutorial for the electronic nose system. Follow these instructions carefully to ensure proper operation and safety of your electronic nose system.