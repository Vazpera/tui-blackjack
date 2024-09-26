use ratatui::{style::Stylize, widgets::Widget};
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
                Suit::Club => "♣".black().to_string(),
                Suit::Spade => "♠".black().to_string(),
                Suit::Diamond => "♦".red().to_string(),
                Suit::Heart => "♥".red().to_string(),
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
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Ace,
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn value(&self) -> Value {
        match self.rank {
            Rank::Ace => Value::Ace,
            Rank::Two => Value::Two,
            Rank::Three => Value::Three,
            Rank::Four => Value::Four,
            Rank::Five => Value::Five,
            Rank::Six => Value::Six,
            Rank::Seven => Value::Seven,
            Rank::Eight => Value::Eight,
            Rank::Nine => Value::Nine,
            Rank::Ten => Value::Ten,
            Rank::Jack => Value::Ten,
            Rank::Queen => Value::Ten,
            Rank::King => Value::Ten,
        }
    }
    pub fn suit(mut self, suit: Suit) -> Self {
        self.suit = suit;
        self
    }
    pub fn rank(mut self, rank: Rank) -> Self {
        self.rank = rank;
        self
    }
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self::default().rank(rank).suit(suit)
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
