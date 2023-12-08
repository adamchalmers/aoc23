use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let data = parse(include_str!("../input.txt"));
    // println!("Q1: {}", q1(&data));
    println!("Q2: {}", q2(&data));
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
    fn parse(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'T' => Self::T,
            'J' => Self::J,
            x => Self::Number(x.to_digit(10).unwrap()),
        }
    }
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
            Card::T => 4,
            Card::J => 5,
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

impl Hand {
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
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            dbg!(self.classify_q2().cmp(&other.classify_q2())).then_with(|| {
                dbg!(self
                    .0
                    .iter()
                    .map(Card::eval_q2)
                    .cmp(other.0.iter().map(Card::eval_q2)))
            }),
        )
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
        let hand = Hand(
            hand.chars()
                .map(Card::parse)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
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
    rows.sort_by(|this, other| this.hand.cmp(&other.hand));
    let mut prod = 0;
    let n = rows.len();
    for (i, row) in rows.iter().enumerate() {
        let rank = n - i;
        eprintln!("{rank}: {row:?}");
        prod += rank * row.bid;
    }
    prod
}

fn q2(data: &Data) -> usize {
    total_scores(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_name() {
    //     let example = include_str!("../example.txt");
    //     let data = parse(example);
    //     let actual = q1(&data);
    //     let expected = 5905;
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn test_q2() {
        let example = include_str!("../example.txt");
        let data = parse(example);
        let actual = q2(&data);
        let expected = 5905;
        assert_eq!(actual, expected);
    }

    #[test]
    fn hand_types() {
        // assert_eq!(Hand(['A'; 5]).classify(), HandType::FiveOfAKind);
        // assert_eq!(
        //     Hand(['A', 'A', '8', 'A', 'A']).classify(),
        //     HandType::FourOfAKind
        // );
        // assert_eq!(
        //     Hand(['2', '3', '3', '3', '2']).classify(),
        //     HandType::FullHouse
        // );
        // assert_eq!(
        //     Hand(['T', 'T', 'T', '9', '8']).classify(),
        //     HandType::ThreeOfAKind
        // );

        // assert_eq!(
        //     Hand(['2', '3', '4', '3', '2']).classify(),
        //     HandType::TwoPair
        // );

        // assert_eq!(
        //     Hand(['A', '2', '3', 'A', '4']).classify(),
        //     HandType::OnePair
        // );
        // assert_eq!(
        //     Hand(['2', '3', '4', '5', '6']).classify(),
        //     HandType::HighCard
        // );
        // assert_eq!(
        //     Hand(['3', '2', 'T', '3', 'K']).classify(),
        //     HandType::OnePair
        // );
        // assert_eq!(
        //     Hand(['K', 'K', '6', '7', '7']).classify(),
        //     HandType::TwoPair
        // );
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

    #[test]
    fn ordering() {
        let h1 = Hand([
            Card::Number(3),
            Card::Number(3),
            Card::Number(3),
            Card::Number(3),
            Card::Number(2),
        ]);
        let h2 = Hand([Card::Number(2), Card::A, Card::A, Card::A, Card::A]);
        assert_eq!(h1.classify_q2(), HandType::FourOfAKind);
        assert_eq!(h2.classify_q2(), HandType::FourOfAKind);
        assert!(h1 < h2);
    }
}
