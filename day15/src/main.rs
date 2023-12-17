#![feature(ascii_char, ascii_char_variants)]

type Number = u128;

fn main() {
    let input = include_str!("../input.txt");
    let parsed = parse(input);
    let q1: Number = parsed.iter().map(|step| step.box_number).sum();
    println!("Q1: {q1}");
}

enum Operation {
    PutLens { focal_length: Number },
    TakeLens,
}

struct Step<'a> {
    label: &'a str,
    op: Operation,
    box_number: Number,
}

fn parse(s: &str) -> Vec<Step<'_>> {
    s.split(',')
        .map(|line| {
            let box_number = hash(line);
            let (label, op) = if line.contains('-') {
                (&line[..line.len() - 1], Operation::TakeLens)
            } else {
                let (label, fl) = line.split_once('=').unwrap();
                let focal_length = fl.parse().unwrap();
                (label, Operation::PutLens { focal_length })
            };
            Step {
                box_number,
                label,
                op,
            }
        })
        .collect()
}

fn hash(s: &str) -> Number {
    let mut curr = 0;
    for ch in s.chars() {
        /*
           Determine the ASCII code for the current character of the string.
           Increase the current value by the ASCII code you just determined.
           Set the current value to itself multiplied by 17.
           Set the current value to the remainder of dividing itself by 256.
        */
        let ascii_char = ch.as_ascii().unwrap();
        if ascii_char == std::ascii::Char::LineFeed {
            continue;
        }
        curr += (ascii_char as u8) as Number;
        curr *= 17;
        curr %= 256;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_q1() {
        let input = include_str!("../example.txt");
        let hashes: Vec<_> = parse(input).iter().map(|step| step.box_number).collect();
        let expected = vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        assert_eq!(hashes, expected);
        let q1: Number = hashes.into_iter().sum();
        assert_eq!(q1, 1320);
    }
}
