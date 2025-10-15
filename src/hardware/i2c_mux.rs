use defmt::info;
use embedded_hal_1::i2c::I2c;

/// TCA9548A I2C Multiplexer
pub struct TCA9548A<I> {
    i2c: I,
    address: u8,
    current_channel: u8,
}

impl<I, E> TCA9548A<I>
where
    I: I2c<Error = E>,
{
    /// Create a new TCA9548A I2C multiplexer
    ///
    /// The default address is 0x70 (when A0, A1, A2 are tied to GND)
    pub fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            current_channel: 0,
        }
    }

    /// Select a specific channel (0-7) on the multiplexer
    pub fn select_channel(&mut self, channel: u8) -> Result<(), E> {
        if channel > 7 {
            // Log warning but don't fail - will handle this downstream
            info!("Warning: TCA9548A channel must be 0-7, got {}", channel);
            return Ok(());
        }

        // The channel is selected by writing a byte where the bit position
        // corresponds to the channel number
        let channel_value = 1 << channel;

        // Write the channel value to the TCA9548A
        let result = self.i2c.write(self.address, &[channel_value]);

        if result.is_ok() {
            self.current_channel = channel;
            info!("Selected I2C MUX channel {}", channel);
        } else {
            info!("Failed to select I2C MUX channel {}", channel);
        }

        result
    }

    /// Get mutable reference to the I2C bus
    pub fn i2c_mut(&mut self) -> &mut I {
        &mut self.i2c
    }
}
