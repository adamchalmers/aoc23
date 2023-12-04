use std::collections::{HashMap, HashSet};

/// A card is just a set of winning numbers.
type Card = Vec<usize>;

fn main() {
    let input = include_str!("../input.txt");
    let cards: Vec<_> = input.lines().map(parse).collect();
    let q1: usize = cards.iter().map(score).sum();
    println!("Q1: {q1}");
    println!("Q2: {}", q2(&cards));
}

fn parse(line: &str) -> Card {
    let (wins, haves) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let winners = number_list(wins);
    let have = number_list(haves);

    winners.intersection(&have).copied().collect()
}

fn score(card: &Card) -> usize {
    let num_matches = card.len();
    if num_matches == 0 {
        0
    } else {
        2usize.pow(num_matches as u32 - 1)
    }
}

/// Parses whitespace-separated list of numbers.
fn number_list(s: &str) -> HashSet<usize> {
    s.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn q2(cards: &[Card]) -> usize {
    let n = cards.len();

    // Map card IDs to how many copies there are.
    // Starts with 1 copy of each card.
    let mut copies: HashMap<usize, usize> = (0..n).map(|id| (id, 1)).collect();
    for card_id in 0..n {
        let number_cards_won = cards[card_id].len();
        for i in 0..number_cards_won {
            // You won a copy of this card ID.
            let id = i + card_id + 1;
            // You won 1 copy of it for each copy of the current card.
            let copies_of_this_card = copies[&card_id];
            *copies.get_mut(&id).unwrap() += copies_of_this_card;
        }
    }

    copies.values().sum()
}
