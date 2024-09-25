pub mod cards;
use std::ops::Deref;

use cards::{Card, Rank, Suit};

fn main() {
    let deck = cards::new_deck();
    let lines = deck
        .into_iter()
        .enumerate()
        .fold(vec!["".to_string()], |acc, (i, x)| {
            let curr = acc.clone().last().unwrap().to_owned();
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
}

