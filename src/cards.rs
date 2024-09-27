use rand::Rng;
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CardValue {
    None,
    Single(u32),
    Double(u32, u32)
}

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub hidden: bool,
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
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self::default().rank(rank).suit(suit)
    }
    pub fn value(&self, is_holder: bool) -> CardValue {
        match (self.hidden, is_holder, self.rank) {
            (true, false, _) => CardValue::None,
            (_, _, Rank::Two) => CardValue::Single(2),
            (_, _, Rank::Three) => CardValue::Single(3),
            (_, _, Rank::Four) => CardValue::Single(4),
            (_, _, Rank::Five) => CardValue::Single(5),
            (_, _, Rank::Six) => CardValue::Single(6),
            (_, _, Rank::Seven) => CardValue::Single(7),
            (_, _, Rank::Eight) => CardValue::Single(8),
            (_, _, Rank::Nine) => CardValue::Single(9),
            (_, _, Rank::Ten) => CardValue::Single(10),
            (_, _, Rank::Jack) => CardValue::Single(10),
            (_, _, Rank::Queen) => CardValue::Single(10),
            (_, _, Rank::King) => CardValue::Single(10),
            (_, _, Rank::Ace) => CardValue::Double(1, 11),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            suit: Suit::Spade,
            rank: Rank::Ace,
            hidden: false,
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
pub fn choose_card(mut deck: Vec<Card>, random: bool) -> (Vec<Card>, Option<Card>) {
    let mut rng = rand::thread_rng();
    if deck.len() == 0 {
        return (deck, None)
    }
    let card: usize = match random {
        true => rng.gen_range(0..deck.clone().len()),
        false => 0
    };

    let removed_card = deck.remove(card);
    return (deck, Some(removed_card));
}

pub fn get_hand_total(deck: Vec<Card>, is_holder: bool)->CardValue {
    return CardValue::None;
}

#[test]
fn test_card_values() {
    for rank_type in Rank::iter() {
       let mock_card = Card::default().rank(rank_type);
       match rank_type  {
           Rank::Two   => assert_eq!(CardValue::Single(    2), mock_card.value(true)),
           Rank::Three => assert_eq!(CardValue::Single(    3), mock_card.value(true)),
           Rank::Four  => assert_eq!(CardValue::Single(    4), mock_card.value(true)),
           Rank::Five  => assert_eq!(CardValue::Single(    5), mock_card.value(true)),
           Rank::Six   => assert_eq!(CardValue::Single(    6), mock_card.value(true)),
           Rank::Seven => assert_eq!(CardValue::Single(    7), mock_card.value(true)),
           Rank::Eight => assert_eq!(CardValue::Single(    8), mock_card.value(true)),
           Rank::Nine  => assert_eq!(CardValue::Single(    9), mock_card.value(true)),
           Rank::Ten   => assert_eq!(CardValue::Single(   10), mock_card.value(true)),
           Rank::Jack  => assert_eq!(CardValue::Single(   10), mock_card.value(true)),
           Rank::Queen => assert_eq!(CardValue::Single(   10), mock_card.value(true)),
           Rank::King  => assert_eq!(CardValue::Single(   10), mock_card.value(true)),
           Rank::Ace   => assert_eq!(CardValue::Double(1, 11), mock_card.value(true)),
       }
    }
}
