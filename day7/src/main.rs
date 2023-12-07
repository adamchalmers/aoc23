use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let data = parse(include_str!("../input.txt"));
    println!("Q1: {}", q1(&data));
    println!("Q2: {}", q2(&data));
}

#[derive(Debug, Clone)]
struct Row {
    hand: Hand,
    bid: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Hand([char; 5]);

const CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn is_better(this: char, other: char) -> bool {
    let this_pos = CARD_VALUES.iter().position(|c| c == &this).unwrap();
    let other_pos = CARD_VALUES.iter().position(|c| c == &other).unwrap();
    this_pos < other_pos
}

impl Hand {
    fn classify_q2(&self) -> HandType {
        let mut freqs = HashMap::new();
        let mut jokers = 0;
        for ch in self.0 {
            if ch == 'J' {
                jokers += 1
            } else {
                *freqs.entry(ch).or_insert(0) += 1;
            }
        }
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }
        if jokers > 0 {
            // If there's already a card with multiples, convert the jokers to that card.
            let (most_freq_character, frequency) = freqs
                .iter()
                .max_by(|this, other| this.1.cmp(other.1))
                .unwrap()
                .to_owned();
            if frequency > &1 {
                freqs.insert(*most_freq_character, frequency + jokers);
            } else {
                // Make the joker the highest-value card I guess
                for ch in CARD_VALUES {
                    if let Some(x) = freqs.get(&ch) {
                        freqs.insert(ch, x + jokers);
                        break;
                    }
                }
            }
        }
        let mut vals: Vec<usize> = freqs.values().copied().collect();
        vals.sort();
        if vals == vec![5] {
            HandType::FiveOfAKind
        } else if vals == vec![1, 4] {
            HandType::FourOfAKind
        } else if vals == vec![2, 3] {
            HandType::FullHouse
        } else if vals.contains(&3) {
            HandType::ThreeOfAKind
        } else if vals == vec![1, 2, 2] {
            HandType::TwoPair
        } else if vals.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn is_winner(&self, other: &Self) -> bool {
        if (self.classify() as i32) < (other.classify() as i32) {
            true
        } else if (self.classify() as i32) > (other.classify() as i32) {
            false
        } else {
            for i in 0..5 {
                if self.0[i] == other.0[i] {
                    continue;
                }
                return is_better(self.0[i], other.0[i]);
            }
            // Complete tie
            panic!("Complete tie, this shouldn't happen");
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.is_winner(&other) {
            Ordering::Greater
        } else {
            Ordering::Less
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 1,
    /// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    /// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    /// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    /// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    /// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    /// High card, where all cards' labels are distinct: 23456
    HighCard,
}

impl Row {
    fn parse(s: &str) -> Self {
        let (hand, bid) = s.split_once(' ').unwrap();
        let hand = Hand(hand.chars().collect::<Vec<_>>().try_into().unwrap());
        let bid = bid.parse().unwrap();
        Row { hand, bid }
    }
}

struct Data {
    rows: Vec<Row>,
}

fn parse(s: &str) -> Data {
    let rows = s.lines().map(Row::parse).collect();
    Data { rows }
}

fn total_scores(data: &Data) -> usize {
    let mut rows = data.rows.to_vec();
    rows.sort_by(|this, other| this.hand.cmp(&other.hand).reverse());
    let mut prod = 0;
    let n = rows.len();
    for (i, row) in rows.iter().enumerate() {
        let rank = n - i;
        prod += rank * row.bid;
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let example = include_str!("../example.txt");
        let data = parse(example);
        let actual = q1(&data);
        let expected = 5905;
        assert_eq!(actual, expected);
    }

    #[test]
    fn hand_types() {
        assert_eq!(Hand(['A'; 5]).classify(), HandType::FiveOfAKind);
        assert_eq!(
            Hand(['A', 'A', '8', 'A', 'A']).classify(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand(['2', '3', '3', '3', '2']).classify(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand(['T', 'T', 'T', '9', '8']).classify(),
            HandType::ThreeOfAKind
        );

        assert_eq!(
            Hand(['2', '3', '4', '3', '2']).classify(),
            HandType::TwoPair
        );

        assert_eq!(
            Hand(['A', '2', '3', 'A', '4']).classify(),
            HandType::OnePair
        );
        assert_eq!(
            Hand(['2', '3', '4', '5', '6']).classify(),
            HandType::HighCard
        );
        assert_eq!(
            Hand(['3', '2', 'T', '3', 'K']).classify(),
            HandType::OnePair
        );
        assert_eq!(
            Hand(['K', 'K', '6', '7', '7']).classify(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand(['T', '5', '5', 'J', '5']).classify(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand(['Q', 'Q', 'Q', 'J', 'A']).classify(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand(['K', 'T', 'J', 'J', 'T']).classify(),
            HandType::FourOfAKind
        );
    }

    #[test]
    fn ordering() {
        // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
        let h1 = Hand(['3', '3', '3', '3', '2']);
        let h2 = Hand(['2', 'A', 'A', 'A', 'A']);
        assert_eq!(h1.classify(), HandType::FourOfAKind);
        assert_eq!(h2.classify(), HandType::FourOfAKind);
        assert!(h1 > h2);
    }
}
