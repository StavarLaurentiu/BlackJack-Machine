use embassy_rp::gpio::Input;
use embassy_time::{Duration, Timer};

/// Button handler for game controls
pub struct GameButton {
    input: Input<'static>,
}

impl GameButton {
    /// Create a new button with the specified GPIO pin
    pub fn new(input: Input<'static>) -> Self {
        Self { input }
    }

    /// Wait for button to be pressed
    pub async fn wait_for_press(&mut self) {
        self.input.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(50)).await; // Simple debounce
    }
}

/// Set of all game buttons
pub struct GameButtons {
    pub hit_button: GameButton,
    pub stand_button: GameButton,
    pub start_button: GameButton,
}

impl GameButtons {
    /// Create a new set of game buttons
    pub fn new(
        hit_input: Input<'static>,
        stand_input: Input<'static>,
        start_input: Input<'static>,
        _debounce_time: Duration,
    ) -> Self {
        Self {
            hit_button: GameButton::new(hit_input),
            stand_button: GameButton::new(stand_input),
            start_button: GameButton::new(start_input),
        }
    }
}
