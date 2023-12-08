use super::{Card, Data, Hand, Row};

impl Card {
    pub fn parse(c: char) -> Self {
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

impl Data {
    pub fn parse(s: &str) -> Data {
        let rows = s.lines().map(Row::parse).collect();
        Data { rows }
    }
}
