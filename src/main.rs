pub mod cards;

fn main() {
    let deck = cards::new_deck();
    let lines = deck
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
}

