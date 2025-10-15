use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};

/// RGB LED colors
#[derive(Debug, Clone, Copy)]
pub enum LedColor {
    Red,
    Green,
    Blue,
    Yellow, // Red + Green
    Off,    // All off
}

/// RGB LED controller
pub struct RgbLed {
    red: Output<'static>,
    green: Output<'static>,
    blue: Output<'static>,
    current_color: LedColor,
}

impl RgbLed {
    /// Create a new RGB LED with the specified GPIO pins
    pub fn new(red: Output<'static>, green: Output<'static>, blue: Output<'static>) -> Self {
        let mut led = Self {
            red,
            green,
            blue,
            current_color: LedColor::Off,
        };
        led.set_color(LedColor::Off);
        led
    }

    /// Set the LED to a specific color
    pub fn set_color(&mut self, color: LedColor) {
        // Set all pins low (LEDs off) first
        self.red.set_low();
        self.green.set_low();
        self.blue.set_low();

        // Then set the appropriate pins high based on the color
        match color {
            LedColor::Red => self.red.set_high(),
            LedColor::Green => self.green.set_high(),
            LedColor::Blue => self.blue.set_high(),
            LedColor::Yellow => {
                self.red.set_high();
                self.green.set_high();
            }
            LedColor::Off => {
                // Already all off
            }
        }

        self.current_color = color;
    }

    /// Get the current color of the LED
    pub fn current_color(&self) -> LedColor {
        self.current_color
    }
}

/// Set of game LEDs
pub struct GameLeds {
    pub player_led: RgbLed,
    pub dealer_led: RgbLed,
}

impl GameLeds {
    /// Create a new set of game LEDs
    pub fn new(
        player_red: Output<'static>,
        player_green: Output<'static>,
        player_blue: Output<'static>,
        dealer_red: Output<'static>,
        dealer_green: Output<'static>,
        dealer_blue: Output<'static>,
    ) -> Self {
        Self {
            player_led: RgbLed::new(player_red, player_green, player_blue),
            dealer_led: RgbLed::new(dealer_red, dealer_green, dealer_blue),
        }
    }

    /// Blink both LEDs simultaneously
    pub async fn blink_both(&mut self, color: LedColor, blink_time: Duration, blinks: u8) {
        let player_original = self.player_led.current_color();
        let dealer_original = self.dealer_led.current_color();

        for _ in 0..blinks {
            // Turn both on
            self.player_led.set_color(color);
            self.dealer_led.set_color(color);
            Timer::after(blink_time).await;

            // Turn both off
            self.player_led.set_color(LedColor::Off);
            self.dealer_led.set_color(LedColor::Off);
            Timer::after(blink_time).await;
        }

        // Restore original colors
        self.player_led.set_color(player_original);
        self.dealer_led.set_color(dealer_original);
    }
}
