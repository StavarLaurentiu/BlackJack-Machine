use super::card::{Card, Suit, Value};
use embassy_time::Instant;
use heapless::Vec;

/// Deck of cards
pub struct Deck {
    cards: Vec<Card, 52>,
}

impl Deck {
    /// Create a new, shuffled deck of 52 cards
    pub fn new() -> Self {
        let mut cards = Vec::new();

        // Add all 52 cards to the deck
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter() {
            for value in [
                Value::Ace,
                Value::Two,
                Value::Three,
                Value::Four,
                Value::Five,
                Value::Six,
                Value::Seven,
                Value::Eight,
                Value::Nine,
                Value::Ten,
                Value::Jack,
                Value::Queen,
                Value::King,
            ]
            .iter()
            {
                // This unwrap is safe because we know cards.len() < 52
                cards.push(Card::new(*suit, *value, true)).unwrap();
            }
        }

        // Shuffle the deck
        Self::shuffle(&mut cards);

        Self { cards }
    }

    /// Shuffle the deck using a simplified pseudo-random approach
    fn shuffle(cards: &mut Vec<Card, 52>) {
        // Use current time as a simple seed
        let seed = Instant::now().as_micros() as u64;

        // Simple Fisher-Yates shuffle with our own basic RNG
        for i in (1..cards.len()).rev() {
            // Generate a primitive pseudo-random index
            let j = ((seed.wrapping_mul(i as u64)) % (i as u64 + 1)) as usize;

            // Swap elements at indices i and j
            cards.swap(i, j);
        }
    }

    /// Draw a card from the deck
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
