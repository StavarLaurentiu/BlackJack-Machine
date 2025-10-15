use crate::game::{Card, Hand};
use crate::hardware::CardDisplays;
use crate::hardware::DisplayPosition;
use crate::images;
use defmt::info;

/// Manager for updating card displays
pub struct CardUI<I, E>
where
    I: embedded_hal_1::i2c::I2c<Error = E>,
{
    card_displays: CardDisplays<I, E>,
}

impl<I, E> CardUI<I, E>
where
    I: embedded_hal_1::i2c::I2c<Error = E>,
{
    /// Create a new card UI manager
    pub fn new(card_displays: CardDisplays<I, E>) -> Self {
        Self { card_displays }
    }

    /// Initialize all card displays
    pub fn init_all_displays(&mut self) -> Result<(), E> {
        info!("Initializing all card displays");
        self.card_displays.init_all_displays()
    }

    /// Display a card using PBM image if available, otherwise fallback to text
    pub fn display_card_with_image(
        &mut self,
        card: &Card,
        position: DisplayPosition,
    ) -> Result<(), E> {
        info!("Displaying card with image at position {:?}", position);

        if let Some(image_data) = images::get_card_image(card) {
            // Display PBM image
            info!("Using PBM image for card (face_up: {})", card.is_face_up);
            self.card_displays.display_pbm_image(image_data, position)?;
        } else {
            // Fallback to text-based display
            info!(
                "Using text-based display for card: {} of {}",
                format_value(card.value),
                format_suit(card.suit)
            );
            self.card_displays.display_card(card, position)?;
        }

        Ok(())
    }

    /// Update displays with the dealer's hand using images
    pub fn update_dealer_hand(&mut self, hand: &Hand) -> Result<(), E> {
        info!(
            "Updating dealer hand displays with {} cards",
            hand.card_count()
        );

        // Clear all dealer displays first
        for position in &[
            DisplayPosition::DealerCard1,
            DisplayPosition::DealerCard2,
            DisplayPosition::DealerCard3,
            DisplayPosition::DealerCard4,
        ] {
            self.card_displays.clear_display(*position)?;
        }

        // Show each card
        let card_positions = [
            DisplayPosition::DealerCard1,
            DisplayPosition::DealerCard2,
            DisplayPosition::DealerCard3,
            DisplayPosition::DealerCard4,
        ];

        for (i, position) in card_positions.iter().enumerate() {
            if i < hand.card_count() {
                if let Some(card) = hand.get_card(i) {
                    info!(
                        "Displaying dealer card {} at position {:?}",
                        i + 1,
                        position
                    );
                    self.display_card_with_image(card, *position)?;
                }
            }
        }

        Ok(())
    }

    /// Update displays with the player's hand using images
    pub fn update_player_hand(&mut self, hand: &Hand) -> Result<(), E> {
        info!(
            "Updating player hand displays with {} cards",
            hand.card_count()
        );

        // Clear all player displays first
        for position in &[
            DisplayPosition::PlayerCard1,
            DisplayPosition::PlayerCard2,
            DisplayPosition::PlayerCard3,
            DisplayPosition::PlayerCard4,
        ] {
            self.card_displays.clear_display(*position)?;
        }

        // Show each card
        let card_positions = [
            DisplayPosition::PlayerCard1,
            DisplayPosition::PlayerCard2,
            DisplayPosition::PlayerCard3,
            DisplayPosition::PlayerCard4,
        ];

        for (i, position) in card_positions.iter().enumerate() {
            if i < hand.card_count() {
                if let Some(card) = hand.get_card(i) {
                    info!(
                        "Displaying player card {} at position {:?}",
                        i + 1,
                        position
                    );
                    self.display_card_with_image(card, *position)?;
                }
            }
        }

        Ok(())
    }

    /// Clear all displays
    pub fn clear_all(&mut self) -> Result<(), E> {
        info!("Clearing all card displays");
        self.card_displays.clear_all_displays()
    }
}

// Helper function to format a card value for printing
fn format_value(value: crate::game::Value) -> &'static str {
    match value {
        crate::game::Value::Ace => "Ace",
        crate::game::Value::Two => "2",
        crate::game::Value::Three => "3",
        crate::game::Value::Four => "4",
        crate::game::Value::Five => "5",
        crate::game::Value::Six => "6",
        crate::game::Value::Seven => "7",
        crate::game::Value::Eight => "8",
        crate::game::Value::Nine => "9",
        crate::game::Value::Ten => "10",
        crate::game::Value::Jack => "Jack",
        crate::game::Value::Queen => "Queen",
        crate::game::Value::King => "King",
    }
}

// Helper function to format a card suit for printing
fn format_suit(suit: crate::game::Suit) -> &'static str {
    match suit {
        crate::game::Suit::Hearts => "Hearts",
        crate::game::Suit::Diamonds => "Diamonds",
        crate::game::Suit::Clubs => "Clubs",
        crate::game::Suit::Spades => "Spades",
    }
}
