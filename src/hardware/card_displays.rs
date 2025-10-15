use super::i2c_mux::TCA9548A;
use super::pbm_image::PBMImage;
use crate::game::Card;
use defmt::{Format, info};
use embedded_hal_1::i2c::I2c;

/// OLED display positions
#[derive(Debug, Clone, Copy, Format)]
pub enum DisplayPosition {
    DealerCard1 = 0,
    DealerCard2 = 1,
    DealerCard3 = 2,
    DealerCard4 = 3,
    PlayerCard1 = 4,
    PlayerCard2 = 5,
    PlayerCard3 = 6,
    PlayerCard4 = 7,
}

impl DisplayPosition {
    /// Convert to MUX channel number
    pub fn channel(&self) -> u8 {
        *self as u8
    }
}

/// Card display manager that handles the I2C MUX and OLED displays for cards
pub struct CardDisplays<I, E>
where
    I: I2c<Error = E>,
{
    mux: TCA9548A<I>,
    display_address: u8,
}

impl<I, E> CardDisplays<I, E>
where
    I: I2c<Error = E>,
{
    /// Create a new card display manager
    pub fn new(i2c: I, mux_address: u8, display_address: u8) -> Self {
        let mux = TCA9548A::new(i2c, mux_address);

        Self {
            mux,
            display_address,
        }
    }

    /// Initialize all displays
    pub fn init_all_displays(&mut self) -> Result<(), E> {
        // Initialize each display
        for pos in [
            DisplayPosition::DealerCard1,
            DisplayPosition::DealerCard2,
            DisplayPosition::DealerCard3,
            DisplayPosition::DealerCard4,
            DisplayPosition::PlayerCard1,
            DisplayPosition::PlayerCard2,
            DisplayPosition::PlayerCard3,
            DisplayPosition::PlayerCard4,
        ]
        .iter()
        {
            if let Err(e) = self.init_display(*pos) {
                info!("Failed to initialize display at position {:?}", *pos);
                return Err(e);
            }
        }

        // Return to channel 0
        self.mux.select_channel(0)?;

        Ok(())
    }

    /// Initialize a specific display by sending raw I2C commands
    fn init_display(&mut self, position: DisplayPosition) -> Result<(), E> {
        info!("Initializing display at position {:?}", position);

        // Select the channel for this display
        self.mux.select_channel(position.channel())?;

        // Get a mutable reference to the I2C bus
        let i2c = self.mux.i2c_mut();

        // SSD1306 initialization sequence (for 128x64 display)
        let init_commands = [
            0xAE, // Display off
            0xD5, 0x80, // Set display clock div
            0xA8, 0x3F, // Set multiplex
            0xD3, 0x00, // Set display offset
            0x40, // Set start line to 0
            0x8D, 0x14, // Charge pump on
            0x20, 0x00, // Memory mode - horizontal addressing
            0xA1, // Segment remap
            0xC8, // COM scan direction
            0xDA, 0x12, // Set COM pins hardware config
            0x81, 0xCF, // Set contrast
            0xD9, 0xF1, // Set precharge period
            0xDB, 0x40, // Set vcom detect
            0xA4, // Resume display
            0xA6, // Normal display (not inverted)
            0xAF, // Display on
        ];

        // Send all initialization commands
        for cmd in init_commands.iter() {
            // Command prefix 0x00
            let buf = [0x00, *cmd];
            i2c.write(self.display_address, &buf)?;
        }

        // Clear the display (all pixels off)
        self.clear_display_raw(position)?;

        info!("Display at position {:?} initialized", position);
        Ok(())
    }

    /// Clear a display by writing zeros to all pixels
    fn clear_display_raw(&mut self, position: DisplayPosition) -> Result<(), E> {
        // Make sure the right channel is selected
        self.mux.select_channel(position.channel())?;

        let i2c = self.mux.i2c_mut();

        // Set column address range (0-127)
        i2c.write(self.display_address, &[0x00, 0x21, 0x00, 0x7F])?;

        // Set page address range (0-7)
        i2c.write(self.display_address, &[0x00, 0x22, 0x00, 0x07])?;

        // Write zeros to clear the entire display
        // Each page is 128 bytes (1 page = 8 pixel rows × 128 columns)
        for _page in 0..8 {
            for _col in 0..16 {
                // Send in chunks of 16 bytes
                let chunk = [0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // 1 data prefix + 16 zeros
                i2c.write(self.display_address, &chunk)?;
            }
        }

        Ok(())
    }

    /// Display a PBM image on a specific OLED
    pub fn display_pbm_image(
        &mut self,
        image_data: &'static [u8],
        position: DisplayPosition,
    ) -> Result<(), E> {
        info!("Displaying PBM image at position {:?}", position);

        // Parse the PBM image
        let image = match PBMImage::new(image_data) {
            Ok(img) => img,
            Err(_) => {
                info!("Failed to parse PBM image");
                return Ok(()); // Don't fail the whole operation
            }
        };

        info!("PBM image parsed: {}x{}", image.width(), image.height());

        // Convert image to display buffer
        let buffer = match image.to_display_buffer(128, 64) {
            Ok(buf) => buf,
            Err(_) => {
                info!("Failed to convert PBM image to display buffer");
                return Ok(());
            }
        };

        // Select the channel for this display
        self.mux.select_channel(position.channel())?;

        let i2c = self.mux.i2c_mut();

        // Set column address range (0-127)
        i2c.write(self.display_address, &[0x00, 0x21, 0x00, 0x7F])?;

        // Set page address range (0-7)
        i2c.write(self.display_address, &[0x00, 0x22, 0x00, 0x07])?;

        // Send image data in chunks
        let mut pos = 0;
        while pos < buffer.len() {
            let chunk_size = core::cmp::min(16, buffer.len() - pos);
            let mut chunk = [0x40; 17]; // Data prefix + up to 16 bytes

            for i in 0..chunk_size {
                chunk[i + 1] = buffer[pos + i];
            }

            i2c.write(self.display_address, &chunk[..(chunk_size + 1)])?;
            pos += chunk_size;
        }

        info!("PBM image displayed at position {:?}", position);
        Ok(())
    }

    /// Display a card on a specific OLED using raw I2C commands
    pub fn display_card(&mut self, card: &Card, position: DisplayPosition) -> Result<(), E> {
        info!("Displaying card at position {:?}", position);

        // Select the channel for this display
        self.mux.select_channel(position.channel())?;

        // Clear display first
        self.clear_display_raw(position)?;

        if !card.is_face_up {
            // Draw card back pattern
            self.draw_card_back_raw()?;
        } else {
            // Draw card face
            self.draw_card_face_raw()?;
        }

        info!("Card displayed at position {:?}", position);
        Ok(())
    }

    /// Draw a card back pattern using raw I2C commands
    fn draw_card_back_raw(&mut self) -> Result<(), E> {
        let i2c = self.mux.i2c_mut();

        // Draw a box outline
        // Set column and page for top edge
        i2c.write(self.display_address, &[0x00, 0x21, 20, 108])?; // Col 20-108
        i2c.write(self.display_address, &[0x00, 0x22, 1, 1])?; // Page 1

        // Draw top edge (filled row)
        let mut top_line = [0x40; 90]; // Data prefix + 89 bytes
        top_line[1..].fill(0xFF); // Fill with 1s for solid line
        i2c.write(self.display_address, &top_line)?;

        Ok(())
    }

    /// Draw a card face using raw I2C commands
    fn draw_card_face_raw(&mut self) -> Result<(), E> {
        let i2c = self.mux.i2c_mut();

        // Draw a box outline
        // Set column and page for top edge
        i2c.write(self.display_address, &[0x00, 0x21, 20, 108])?; // Col 20-108
        i2c.write(self.display_address, &[0x00, 0x22, 1, 1])?; // Page 1

        // Draw top edge (border only)
        let mut top_line = [0x40; 90]; // Data prefix + 89 bytes
        top_line[1] = 0xFF; // Left border
        top_line[89] = 0xFF; // Right border
        i2c.write(self.display_address, &top_line)?;

        Ok(())
    }

    /// Clear a specific display
    pub fn clear_display(&mut self, position: DisplayPosition) -> Result<(), E> {
        info!("Clearing display at position {:?}", position);
        self.clear_display_raw(position)
    }

    /// Clear all displays
    pub fn clear_all_displays(&mut self) -> Result<(), E> {
        for pos in [
            DisplayPosition::DealerCard1,
            DisplayPosition::DealerCard2,
            DisplayPosition::DealerCard3,
            DisplayPosition::DealerCard4,
            DisplayPosition::PlayerCard1,
            DisplayPosition::PlayerCard2,
            DisplayPosition::PlayerCard3,
            DisplayPosition::PlayerCard4,
        ]
        .iter()
        {
            if let Err(e) = self.clear_display(*pos) {
                return Err(e);
            }
        }

        Ok(())
    }
}
