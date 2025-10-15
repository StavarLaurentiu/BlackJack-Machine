# BlackJack Machine

A singleplayer BlackJack game powered by a Raspberry Pi Pico 2 (RP2350A) and implemented in Rust using the Embassy async framework.

## Description

This project implements a fully interactive BlackJack game machine built on a Raspberry Pi Pico 2. The system uses multiple OLED displays to show card information and game state, buttons for player interaction, and RGB LEDs for visual feedback.

### Features

- **Gameplay**: Complete BlackJack rules implementation with player and dealer turns
- **Interactive Controls**: Three buttons for game actions (HIT, STAND, START GAME)
- **Visual Display**:
  - 8 OLED 0.96" displays to show player and dealer cards
  - 1 dedicated OLED display for game state messages
- **Status Indicators**: 2 RGB LEDs showing turn status and round outcomes
- **Hardware Management**: TCA9548A I2C multiplexer to control multiple OLED displays with identical addresses

## Architecture

The Raspberry Pi Pico 2 (RP2350A) serves as the central controller, handling all game logic and peripheral communication:

- **Input**: Button GPIOs with pull-up resistors
- **Output**: RGB LEDs connected to PWM-capable pins
- **Display**: 9 OLED displays managed through an I2C multiplexer
- **Software**: Rust with Embassy async framework for concurrent operations

### System Block Diagram

```
                    ┌─────────────────┐
                    │                 │
                    │  START Button   │────┐
                    │                 │    │
                    └─────────────────┘    │ GPIO21
                                           │
                    ┌─────────────────┐    │
                    │                 │    │
                    │  HIT Button     │────┤
                    │                 │    │ GPIO20
                    └─────────────────┘    │
                                           │
                    ┌─────────────────┐    │
                    │                 │    │
                    │  STAND Button   │────┤ GPIO19
                    │                 │    │
                    └─────────────────┘    │
                            ┌──────────────┘
                    ┌───────┴─────────┐        ┌─────────────────┐
                    │                 │ GPIO6  │   Player RGB    │
                    │                 ├────────┤      LED        │
                    │                 │        │                 │
                    │                 │        └─────────────────┘
                    │                 │
                    │   Raspberry     │        ┌─────────────────┐
                    │   Pi Pico 2     │ GPIO7  │   Dealer RGB    │
                    │   (RP2350A)     ├────────┤      LED        │
                    │                 │        │                 │
                    │                 │        └─────────────────┘
                    │                 │
                    │                 │        ┌─────────────────┐
                    │                 │        │  Game State     │
                    │                 ├────────┤     OLED        │
                    │                 │I2C_1   │                 │
                    └────────┬────────┘        └─────────────────┘
                             │
                             │I2C_0
                             │
                    ┌────────┴────────┐
                    │                 │
                    │   TCA9548A      │
                    │   I2C MUX       │
                    │                 │
                    └┬───────────────┬┘
                     │               │
                     │               │
                     │               │
          ┌──────────┴─────┐ ┌───────┴──────┐
          │ Player Cards   │ │Dealer's Cards│
          │ OLED 1-4       │ │ OLED 5-8     │
          └────────────────┘ └──────────────┘
```

### Game Flow

1. **Start**: System initializes, displays welcome message and waits for START button press
2. **Deal**: System deals 2 cards each to player and dealer (one dealer card hidden)
3. **Player Turn**: Player can press HIT to get another card or STAND to end their turn
   - If player busts (score > 21), game ends
4. **Dealer Turn**: Dealer reveals hidden card and draws cards until reaching score ≥ 17
5. **Resolution**: System compares scores, displays winner, and waits 5 seconds before reset

## Hardware Components

| Component                            | Quantity | Description                                |
| ------------------------------------ | -------- | ------------------------------------------ |
| Raspberry Pi Pico 2 (RP2350A)        | 1        | Main microcontroller                       |
| OLED Display 0.96" (SSD1306, 128x64) | 9        | Card and game state displays               |
| TCA9548A I2C Multiplexer             | 1        | Multiplexes the I2C bus for multiple OLEDs |
| RGB LEDs                             | 2        | Visual indicators for game state           |
| Push Buttons                         | 3        | User input controls                        |
| Breadboard                           | 1        | For prototyping and connections            |
| Jumper Wires                         | Various  | For connections between components         |

## Software Architecture

This project is built using Rust with the Embassy async framework, providing a robust and memory-safe implementation with non-blocking I/O operations.

### Key Software Components

- **Card Representation**: Representation of the deck, cards, and hands
- **Game State Manager**: Controls game flow and rules enforcement
- **Input Handlers**: Asynchronous handling of button inputs
- **Display Manager**: Controls the OLED displays through I2C
- **LED Controller**: Manages RGB LED states to indicate game progress

## Getting Started

### Prerequisites

- Rust toolchain (version 1.85+)
- Rust ARMv8-M target: `rustup target add thumbv8m.main-none-eabihf`
- probe-rs for flashing: `cargo install probe-rs-tools`
- cargo-binutils: `cargo install cargo-binutils`
- Rust LLVM Tools: `rustup component add llvm-tools`
- Hardware components as listed in the Bill of Materials

### Building the Project

```bash
# Clone the repository
git clone https://github.com/StavarLaurentiu/BlackJack-Machine
cd proiect-StavarLaurentiu

# Build the project
cargo build

# Flash to the Raspberry Pi Pico 2
cargo run
```

### Hardware Setup

1. Connect the three buttons to GPIO pins with pull-up resistors
2. Connect the RGB LEDs to PWM-capable GPIO pins (with appropriate resistors)
3. Connect the TCA9548A I2C multiplexer to the I2C0 bus
4. Connect the game state OLED to I2C1 bus
5. Connect the 8 card display OLEDs to the TCA9548A multiplexer outputs

## Acknowledgments

- Rust Embedded Working Group for embedded-hal
- Embassy-rs team for the async embedded framework
- The Raspberry Pi Foundation for the Pico 2 platform
