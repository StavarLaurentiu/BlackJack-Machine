use super::card::Card;
use heapless::Vec;

/// BlackJack hand
pub struct Hand {
    cards: Vec<Card, 10>, // Maximum of 10 cards should be enough for BlackJack
}

impl Hand {
    /// Create a new empty hand
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: Card) -> bool {
        // Return false if the hand already has 4 cards (display limit)
        if self.cards.len() >= 4 {
            return false;
        }

        // This should never fail since we checked length above
        self.cards.push(card).unwrap();
        true
    }

    /// Calculate the BlackJack value of the hand
    pub fn value(&self) -> u8 {
        let mut value = 0;
        let mut num_aces = 0;

        // Sum up the values of all cards
        for card in &self.cards {
            if card.value == super::card::Value::Ace {
                num_aces += 1;
                value += 11; // Count aces as 11 initially
            } else {
                value += card.blackjack_value();
            }
        }

        // Convert aces from 11 to 1 as needed to avoid busting
        while value > 21 && num_aces > 0 {
            value -= 10; // Convert one ace from 11 to 1
            num_aces -= 1;
        }

        value
    }

    /// Check if the hand has busted (value > 21)
    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }

    /// Check if the hand has BlackJack (exactly 21 with 2 cards)
    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    /// Get the number of cards in the hand
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// Get a reference to a specific card in the hand
    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Flip all cards face up
    pub fn reveal_all(&mut self) {
        for card in &mut self.cards {
            card.set_face_up(true);
        }
    }
}
