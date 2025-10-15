use crate::game::{Card, Suit, Value};

/// Get PBM image data for a specific card
pub fn get_card_image(card: &Card) -> Option<&'static [u8]> {
    if !card.is_face_up {
        return Some(HIDDEN_CARD);
    }

    match (card.value, card.suit) {
        // Hearts
        (Value::Ace, Suit::Hearts) => Some(ACE_HEARTS),
        (Value::Two, Suit::Hearts) => Some(TWO_HEARTS),
        (Value::Three, Suit::Hearts) => Some(THREE_HEARTS),
        (Value::Four, Suit::Hearts) => Some(FOUR_HEARTS),
        (Value::Five, Suit::Hearts) => Some(FIVE_HEARTS),
        (Value::Six, Suit::Hearts) => Some(SIX_HEARTS),
        (Value::Seven, Suit::Hearts) => Some(SEVEN_HEARTS),
        (Value::Eight, Suit::Hearts) => Some(EIGHT_HEARTS),
        (Value::Nine, Suit::Hearts) => Some(NINE_HEARTS),
        (Value::Ten, Suit::Hearts) => Some(TEN_HEARTS),
        (Value::Jack, Suit::Hearts) => Some(JACK_HEARTS),
        (Value::Queen, Suit::Hearts) => Some(QUEEN_HEARTS),
        (Value::King, Suit::Hearts) => Some(KING_HEARTS),

        // Diamonds
        (Value::Ace, Suit::Diamonds) => Some(ACE_DIAMONDS),
        (Value::Two, Suit::Diamonds) => Some(TWO_DIAMONDS),
        (Value::Three, Suit::Diamonds) => Some(THREE_DIAMONDS),
        (Value::Four, Suit::Diamonds) => Some(FOUR_DIAMONDS),
        (Value::Five, Suit::Diamonds) => Some(FIVE_DIAMONDS),
        (Value::Six, Suit::Diamonds) => Some(SIX_DIAMONDS),
        (Value::Seven, Suit::Diamonds) => Some(SEVEN_DIAMONDS),
        (Value::Eight, Suit::Diamonds) => Some(EIGHT_DIAMONDS),
        (Value::Nine, Suit::Diamonds) => Some(NINE_DIAMONDS),
        (Value::Ten, Suit::Diamonds) => Some(TEN_DIAMONDS),
        (Value::Jack, Suit::Diamonds) => Some(JACK_DIAMONDS),
        (Value::Queen, Suit::Diamonds) => Some(QUEEN_DIAMONDS),
        (Value::King, Suit::Diamonds) => Some(KING_DIAMONDS),

        // Clubs
        (Value::Ace, Suit::Clubs) => Some(ACE_CLUBS),
        (Value::Two, Suit::Clubs) => Some(TWO_CLUBS),
        (Value::Three, Suit::Clubs) => Some(THREE_CLUBS),
        (Value::Four, Suit::Clubs) => Some(FOUR_CLUBS),
        (Value::Five, Suit::Clubs) => Some(FIVE_CLUBS),
        (Value::Six, Suit::Clubs) => Some(SIX_CLUBS),
        (Value::Seven, Suit::Clubs) => Some(SEVEN_CLUBS),
        (Value::Eight, Suit::Clubs) => Some(EIGHT_CLUBS),
        (Value::Nine, Suit::Clubs) => Some(NINE_CLUBS),
        (Value::Ten, Suit::Clubs) => Some(TEN_CLUBS),
        (Value::Jack, Suit::Clubs) => Some(JACK_CLUBS),
        (Value::Queen, Suit::Clubs) => Some(QUEEN_CLUBS),
        (Value::King, Suit::Clubs) => Some(KING_CLUBS),

        // Spades
        (Value::Ace, Suit::Spades) => Some(ACE_SPADES),
        (Value::Two, Suit::Spades) => Some(TWO_SPADES),
        (Value::Three, Suit::Spades) => Some(THREE_SPADES),
        (Value::Four, Suit::Spades) => Some(FOUR_SPADES),
        (Value::Five, Suit::Spades) => Some(FIVE_SPADES),
        (Value::Six, Suit::Spades) => Some(SIX_SPADES),
        (Value::Seven, Suit::Spades) => Some(SEVEN_SPADES),
        (Value::Eight, Suit::Spades) => Some(EIGHT_SPADES),
        (Value::Nine, Suit::Spades) => Some(NINE_SPADES),
        (Value::Ten, Suit::Spades) => Some(TEN_SPADES),
        (Value::Jack, Suit::Spades) => Some(JACK_SPADES),
        (Value::Queen, Suit::Spades) => Some(QUEEN_SPADES),
        (Value::King, Suit::Spades) => Some(KING_SPADES),
    }
}

// Hidden card (card back) - always required
const HIDDEN_CARD: &[u8] = include_bytes!("../../assets/hidden.pbm");

// Hearts
const ACE_HEARTS: &[u8] = include_bytes!("../../assets/ace_hearts.pbm");
const TWO_HEARTS: &[u8] = include_bytes!("../../assets/two_hearts.pbm");
const THREE_HEARTS: &[u8] = include_bytes!("../../assets/three_hearts.pbm");
const FOUR_HEARTS: &[u8] = include_bytes!("../../assets/four_hearts.pbm");
const FIVE_HEARTS: &[u8] = include_bytes!("../../assets/five_hearts.pbm");
const SIX_HEARTS: &[u8] = include_bytes!("../../assets/six_hearts.pbm");
const SEVEN_HEARTS: &[u8] = include_bytes!("../../assets/seven_hearts.pbm");
const EIGHT_HEARTS: &[u8] = include_bytes!("../../assets/eight_hearts.pbm");
const NINE_HEARTS: &[u8] = include_bytes!("../../assets/nine_hearts.pbm");
const TEN_HEARTS: &[u8] = include_bytes!("../../assets/ten_hearts.pbm");
const JACK_HEARTS: &[u8] = include_bytes!("../../assets/jack_hearts.pbm");
const QUEEN_HEARTS: &[u8] = include_bytes!("../../assets/queen_hearts.pbm");
const KING_HEARTS: &[u8] = include_bytes!("../../assets/king_hearts.pbm");

// Diamonds
const ACE_DIAMONDS: &[u8] = include_bytes!("../../assets/ace_diamonds.pbm");
const TWO_DIAMONDS: &[u8] = include_bytes!("../../assets/two_diamonds.pbm");
const THREE_DIAMONDS: &[u8] = include_bytes!("../../assets/three_diamonds.pbm");
const FOUR_DIAMONDS: &[u8] = include_bytes!("../../assets/four_diamonds.pbm");
const FIVE_DIAMONDS: &[u8] = include_bytes!("../../assets/five_diamonds.pbm");
const SIX_DIAMONDS: &[u8] = include_bytes!("../../assets/six_diamonds.pbm");
const SEVEN_DIAMONDS: &[u8] = include_bytes!("../../assets/seven_diamonds.pbm");
const EIGHT_DIAMONDS: &[u8] = include_bytes!("../../assets/eight_diamonds.pbm");
const NINE_DIAMONDS: &[u8] = include_bytes!("../../assets/nine_diamonds.pbm");
const TEN_DIAMONDS: &[u8] = include_bytes!("../../assets/ten_diamonds.pbm");
const JACK_DIAMONDS: &[u8] = include_bytes!("../../assets/jack_diamonds.pbm");
const QUEEN_DIAMONDS: &[u8] = include_bytes!("../../assets/queen_diamonds.pbm");
const KING_DIAMONDS: &[u8] = include_bytes!("../../assets/king_diamonds.pbm");

// Clubs
const ACE_CLUBS: &[u8] = include_bytes!("../../assets/ace_clubs.pbm");
const TWO_CLUBS: &[u8] = include_bytes!("../../assets/two_clubs.pbm");
const THREE_CLUBS: &[u8] = include_bytes!("../../assets/three_clubs.pbm");
const FOUR_CLUBS: &[u8] = include_bytes!("../../assets/four_clubs.pbm");
const FIVE_CLUBS: &[u8] = include_bytes!("../../assets/five_clubs.pbm");
const SIX_CLUBS: &[u8] = include_bytes!("../../assets/six_clubs.pbm");
const SEVEN_CLUBS: &[u8] = include_bytes!("../../assets/seven_clubs.pbm");
const EIGHT_CLUBS: &[u8] = include_bytes!("../../assets/eight_clubs.pbm");
const NINE_CLUBS: &[u8] = include_bytes!("../../assets/nine_clubs.pbm");
const TEN_CLUBS: &[u8] = include_bytes!("../../assets/ten_clubs.pbm");
const JACK_CLUBS: &[u8] = include_bytes!("../../assets/jack_clubs.pbm");
const QUEEN_CLUBS: &[u8] = include_bytes!("../../assets/queen_clubs.pbm");
const KING_CLUBS: &[u8] = include_bytes!("../../assets/king_clubs.pbm");

// Spades
const ACE_SPADES: &[u8] = include_bytes!("../../assets/ace_spades.pbm");
const TWO_SPADES: &[u8] = include_bytes!("../../assets/two_spades.pbm");
const THREE_SPADES: &[u8] = include_bytes!("../../assets/three_spades.pbm");
const FOUR_SPADES: &[u8] = include_bytes!("../../assets/four_spades.pbm");
const FIVE_SPADES: &[u8] = include_bytes!("../../assets/five_spades.pbm");
const SIX_SPADES: &[u8] = include_bytes!("../../assets/six_spades.pbm");
const SEVEN_SPADES: &[u8] = include_bytes!("../../assets/seven_spades.pbm");
const EIGHT_SPADES: &[u8] = include_bytes!("../../assets/eight_spades.pbm");
const NINE_SPADES: &[u8] = include_bytes!("../../assets/nine_spades.pbm");
const TEN_SPADES: &[u8] = include_bytes!("../../assets/ten_spades.pbm");
const JACK_SPADES: &[u8] = include_bytes!("../../assets/jack_spades.pbm");
const QUEEN_SPADES: &[u8] = include_bytes!("../../assets/queen_spades.pbm");
const KING_SPADES: &[u8] = include_bytes!("../../assets/king_spades.pbm");
