#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    i2c::{Config as I2cConfig, I2c},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Import our modules
mod game;
mod hardware;
mod images;
mod irqs;
mod ui;

use game::{BlackJackGame, GameResult, GameState};
use hardware::{CardDisplays, GameButtons, GameLeds, GameStateDisplay, LedColor};
use ui::CardUI;

// Constants for I2C addresses
const I2C_MUX_ADDRESS: u8 = 0x70; // Default address for TCA9548A
const OLED_ADDRESS: u8 = 0x3C; // Default address for SSD1306 OLED displays

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());

    // Initialize buttons
    let hit_button = Input::new(p.PIN_14, Pull::Up);
    let stand_button = Input::new(p.PIN_13, Pull::Up);
    let start_button = Input::new(p.PIN_15, Pull::Up);

    // Initialize RGB LEDs
    let player_red = Output::new(p.PIN_2, Level::Low);
    let player_green = Output::new(p.PIN_3, Level::Low);
    let player_blue = Output::new(p.PIN_4, Level::Low);

    let dealer_red = Output::new(p.PIN_10, Level::Low);
    let dealer_green = Output::new(p.PIN_11, Level::Low);
    let dealer_blue = Output::new(p.PIN_12, Level::Low);

    // Create button and LED controllers
    let mut buttons = GameButtons::new(
        hit_button,
        stand_button,
        start_button,
        Duration::from_millis(50),
    );

    let mut leds = GameLeds::new(
        player_red,
        player_green,
        player_blue,
        dealer_red,
        dealer_green,
        dealer_blue,
    );

    // Initialize I2C buses
    info!("Initializing I2C0 for card displays");
    let sda0 = p.PIN_16;
    let scl0 = p.PIN_17;
    let i2c0 = I2c::new_blocking(p.I2C0, scl0, sda0, I2cConfig::default());

    info!("Initializing I2C1 for game state display");
    let sda1 = p.PIN_6;
    let scl1 = p.PIN_7;
    let i2c1 = I2c::new_blocking(p.I2C1, scl1, sda1, I2cConfig::default());

    // Create display interfaces
    let interface = ssd1306::I2CDisplayInterface::new(i2c1);

    let mut game_state_display = match GameStateDisplay::new(interface) {
        Some(display) => display,
        None => {
            info!("Failed to initialize game state display");
            defmt::panic!("Could not initialize display");
        }
    };

    let card_displays = CardDisplays::new(i2c0, I2C_MUX_ADDRESS, OLED_ADDRESS);
    let mut card_ui = CardUI::new(card_displays);

    info!("Initializing card displays");
    match card_ui.init_all_displays() {
        Ok(_) => info!("Card displays initialized successfully"),
        Err(_) => info!("Failed to initialize card displays - will continue without them"),
    }

    // Create the BlackJack game
    let mut game = BlackJackGame::new();

    // Display welcome message
    if !game_state_display.show_welcome() {
        info!("Failed to show welcome message");
    }

    info!("BlackJack Machine initialized!");

    // System ready indication
    leds.player_led.set_color(LedColor::Green);
    leds.dealer_led.set_color(LedColor::Green);
    Timer::after(Duration::from_secs(2)).await;
    leds.player_led.set_color(LedColor::Off);
    leds.dealer_led.set_color(LedColor::Off);

    // Game loop
    loop {
        match game.state() {
            GameState::WaitingForStart => {
                info!("Waiting for START button press...");
                buttons.start_button.wait_for_press().await;
                info!("START button pressed!");

                if !game_state_display.show_game_state("Starting\ngame...", None, None) {
                    info!("Failed to update display");
                }

                let _ = card_ui.clear_all();
                leds.blink_both(LedColor::Yellow, Duration::from_millis(500), 4)
                    .await;
                game.start_game();
            }

            GameState::DealerDealing => {
                if !game_state_display.show_game_state("Dealer is\ndealing cards...", None, None) {
                    info!("Failed to update display");
                }

                leds.dealer_led.set_color(LedColor::Blue);
                Timer::after(Duration::from_secs(2)).await;
                leds.dealer_led.set_color(LedColor::Green);

                game.deal_initial_cards();

                let _ = card_ui.update_dealer_hand(game.dealer_hand());
                let _ = card_ui.update_player_hand(game.player_hand());

                if !game_state_display.show_game_state(
                    "Cards dealt!",
                    Some(game.player_value()),
                    Some(game.dealer_value()),
                ) {
                    info!("Failed to update display");
                }

                Timer::after(Duration::from_secs(3)).await;
                leds.dealer_led.set_color(LedColor::Off);
            }

            GameState::PlayerTurn => {
                if !game_state_display.show_game_state(
                    "HIT/STAND",
                    Some(game.player_value()),
                    Some(game.dealer_value()),
                ) {
                    info!("Failed to update display");
                }

                leds.player_led.set_color(LedColor::Blue);
                Timer::after(Duration::from_secs(2)).await;
                leds.player_led.set_color(LedColor::Green);

                info!("Waiting for player action...");

                let hit_future = buttons.hit_button.wait_for_press();
                let stand_future = buttons.stand_button.wait_for_press();

                match embassy_futures::select::select(hit_future, stand_future).await {
                    embassy_futures::select::Either::First(_) => {
                        info!("HIT button pressed!");
                        leds.player_led.set_color(LedColor::Blue);

                        if game.player_hit() {
                            let _ = card_ui.update_player_hand(game.player_hand());

                            if !game_state_display.show_game_state(
                                "You hit!",
                                Some(game.player_value()),
                                Some(game.dealer_value()),
                            ) {
                                info!("Failed to update display");
                            }

                            Timer::after(Duration::from_secs(3)).await;

                            if game.state() == GameState::GameOver {
                                // Player busted - dealer wins automatically
                                leds.player_led.set_color(LedColor::Red);

                                if !game_state_display.show_game_state(
                                    "Bust! You lose.",
                                    Some(game.player_value()),
                                    None,
                                ) {
                                    info!("Failed to update display");
                                }

                                Timer::after(Duration::from_secs(3)).await;

                                // Note: Game state is already GameOver, so it will skip to the final result
                                // The dealer's hidden card remains hidden since they didn't need to play
                            } else {
                                if game.player_value() == 21 {
                                    if !game_state_display.show_game_state(
                                        "21 points!\nWe move to\ndealer's turn.",
                                        None,
                                        None,
                                    ) {
                                        info!("Failed to update display");
                                    }

                                    Timer::after(Duration::from_secs(3)).await;
                                    game.player_has_21();
                                }
                            }
                        }

                        leds.player_led.set_color(LedColor::Off);
                    }
                    embassy_futures::select::Either::Second(_) => {
                        info!("STAND button pressed!");
                        leds.player_led.set_color(LedColor::Blue);

                        if !game_state_display.show_game_state(
                            "You stand!",
                            Some(game.player_value()),
                            None, // Don't show dealer score to avoid spoiling
                        ) {
                            info!("Failed to update display");
                        }

                        Timer::after(Duration::from_secs(3)).await;
                        leds.player_led.set_color(LedColor::Off);
                        game.player_stand();
                    }
                }
            }

            GameState::DealerTurn => {
                // Start the detailed dealer turn sequence
                if !game_state_display.show_game_state("Dealer's turn...", None, None) {
                    info!("Failed to update display");
                }

                leds.dealer_led.set_color(LedColor::Blue);
                Timer::after(Duration::from_secs(2)).await;
                leds.dealer_led.set_color(LedColor::Green);

                // Move to revealing phase
                game.start_dealer_turn();
            }

            GameState::DealerRevealing => {
                info!("=== DEALER REVEALING CARDS ===");

                // Show "dealer will reveal cards" message
                if !game_state_display.show_game_state(
                    "Dealer will\nreveal his\ncards...",
                    None,
                    None,
                ) {
                    info!("Failed to update display");
                }

                Timer::after(Duration::from_secs(2)).await;

                // Reveal the cards in the game logic
                game.reveal_dealer_cards();

                // Update the card display to show all cards face up
                let _ = card_ui.update_dealer_hand(game.dealer_hand());

                // Show "Cards revealed" with dealer score for 3 seconds
                if !game_state_display.show_game_state(
                    "Cards revealed",
                    None, // Don't show player score anymore
                    Some(game.dealer_value()),
                ) {
                    info!("Failed to update display");
                }

                Timer::after(Duration::from_secs(3)).await;
            }

            GameState::DealerDrawing => {
                info!("=== DEALER DRAWING PHASE ===");

                // Check if dealer needs to draw more cards
                if game.dealer_needs_card() {
                    // Show "dealer will draw one more card" message
                    if !game_state_display.show_game_state(
                        "Dealer will\ndraw one\nmore card",
                        None,
                        None,
                    ) {
                        info!("Failed to update display");
                    }

                    Timer::after(Duration::from_secs(3)).await;

                    // Dealer draws a card
                    let card_drawn = game.dealer_draw_card();

                    if card_drawn {
                        // Update the display to show the new card
                        let _ = card_ui.update_dealer_hand(game.dealer_hand());

                        // Show the new dealer value
                        if !game_state_display.show_game_state(
                            "New dealer\nvalue!",
                            None, // Don't show player value
                            Some(game.dealer_value()),
                        ) {
                            info!("Failed to update display");
                        }

                        Timer::after(Duration::from_secs(3)).await;

                        // If dealer busted or reached 17+, the game state will have changed to GameOver
                        // Otherwise, we'll loop back to draw another card
                    }
                } else {
                    // Dealer is done drawing, should move to game over
                    info!("Dealer finished drawing");
                }
            }

            GameState::GameOver => {
                info!("=== GAME OVER ===");

                // Turn off any active LEDs first
                leds.dealer_led.set_color(LedColor::Off);
                leds.player_led.set_color(LedColor::Off);

                // Check if this is a bust scenario
                let is_player_bust = game.player_value() > 21;

                if is_player_bust {
                    // Player busted - show bust message, dealer didn't need to play
                    if !game_state_display.show_game_state(
                        "Player busted!\nDealer wins!",
                        None, // Don't show player score either - just the message
                        None, // Don't show dealer score since they didn't play
                    ) {
                        info!("Failed to update display");
                    }
                } else {
                    // Normal game end - show "Dealer Finished" message with final dealer score
                    if !game_state_display.show_game_state(
                        "Dealer\nFinished",
                        None, // Don't show player score
                        Some(game.dealer_value()),
                    ) {
                        info!("Failed to update display");
                    }
                }

                Timer::after(Duration::from_secs(3)).await;

                // Now show the final result
                let (message, player_color, dealer_color) = match game.result() {
                    GameResult::PlayerWins => ("You win!", LedColor::Green, LedColor::Red),
                    GameResult::DealerWins => {
                        if is_player_bust {
                            ("You busted!", LedColor::Red, LedColor::Green)
                        } else {
                            ("Dealer wins!", LedColor::Red, LedColor::Green)
                        }
                    }
                    GameResult::Push => ("It's a tie.", LedColor::Yellow, LedColor::Yellow),
                    GameResult::InProgress => ("Game in progress...", LedColor::Off, LedColor::Off),
                };

                // Set LEDs based on result
                leds.player_led.set_color(player_color);
                leds.dealer_led.set_color(dealer_color);

                // Show final result
                if is_player_bust {
                    // For bust, show final message without any scores
                    if !game_state_display.show_game_state(
                        message, None, // Don't show any scores for bust scenario
                        None,
                    ) {
                        info!("Failed to update display");
                    }
                } else {
                    // Normal game end - show both scores
                    if !game_state_display.show_game_state(
                        message,
                        Some(game.player_value()),
                        Some(game.dealer_value()),
                    ) {
                        info!("Failed to update display");
                    }
                }

                // Wait before starting a new game
                Timer::after(Duration::from_secs(5)).await;

                // Reset LEDs
                leds.player_led.set_color(LedColor::Off);
                leds.dealer_led.set_color(LedColor::Off);

                // Reset to waiting for start
                game = BlackJackGame::new();

                // Clear all card displays
                let _ = card_ui.clear_all();

                // Show welcome message again
                if !game_state_display.show_welcome() {
                    info!("Failed to show welcome message");
                }
            }
        }

        // Small delay to prevent busy-waiting
        Timer::after(Duration::from_millis(10)).await;
    }
}
