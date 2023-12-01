use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    // This can be trivially parallelized!
    // Split up the parsing work on each thread.
    let answer1: u32 = input.lines().par_bridge().map(parser_1).sum();
    eprintln!("{answer1}");
    let answer2: u32 = input.lines().par_bridge().map(parser_2).sum();
    eprintln!("{answer2}");
}

fn parser_1(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|ch| ch.to_digit(10));
    let first = digits.next().unwrap();
    first * 10 + digits.last().unwrap_or(first)
}

const NUMERALS: [(&str, u32); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn parser_2(line: &str) -> u32 {
    let mut digits = (0..line.len()).filter_map(|i| {
        NUMERALS
            .iter()
            .find(|(numeral, _value)| line[i..].starts_with(numeral))
            .map(|(_numeral, value)| value)
    });
    let first = digits.next().unwrap();
    first * 10 + digits.last().unwrap_or(first)
}
