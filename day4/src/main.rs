use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let cards: Vec<_> = input.lines().map(Card::parse).collect();
    let q1: u32 = cards.iter().map(Card::score).sum();
    println!("Q1: {q1}");
}

#[derive(Debug)]
struct Card {
    id: usize,
    winners: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (header, nums) = line.split_once(": ").unwrap();
        let id = header
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let (wins, haves) = nums.split_once(" | ").unwrap();
        let winners = wins
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have = haves
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Card { id, winners, have }
    }

    fn score(&self) -> u32 {
        let num_matches = self.winners.intersection(&self.have).count() as u32;
        if num_matches == 0 {
            0
        } else {
            2u32.pow(num_matches - 1)
        }
    }
}
