use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let diagram = Diagram::parse(include_str!("../input.txt"));
    eprintln!("Q1: {}", diagram.q1());
    eprintln!("Q2: {}", diagram.q2());
}

struct Diagram {
    symbols: HashMap<Point, char>,
    part_numbers: Vec<(Vec<Point>, usize)>,
}

impl Diagram {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        // Find all the diagram's symbols.
        let mut symbols = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' && !char.is_digit(10) {
                    symbols.insert(Point { x, y }, char);
                }
            }
        }

        // Find all the numbers in the diagram, and their indices.
        let mut digits: Vec<usize> = Vec::new();
        let mut points_for_number = Vec::new();
        let mut numbers: Vec<(Vec<_>, usize)> = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if let Some(d) = char.to_digit(10) {
                    digits.push(d as usize);
                    points_for_number.push(Point { x, y });
                } else {
                    if !digits.is_empty() {
                        let mut sum = 0usize;
                        for (i, d) in digits.drain(0..).rev().enumerate() {
                            sum += d * 10usize.pow(i as u32) as usize;
                        }
                        numbers.push((points_for_number.drain(0..).collect(), sum));
                    }
                }
            }
        }

        let part_numbers = numbers
            .into_iter()
            .filter(|(points, _number)| {
                points.iter().any(|p0| {
                    for p1 in symbols.keys() {
                        if p0.is_adjacent(*p1) {
                            return true;
                        }
                    }
                    false
                })
            })
            .collect();

        Diagram {
            symbols,
            part_numbers,
        }
    }

    fn q1(&self) -> usize {
        self.part_numbers
            .iter()
            .map(|(_points, number)| number)
            .sum()
    }

    fn q2(&self) -> usize {
        let mut answer = 0;
        for (p, symbol) in &self.symbols {
            if symbol != &'*' {
                continue;
            }
            let adjacent_to_this_gear: Vec<_> = self
                .part_numbers
                .iter()
                .filter_map(|(points, value)| {
                    // Is the number adjacent?
                    if points.iter().any(|p0| p0.is_adjacent(*p)) {
                        Some(*value)
                    } else {
                        None
                    }
                })
                .collect();

            let res: Result<[usize; 2], _> = adjacent_to_this_gear.try_into();
            if let Ok([x, y]) = res {
                answer += x * y;
            }
        }
        answer
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_adjacent(self, other: Self) -> bool {
        ((self.x as isize) - (other.x as isize)).abs() <= 1
            && ((self.y as isize) - (other.y as isize)).abs() <= 1
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:3}, {:3})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let d = Diagram::parse(include_str!("../example.txt"));
        for (point, symbol) in &d.symbols {
            eprintln!("{point:?}: {symbol}");
        }
        for (point, number) in &d.part_numbers {
            eprintln!("{number}");
            eprintln!("{point:?}");
        }
        assert_eq!(d.q1(), 4361);
        assert_eq!(d.q2(), 467835);
    }
}
