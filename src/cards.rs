use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Club => "♣",
                Suit::Spade => "♠",
                Suit::Diamond => "♦",
                Suit::Heart => "♥",
            }
        )
    }
}
#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Rank {
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
    Ace,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Ace => "A",
                Rank::King => "K",
                Rank::Queen => "Q",
                Rank::Jack => "J",
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn suit(mut self, suit: Suit) -> Self {
        self.suit = suit;
        self
    }
    pub fn rank(mut self, rank: Rank) -> Self {
        self.rank = rank;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            suit: Suit::Spade,
            rank: Rank::Ace,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

pub fn new_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            cards.push(Card::default().suit(suit).rank(rank))
        }
    }
    cards
}
