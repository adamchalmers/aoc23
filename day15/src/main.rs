#![feature(ascii_char, ascii_char_variants)]

fn main() {
    let input = include_str!("../input.txt");
    let parsed = parse(input);
    let a1: usize = parsed.iter().map(|step| step.hash).sum();
    println!("Q1: {a1}");
    assert_eq!(a1, 516469);
    let boxes = q2(parsed);
    let a2 = focusing_power(&boxes);
    println!("Q2: {a2}");
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    PutLens { focal_length: usize },
    TakeLens,
}

#[derive(Debug, Eq, PartialEq)]
struct Step<'a> {
    label: &'a str,
    op: Operation,
    hash: usize,
    box_number: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

type Boxes<'a> = [Vec<Lens<'a>>; 256];

fn focusing_power(boxes: &Boxes<'_>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, box_of_lenses)| {
            box_of_lenses
                .iter()
                .enumerate()
                // The focusing power of a single lens is the result of multiplying together:
                //   One plus the box number of the lens in question.
                //   The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
                //   The focal length of the lens.
                .map(|(slot_number, lens)| (1 + box_number) * (slot_number + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn q2(steps: Vec<Step<'_>>) -> Boxes<'_> {
    const EMPTY: Vec<Lens<'_>> = Vec::new();
    steps.into_iter().fold([EMPTY; 256], |mut boxes, step| {
        let this_box = &mut boxes[step.box_number];
        match step.op {
            Operation::PutLens { focal_length } => {
                let new_lens = Lens {
                    label: step.label,
                    focal_length,
                };
                if let Some(i) = this_box.iter().position(|lens| lens.label == step.label) {
                    // If there is already a lens in the box with the same label,
                    // replace the old lens with the new lens:
                    // remove the old lens and put the new lens in its place,
                    // not moving any other lenses in the box.
                    this_box[i] = new_lens;
                } else {
                    // If there is not already a lens in the box with the same label,
                    // add the lens to the box immediately behind any lenses already in the box.
                    // Don't move any of the other lenses when you do this.
                    // If there aren't any lenses in the box, the new lens goes all the way to the
                    // front of the box.
                    this_box.push(new_lens);
                }
            }
            Operation::TakeLens => {
                // Go to the relevant box and remove the lens with the given label if it is present
                // in the box. Then, move any remaining lenses as far forward in the box as they can
                // go without changing their order, filling any space made by removing the indicated
                // lens. (If no lens in that box has the given label, nothing happens.)
                if let Some(i) = this_box.iter().position(|lens| lens.label == step.label) {
                    this_box.remove(i);
                }
            }
        }
        boxes
    })
}

fn parse(s: &str) -> Vec<Step<'_>> {
    s.split(',')
        .map(|line| {
            let overall_hash = hash(line);
            let (label, op) = if line.contains('-') {
                (&line[..line.len() - 1], Operation::TakeLens)
            } else {
                let (label, fl) = line.split_once('=').unwrap();
                let focal_length = fl.parse().unwrap();
                (label, Operation::PutLens { focal_length })
            };
            let box_number = hash(label);
            Step {
                box_number,
                label,
                op,
                hash: overall_hash,
            }
        })
        .collect()
}

fn hash(s: &str) -> usize {
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
        curr += (ascii_char as u8) as usize;
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
        let hashes: Vec<_> = parse(input).iter().map(|step| step.hash).collect();
        let expected = vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        assert_eq!(hashes, expected);
        let q1: usize = hashes.into_iter().sum();
        assert_eq!(q1, 1320);
    }

    #[test]
    fn test_q2() {
        let input = include_str!("../example.txt");
        let parsed = parse(input);
        let boxes = q2(parsed);
        assert_eq!(
            boxes[0],
            vec![
                Lens {
                    label: "rn",
                    focal_length: 1
                },
                Lens {
                    label: "cm",
                    focal_length: 2
                }
            ]
        );
        assert_eq!(
            boxes[3],
            vec![
                Lens {
                    label: "ot",
                    focal_length: 7
                },
                Lens {
                    label: "ab",
                    focal_length: 5
                },
                Lens {
                    label: "pc",
                    focal_length: 6
                }
            ]
        );
        let expected = 145;
        let actual = focusing_power(&boxes);
        assert_eq!(actual, expected);
    }
}
