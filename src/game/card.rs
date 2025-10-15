/// Card suits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

/// Card values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// Playing card representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
    pub is_face_up: bool,
}

impl Card {
    /// Create a new card
    pub fn new(suit: Suit, value: Value, is_face_up: bool) -> Self {
        Self {
            suit,
            value,
            is_face_up,
        }
    }

    /// Get the card's value in BlackJack
    pub fn blackjack_value(&self) -> u8 {
        match self.value {
            Value::Ace => 11, // Aces are 11 initially, but can be 1 if needed
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
        }
    }

    /// Show or hide the card
    pub fn set_face_up(&mut self, is_face_up: bool) {
        self.is_face_up = is_face_up;
    }
}
