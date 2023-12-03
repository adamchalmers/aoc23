fn main() {
    let diagram = Diagram::parse(include_str!("../input.txt"));
    println!("Q1: {}", diagram.q1());
    println!("Q2: {}", diagram.q2());
}

struct Diagram {
    symbols: Vec<Symbol>,
    part_numbers: Vec<PartNumber>,
}

struct Symbol {
    point: Point,
    symbol: char,
}

struct PartNumber {
    points: Vec<Point>,
    value: u32,
}

impl Diagram {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        // Find all the diagram's symbols and the point they're located at.
        let mut symbols = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    symbols.push(Symbol {
                        point: Point { x, y },
                        symbol: char,
                    });
                }
            }
        }

        // Find all the numbers in the diagram, and the points they span across.
        let mut digits: Vec<u32> = Vec::new(); // Each digit in the number.
        let mut points_for_number = Vec::new(); // Each point the number spans.
        let mut part_numbers = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if let Some(d) = char.to_digit(10) {
                    digits.push(d);
                    points_for_number.push(Point { x, y });
                } else if !digits.is_empty() {
                    // The number is finished, check if it's a part number.
                    let points = std::mem::take(&mut points_for_number);
                    let mut digits = std::mem::take(&mut digits);

                    // A number is a part number if it's adjacent to a symbol.
                    if points
                        .iter()
                        .any(|p| symbols.iter().any(|symbol| symbol.point.is_adjacent(*p)))
                    {
                        // Calculate the actual number by aggregating the digits.
                        let value = digits
                            .drain(0..)
                            .rev()
                            .enumerate()
                            .map(|(i, d)| d * 10u32.pow(u32::try_from(i).unwrap()))
                            .sum();
                        part_numbers.push(PartNumber { points, value });
                    }
                }
            }
        }

        Diagram {
            symbols,
            part_numbers,
        }
    }

    // What is the sum of all of the part numbers in the engine schematic?
    fn q1(&self) -> u32 {
        self.part_numbers
            .iter()
            .map(|part_num| part_num.value)
            .sum()
    }

    // What is the sum of all of the gear ratios in your engine schematic?
    fn q2(&self) -> u32 {
        self.symbols
            .iter()
            .map(|Symbol { point, symbol }| {
                if symbol != &'*' {
                    return 0;
                }
                let adjacent_to_this_gear: Vec<_> = self
                    .part_numbers
                    .iter()
                    .filter_map(|PartNumber { points, value }| {
                        // Is the number adjacent?
                        if points.iter().any(|p0| p0.is_adjacent(*point)) {
                            Some(*value)
                        } else {
                            None
                        }
                    })
                    .collect();

                // A gear is any * symbol that is adjacent to exactly two part numbers.
                if adjacent_to_this_gear.len() != 2 {
                    return 0;
                }

                // Its gear ratio is the result of multiplying those two numbers together.
                adjacent_to_this_gear.into_iter().product()
            })
            .sum()
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_adjacent(self, other: Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
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
    fn test_example() {
        let d = Diagram::parse(include_str!("../example.txt"));
        assert_eq!(d.q1(), 4361);
        assert_eq!(d.q2(), 467835);
    }
}
