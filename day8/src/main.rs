use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let data = Data::parse(input);
    println!("Q1: {}", q1(&data));
    println!("Q2: {}", q2(&data));
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        if value == 'R' {
            Self::Right
        } else if value == 'L' {
            Self::Left
        } else {
            panic!("Invalid direction {value}");
        }
    }
}

type Node = String;

/// A map of the desert, given as problem input.
#[derive(Debug)]
struct Data {
    directions: Vec<Direction>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Data {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let directions: Vec<_> = lines.next().unwrap().chars().map(Direction::from).collect();
        lines.next().unwrap();
        let nodes: HashMap<_, _> = lines
            .map(|line| {
                let (key, val) = line.split_once(" = ").unwrap();
                let val_without_parens = &val[1..val.len() - 1];
                let (l, r) = val_without_parens.split_once(", ").unwrap();
                (key.to_owned(), (l.to_owned(), r.to_owned()))
            })
            .collect();
        Self { directions, nodes }
    }
}

fn q1(data: &Data) -> usize {
    let mut curr = "AAA".to_owned();
    for (dir, steps) in data.directions.iter().cycle().zip(0..) {
        if curr == "ZZZ" {
            return steps;
        }
        let (to_left, to_right) = data.nodes.get(&curr).unwrap();
        curr = match dir {
            Direction::Right => to_right.to_owned(),
            Direction::Left => to_left.to_owned(),
        };
    }
    unreachable!()
}

fn q2(data: &Data) -> usize {
    let starting_nodes = data
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned();
    let cycle_lengths = starting_nodes.map(|mut curr| {
        for (dir, steps) in data.directions.iter().cycle().zip(0usize..) {
            if curr.ends_with('Z') {
                return steps;
            }
            let (to_left, to_right) = data.nodes.get(&curr).unwrap();
            curr = match dir {
                Direction::Right => to_right.to_owned(),
                Direction::Left => to_left.to_owned(),
            };
        }
        unreachable!()
    });
    lowest_common_multiple(cycle_lengths)
}

fn lowest_common_multiple(numbers: impl Iterator<Item = usize>) -> usize {
    fn lcm(a: usize, b: usize) -> usize {
        a * b / gcd(a, b)
    }
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            (a, b) = (b, a % b)
        }
        a
    }
    numbers.reduce(lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input = include_str!("../example.txt");
        let data = Data::parse(input);
        let actual = q1(&data);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_q2() {
        let input = include_str!("../example2.txt");
        let data = Data::parse(input);
        let actual = q2(&data);
        let expected = 6;
        assert_eq!(actual, expected);
    }
}
