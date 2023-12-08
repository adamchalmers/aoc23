use std::{cmp::Ordering, collections::HashMap};

mod parse;

fn main() {
    let data = Data::parse(include_str!("../input.txt"));
    let q1 = total_scores(&data, Q::Q1);
    assert_eq!(250957639, q1);
    println!("Q1: {q1}");
    let q2 = total_scores(&data, Q::Q2);
    assert_eq!(251515496, q2);
    println!("Q2: {q2}");
}

#[derive(Debug, Clone)]
struct Row {
    hand: Hand,
    bid: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Hand([Card; 5]);

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
enum Card {
    A,
    K,
    Q,
    T,
    Number(u32),
    J,
}

impl Card {
    fn eval_q2(&self) -> u32 {
        match self {
            Card::A => 1,
            Card::K => 2,
            Card::Q => 3,
            Card::T => 4,
            Card::Number(n) => 9 - *n + 5,
            Card::J => 13,
        }
    }
    fn eval_q1(&self) -> u32 {
        match self {
            Card::A => 1,
            Card::K => 2,
            Card::Q => 3,
            Card::J => 4,
            Card::T => 5,
            Card::Number(n) => 9 - *n + 6,
        }
    }
    fn list_q2() -> [Self; 12] {
        [
            Card::A,
            Card::K,
            Card::Q,
            Card::T,
            Card::Number(9),
            Card::Number(8),
            Card::Number(7),
            Card::Number(6),
            Card::Number(5),
            Card::Number(4),
            Card::Number(3),
            Card::Number(2),
        ]
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
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

impl HandType {
    fn from_card_frequencies(frequencies: Vec<usize>) -> Self {
        match frequencies.as_slice() {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            vals if vals.contains(&3) => Self::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            vals if vals.contains(&2) => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl Hand {
    fn classify_q1(&self) -> HandType {
        let mut freqs = HashMap::new();
        for ch in self.0 {
            *freqs.entry(ch).or_insert(0) += 1;
        }
        let mut vals: Vec<_> = freqs.values().copied().collect();
        vals.sort();
        HandType::from_card_frequencies(vals)
    }

    fn classify_q2(&self) -> HandType {
        let mut freqs = HashMap::new();
        let mut jokers = 0;
        for ch in self.0 {
            if ch == Card::J {
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
                for ch in Card::list_q2() {
                    if let Some(x) = freqs.get(&ch) {
                        freqs.insert(ch, x + jokers);
                        break;
                    }
                }
            }
        }
        let mut vals: Vec<_> = freqs.values().copied().collect();
        vals.sort();
        HandType::from_card_frequencies(vals)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Q {
    Q1,
    Q2,
}

#[derive(Clone, Debug)]
struct RowForQuestion<'a> {
    row: &'a Row,
    question: Q,
}

impl<'a> PartialEq for RowForQuestion<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.row.hand == other.row.hand
    }
}
impl<'a> Eq for RowForQuestion<'a> {}

impl<'a> RowForQuestion<'a> {
    fn hand_type(&self) -> HandType {
        match self.question {
            Q::Q1 => self.row.hand.classify_q1(),
            Q::Q2 => self.row.hand.classify_q2(),
        }
    }
    fn card_values(&self) -> Vec<u32> {
        self.row
            .hand
            .0
            .iter()
            .map(|card| match self.question {
                Q::Q1 => card.eval_q1(),
                Q::Q2 => card.eval_q2(),
            })
            .collect()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Q2<'a>(&'a Hand);

impl<'a> PartialOrd for RowForQuestion<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for RowForQuestion<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.card_values().cmp(&other.card_values()))
    }
}

struct Data {
    rows: Vec<Row>,
}

fn total_scores(data: &Data, question: Q) -> usize {
    // Sort all rows, highest-ranked hands first.
    let mut rows: Vec<_> = data
        .rows
        .iter()
        .map(|row| RowForQuestion { row, question })
        .collect();
    rows.sort();
    // Calculate the total score by summing each row's bid * rank.
    let n = rows.len();
    rows.iter()
        .enumerate()
        .map(|(i, row)| (n - i) * row.row.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let example = include_str!("../example.txt");
        let data = Data::parse(example);
        let actual = total_scores(&data, Q::Q1);
        let expected = 6440;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_q2() {
        let example = include_str!("../example.txt");
        let data = Data::parse(example);
        let actual = total_scores(&data, Q::Q2);
        let expected = 5905;
        assert_eq!(actual, expected);
    }

    #[test]
    fn hand_types() {
        for (row, expected) in [
            ("KK677", HandType::TwoPair),
            ("AAAAA", HandType::FiveOfAKind),
            ("AA8AA", HandType::FourOfAKind),
            ("23332", HandType::FullHouse),
            ("TTT98", HandType::ThreeOfAKind),
            ("23432", HandType::TwoPair),
            ("A23A4", HandType::OnePair),
            ("23456", HandType::HighCard),
            ("32T3K", HandType::OnePair),
        ] {
            assert_eq!(
                Hand(
                    row.chars()
                        .map(Card::parse)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                )
                .classify_q1(),
                expected,
            );
        }
        assert_eq!(
            Hand([
                Card::T,
                Card::Number(5),
                Card::Number(5),
                Card::J,
                Card::Number(5),
            ])
            .classify_q2(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand([Card::Q, Card::Q, Card::Q, Card::J, Card::A,]).classify_q2(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand([Card::K, Card::T, Card::J, Card::J, Card::T,]).classify_q2(),
            HandType::FourOfAKind
        );
    }
}
