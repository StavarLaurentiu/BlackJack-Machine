use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_9X15},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{Ssd1306, mode::BufferedGraphicsMode, prelude::*, size::DisplaySize128x64};

/// OLED display for game state messages
pub struct GameStateDisplay<DI>
where
    DI: WriteOnlyDataCommand,
{
    display: Ssd1306<DI, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
}

impl<DI> GameStateDisplay<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create a new game state display
    pub fn new(interface: DI) -> Option<Self> {
        // Create display with default configuration
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        // Initialize display - return None if failed
        if display.init().is_err() {
            return None;
        }

        // Clear with black (off pixels)
        let _ = display.clear(BinaryColor::Off);

        // Flush display - return None if failed
        if display.flush().is_err() {
            return None;
        }

        Some(Self { display })
    }

    /// Display a welcome message on startup
    pub fn show_welcome(&mut self) -> bool {
        // Clear with black (off pixels)
        let _ = self.display.clear(BinaryColor::Off);

        let text_style = MonoTextStyle::new(&FONT_9X15, BinaryColor::On);

        // Game title
        let _ = Text::new("BlackJack", Point::new(20, 16), text_style).draw(&mut self.display);

        // Instructions
        let _ = Text::new("Press START...", Point::new(4, 40), text_style).draw(&mut self.display);

        // Return true if flush succeeded, false otherwise
        self.display.flush().is_ok()
    }

    /// Display the current game state message
    pub fn show_game_state(
        &mut self,
        message: &str,
        player_score: Option<u8>,
        dealer_score: Option<u8>,
    ) -> bool {
        // Clear with black (off pixels)
        let _ = self.display.clear(BinaryColor::Off);

        let text_style = MonoTextStyle::new(&FONT_9X15, BinaryColor::On);

        // Split message by \n and draw each line
        let lines: heapless::Vec<&str, 4> = message.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            let y_position = 12 + i as i32 * 16;
            let _ = Text::new(line, Point::new(0, y_position), text_style).draw(&mut self.display);
        }

        // Draw player score if available
        if let Some(score) = player_score {
            // Draw static text
            let _ = Text::new("Player: ", Point::new(0, 35), text_style).draw(&mut self.display);

            // Convert score to string manually
            let score_str = score_to_str(score);

            let _ = Text::new(
                score_str,
                Point::new(60, 35), // Offset to position after "Player: "
                text_style,
            )
            .draw(&mut self.display);
        }

        // Draw dealer score if available
        if let Some(score) = dealer_score {
            let _ = Text::new("Dealer: ", Point::new(0, 55), text_style).draw(&mut self.display);

            // Convert score to string manually
            let score_str = score_to_str(score);

            let _ = Text::new(
                score_str,
                Point::new(60, 55), // Offset to position after "Dealer: "
                text_style,
            )
            .draw(&mut self.display);
        }

        // Return true if flush succeeded, false otherwise
        self.display.flush().is_ok()
    }
}

// Helper function to convert score to &'static str
fn score_to_str(score: u8) -> &'static str {
    match score {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        11 => "11",
        12 => "12",
        13 => "13",
        14 => "14",
        15 => "15",
        16 => "16",
        17 => "17",
        18 => "18",
        19 => "19",
        20 => "20",
        21 => "21",
        22 => "22",
        23 => "23",
        24 => "24",
        25 => "25",
        26 => "26",
        27 => "27",
        28 => "28",
        29 => "29",
        30 => "30",
        31 => "31",
        _ => "32+", // For extremely high values (rare in BlackJack)
    }
}
