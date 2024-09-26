pub mod cards;
use cards::*;

fn main() {
    let mut deck = new_deck();
    let mut new_deck: Vec<Card> = Vec::new();
    while deck.len() > 0 {
        let new_card: Option<Card>;
        (deck, new_card) = choose_card(deck, true);
        new_deck.push(new_card.expect("No card found"));
    }

    for card in new_deck {
        println!("{}", card);
    }
}



