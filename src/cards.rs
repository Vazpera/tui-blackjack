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
pub enum SingleOrDouble {
    Single(u32),
    Double(u32, u32),
}

type CardValue = Option<SingleOrDouble>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum HandValue {
    Hidden,
    PartiallyHidden(SingleOrDouble),
    Shown(SingleOrDouble),
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
    pub fn get_value(&self, is_holder: bool)->CardValue {
        use SingleOrDouble::*;
        use Rank::*;
        if !is_holder && self.hidden { return None };
        match self.rank {
            Ace => Some(Double(1, 11)),
            Two => Some(Single(2)),
            Three => Some(Single(3)),
            Four => Some(Single(4)),
            Five => Some(Single(5)),
            Six => Some(Single(6)),
            Seven => Some(Single(7)),
            Eight => Some(Single(8)),
            Nine => Some(Single(9)),
            Ten => Some(Single(10)),
            Jack => Some(Single(10)),
            Queen => Some(Single(10)),
            King => Some(Single(10)),
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
pub fn choose_card(deck: &mut Vec<Card>, random: bool) -> Option<Card> {
    let mut rng = rand::thread_rng();
    if deck.len() == 0 {
        return None
    }
    let card: usize = match random {
        true => rng.gen_range(0..deck.clone().len()),
        false => 0
    };

    let removed_card = deck.remove(card);
    return Some(removed_card);
}


#[test]
fn check_card_value() {
    for rank in Rank::iter() {
        use Rank::*;
        use SingleOrDouble::*;
        let mock_card = Card::default().rank(rank);
        match mock_card.rank {
            Ace => assert_eq!(mock_card.get_value(true), Some(Double(1, 11))), 
           Two => assert_eq!(mock_card.get_value(true), Some(Single(2))),
           Three => assert_eq!(mock_card.get_value(true), Some(Single(3))),
           Four => assert_eq!(mock_card.get_value(true), Some(Single(4))),
           Five => assert_eq!(mock_card.get_value(true), Some(Single(5))),
           Six => assert_eq!(mock_card.get_value(true), Some(Single(6))),
           Seven => assert_eq!(mock_card.get_value(true), Some(Single(7))),
           Eight => assert_eq!(mock_card.get_value(true), Some(Single(8))),
           Nine => assert_eq!(mock_card.get_value(true), Some(Single(9))),
           Ten => assert_eq!(mock_card.get_value(true), Some(Single(10))),
           Jack => assert_eq!(mock_card.get_value(true), Some(Single(10))),
           Queen => assert_eq!(mock_card.get_value(true), Some(Single(10))),
           King => assert_eq!(mock_card.get_value(true), Some(Single(10))),
        }
    }
}
