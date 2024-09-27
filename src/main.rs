pub mod cards;
use cards::*;

fn main() {
    let mut deck = new_deck();
    let mut new_deck: Vec<Card> = Vec::new();
    while let (altered_deck, Some(new_card)) = choose_card(deck, true) {
        deck = altered_deck;
        new_deck.push(new_card);
    }
    let lines = new_deck.clone()
        .into_iter()
        .enumerate()
        .fold(Vec::new(), |acc: Vec<String>, (i, x)| {
            let curr = match acc.clone().last() {
                Some(j) => j.to_owned(),
                None => String::new()
            };
            let mut next = acc.clone();
            if i % 13 == 0 {
                next.push(format!("{}", x));
            } else {
                next = next
                    .clone()
                    .into_iter()
                    .take(acc.clone().len() - 1)
                    .collect::<Vec<String>>();
                next.push(curr + &format!(" {}", x).as_str());
            }
            next
        });
    for line in lines {
        println!("{line}")
    }
    println!("{}", new_deck.len());
}



