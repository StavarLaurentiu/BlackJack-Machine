// src/game/state.rs - Updated version with step-by-step dealer control
use super::card::{Suit, Value};
use super::deck::Deck;
use super::hand::Hand;
use defmt::info;

/// Game state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WaitingForStart,
    DealerDealing,
    PlayerTurn,
    DealerTurn,
    DealerRevealing, // NEW: Dealer reveals hidden card
    DealerDrawing,   // NEW: Dealer is drawing additional cards
    GameOver,
}

/// Game result enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    PlayerWins,
    DealerWins,
    Push, // Tie
    InProgress,
}

/// BlackJack game manager
pub struct BlackJackGame {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    state: GameState,
    result: GameResult,
    dealer_cards_revealed: bool, // NEW: Track if dealer cards are revealed
}

impl BlackJackGame {
    /// Create a new BlackJack game
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            state: GameState::WaitingForStart,
            result: GameResult::InProgress,
            dealer_cards_revealed: false,
        }
    }

    /// Start a new game
    pub fn start_game(&mut self) {
        info!("Starting new BlackJack game");

        // Create a new deck
        self.deck = Deck::new();

        // Clear hands
        self.player_hand = Hand::new();
        self.dealer_hand = Hand::new();

        // Update state
        self.state = GameState::DealerDealing;
        self.result = GameResult::InProgress;
        self.dealer_cards_revealed = false;
    }

    /// Deal initial cards (2 to each player)
    pub fn deal_initial_cards(&mut self) {
        // Deal first card to player (face up)
        if let Some(card) = self.deck.draw() {
            info!(
                "Player receives: {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );
            self.player_hand.add_card(card);
        }

        // Deal first card to dealer (face up)
        if let Some(mut card) = self.deck.draw() {
            card.set_face_up(true);
            info!(
                "Dealer receives (face up): {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );
            self.dealer_hand.add_card(card);
        }

        // Deal second card to player (face up)
        if let Some(card) = self.deck.draw() {
            info!(
                "Player receives: {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );
            self.player_hand.add_card(card);
        }

        // Deal second card to dealer (face down)
        if let Some(mut card) = self.deck.draw() {
            card.set_face_up(false);
            info!(
                "Dealer receives (face down): {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );
            self.dealer_hand.add_card(card);
        }

        // Print current player hand
        info!(
            "Player has {} cards with score: {}",
            self.player_hand.card_count(),
            self.player_hand.value()
        );
        for i in 0..self.player_hand.card_count() {
            if let Some(card) = self.player_hand.get_card(i) {
                info!(
                    "  Card {}: {} of {}",
                    i + 1,
                    format_value(card.value),
                    format_suit(card.suit)
                );
            }
        }

        // Print current dealer hand
        let visible_score = self.dealer_value(); // This should now only count face-up cards
        info!(
            "Dealer has {} cards with visible score: {}",
            self.dealer_hand.card_count(),
            visible_score
        );
        for i in 0..self.dealer_hand.card_count() {
            if let Some(card) = self.dealer_hand.get_card(i) {
                if card.is_face_up {
                    info!(
                        "  Card {}: {} of {}",
                        i + 1,
                        format_value(card.value),
                        format_suit(card.suit)
                    );
                } else {
                    info!("  Card {}: [HIDDEN]", i + 1);
                }
            }
        }

        // Check for BlackJack
        if self.player_hand.is_blackjack() {
            info!("Player has BlackJack!");
            self.dealer_hand.reveal_all();
            self.dealer_cards_revealed = true;

            if self.dealer_hand.is_blackjack() {
                // Both have BlackJack, it's a push
                info!("Dealer also has BlackJack! Push.");
                self.result = GameResult::Push;
            } else {
                // Player wins with BlackJack
                info!("Player wins with BlackJack!");
                self.result = GameResult::PlayerWins;
            }

            self.state = GameState::GameOver;
        } else {
            // Continue to player's turn
            self.state = GameState::PlayerTurn;
        }
    }

    /// Player hits (draws a card)
    pub fn player_hit(&mut self) -> bool {
        if self.state != GameState::PlayerTurn {
            info!("Cannot hit: not player's turn");
            return false;
        }

        info!("Player hits");

        // Draw a card for the player
        if let Some(card) = self.deck.draw() {
            info!(
                "Player receives: {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );

            // Try to add the card - if it fails, player has reached display limit
            if !self.player_hand.add_card(card) {
                info!("Player has reached maximum cards (4) - automatically standing");
                self.state = GameState::DealerTurn;
                return true;
            }

            // Print current player hand
            info!(
                "Player now has {} cards with score: {}",
                self.player_hand.card_count(),
                self.player_hand.value()
            );
            for i in 0..self.player_hand.card_count() {
                if let Some(card) = self.player_hand.get_card(i) {
                    info!(
                        "  Card {}: {} of {}",
                        i + 1,
                        format_value(card.value),
                        format_suit(card.suit)
                    );
                }
            }

            // Check if player busts
            if self.player_hand.is_bust() {
                info!(
                    "Player busts with score {}! Dealer wins automatically.",
                    self.player_hand.value()
                );
                // When player busts, dealer wins automatically - no need to reveal dealer cards
                // Keep dealer cards as they were (don't reveal the hidden card)
                self.result = GameResult::DealerWins;
                self.state = GameState::GameOver;
            }

            return true;
        }

        false
    }

    /// Player stands (ends turn)
    pub fn player_stand(&mut self) -> bool {
        if self.state != GameState::PlayerTurn {
            info!("Cannot stand: not player's turn");
            return false;
        }

        info!("Player stands");

        // Move to dealer's turn
        self.state = GameState::DealerTurn;

        true
    }

    /// NEW: Start dealer's turn by revealing cards
    pub fn start_dealer_turn(&mut self) {
        if self.state != GameState::DealerTurn {
            return;
        }

        info!("Starting dealer's turn - revealing cards");
        self.state = GameState::DealerRevealing;
    }

    /// NEW: Reveal dealer's hidden cards
    pub fn reveal_dealer_cards(&mut self) {
        if self.state != GameState::DealerRevealing {
            return;
        }

        info!("Revealing dealer's hidden cards");

        // Reveal all dealer cards
        self.dealer_hand.reveal_all();
        self.dealer_cards_revealed = true;

        // Print dealer's full hand
        info!(
            "Dealer reveals hand with score: {}",
            self.dealer_hand.value()
        );
        for i in 0..self.dealer_hand.card_count() {
            if let Some(card) = self.dealer_hand.get_card(i) {
                info!(
                    "  Card {}: {} of {}",
                    i + 1,
                    format_value(card.value),
                    format_suit(card.suit)
                );
            }
        }

        // Check if dealer needs to draw more cards
        if self.dealer_hand.value() < 17 {
            self.state = GameState::DealerDrawing;
        } else {
            // Dealer stands, game is over
            self.state = GameState::GameOver;
            self.determine_winner();
        }
    }

    /// NEW: Check if dealer needs to draw a card
    pub fn dealer_needs_card(&self) -> bool {
        self.state == GameState::DealerDrawing && self.dealer_hand.value() < 17
    }

    /// NEW: Dealer draws one card
    pub fn dealer_draw_card(&mut self) -> bool {
        if self.state != GameState::DealerDrawing {
            info!("Cannot draw: not dealer's drawing phase");
            return false;
        }

        if self.dealer_hand.value() >= 17 {
            info!("Dealer stands with {}", self.dealer_hand.value());
            self.state = GameState::GameOver;
            self.determine_winner();
            return false;
        }

        info!("Dealer draws a card");

        if let Some(card) = self.deck.draw() {
            info!(
                "Dealer receives: {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );

            // Try to add the card - if it fails, dealer has reached display limit
            if !self.dealer_hand.add_card(card) {
                info!("Dealer has reached maximum cards (4) - automatically standing");
                self.state = GameState::GameOver;
                self.determine_winner();
                return true;
            }

            // Print current dealer hand
            info!(
                "Dealer now has {} cards with score: {}",
                self.dealer_hand.card_count(),
                self.dealer_hand.value()
            );
            for i in 0..self.dealer_hand.card_count() {
                if let Some(card) = self.dealer_hand.get_card(i) {
                    info!(
                        "  Card {}: {} of {}",
                        i + 1,
                        format_value(card.value),
                        format_suit(card.suit)
                    );
                }
            }

            // Check if dealer busts
            if self.dealer_hand.is_bust() {
                info!("Dealer busts with score {}!", self.dealer_hand.value());
                self.result = GameResult::PlayerWins;
                self.state = GameState::GameOver;
                return true;
            }

            // Check if dealer must stand
            if self.dealer_hand.value() >= 17 {
                info!("Dealer must stand with {}", self.dealer_hand.value());
                self.state = GameState::GameOver;
                self.determine_winner();
            }

            return true;
        }

        false
    }

    /// Determine the winner after dealer's turn
    fn determine_winner(&mut self) {
        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        match player_value.cmp(&dealer_value) {
            core::cmp::Ordering::Greater => {
                info!("Player wins: {} vs {}", player_value, dealer_value);
                self.result = GameResult::PlayerWins;
            }
            core::cmp::Ordering::Less => {
                info!("Dealer wins: {} vs {}", dealer_value, player_value);
                self.result = GameResult::DealerWins;
            }
            core::cmp::Ordering::Equal => {
                info!("Push: {} vs {}", player_value, dealer_value);
                self.result = GameResult::Push;
            }
        }
    }

    /// Get the current game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Get the game result
    pub fn result(&self) -> GameResult {
        self.result
    }

    /// Get player's hand value
    pub fn player_value(&self) -> u8 {
        self.player_hand.value()
    }

    /// Get dealer's hand value
    pub fn dealer_value(&self) -> u8 {
        // Only show dealer's visible cards value during player's turn and before cards are revealed
        if !self.dealer_cards_revealed
            && (self.state == GameState::PlayerTurn || self.state == GameState::DealerDealing)
        {
            let mut value = 0;
            for i in 0..self.dealer_hand.card_count() {
                if let Some(card) = self.dealer_hand.get_card(i) {
                    if card.is_face_up {
                        value += card.blackjack_value();
                    }
                }
            }
            value
        } else {
            self.dealer_hand.value()
        }
    }

    /// Get a reference to the player's hand
    pub fn player_hand(&self) -> &Hand {
        &self.player_hand
    }

    /// Get a reference to the dealer's hand
    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer_hand
    }

    /// Function called when player gets to 21
    pub fn player_has_21(&mut self) {
        info!("Player has 21!");
        self.state = GameState::DealerTurn;
    }
}

// Helper function to format a card value for printing
fn format_value(value: Value) -> &'static str {
    match value {
        Value::Ace => "Ace",
        Value::Two => "2",
        Value::Three => "3",
        Value::Four => "4",
        Value::Five => "5",
        Value::Six => "6",
        Value::Seven => "7",
        Value::Eight => "8",
        Value::Nine => "9",
        Value::Ten => "10",
        Value::Jack => "Jack",
        Value::Queen => "Queen",
        Value::King => "King",
    }
}

// Helper function to format a card suit for printing
fn format_suit(suit: Suit) -> &'static str {
    match suit {
        Suit::Hearts => "Hearts",
        Suit::Diamonds => "Diamonds",
        Suit::Clubs => "Clubs",
        Suit::Spades => "Spades",
    }
}
