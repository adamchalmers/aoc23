#![feature(array_windows)]
use rustc_hash::FxHashSet as HashSet;

fn main() {
    let input_file = include_str!("../input.txt");
    let (input, input_hex): (Vec<_>, Vec<_>) = input_file.lines().map(Instruction::parse).unzip();
    let q1 = size_of_trench(&input);
    assert_eq!(34329, q1);
    let q2 = size_of_trench(&input_hex);
    assert_eq!(q2, 42617947302920);
}

fn size_of_trench(input: &[Instruction]) -> usize {
    let trench = Trench::dig_from(input);
    let trench_size = trench.edge.len();
    let filled = trench.count_inside();
    let size = filled + trench_size;
    println!("Total trench size: {size}");
    size
}

struct Trench {
    edge: Vec<Point>,
    width: u32,
    height: u32,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    /// Find the point one unit along the given direction.
    fn advance(&self, dir: Dir, len: i32) -> Point {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y - len,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + len,
            },
            Dir::Left => Self {
                y: self.y,
                x: self.x - len,
            },
            Dir::Right => Self {
                y: self.y,
                x: self.x + len,
            },
        }
    }

    /// Get all `n` points from `start` advancing one meter at a time in the given direction.
    fn points_along(self, dir: Dir, n: u32) -> impl Iterator<Item = Point> {
        (0..n).map(move |i| self.advance(dir, i.try_into().unwrap()))
    }
}

impl Trench {
    fn dig_from(instructions: &[Instruction]) -> Self {
        let (mut edge, _curr) =
            instructions.iter().fold(
                (Vec::new(), Point::default()),
                |(mut edge, mut curr), instr| {
                    edge.extend(curr.points_along(instr.dir, instr.metres));
                    curr = edge.last().copied().unwrap().advance(instr.dir, 1);
                    (edge, curr)
                },
            );
        let (min_x, max_x) = edge
            .iter()
            .skip(1)
            .fold((edge[0].x, edge[0].x), |(min, max), p| {
                (min.min(p.x), max.max(p.x))
            });
        let (min_y, max_y) = edge
            .iter()
            .skip(1)
            .fold((edge[0].y, edge[0].y), |(min, max), p| {
                (min.min(p.y), max.max(p.y))
            });
        let height = (max_y - min_y + 1) as u32;
        let width = (max_x - min_x + 1) as u32;
        let above = if min_y < 0 { -min_y } else { 0 };
        let left = if min_x < 0 { -min_x } else { 0 };
        for p in &mut edge {
            p.x += left;
            p.y += above;
        }

        Self {
            edge,
            height,
            width,
        }
    }

    /// Print a visual of the maze to stdout.
    #[allow(dead_code)]
    fn visualize(&self) {
        for y in 0..self.height {
            let row = (0..self.width).map(|x| {
                if self.edge.contains(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    '#'
                } else {
                    '.'
                }
            });
            println!("{}", row.collect::<String>())
        }
    }

    fn count_inside(self) -> usize {
        let mut points_per_row = vec![vec![]; self.height as usize];
        for point in &self.edge {
            points_per_row[point.y as usize].push(point.x);
        }
        let edge: HashSet<_> = self.edge.into_iter().collect();
        let mut inside = 0usize;
        for (y, mut xs) in points_per_row.into_iter().enumerate() {
            // println!("Starting row {y}");
            xs.sort();
            let spans = xs.into_iter().fold(Vec::new(), |mut spans, x| {
                match spans.pop() {
                    Some((start, end)) => {
                        if x - end == 1 {
                            spans.push((start, x));
                        } else {
                            spans.push((start, end));
                            spans.push((x, x));
                        }
                    }
                    None => {
                        spans.push((x, x));
                    }
                }
                spans
            });
            let mut this_row = 0usize;
            let mut outside = false;
            for [span_l, span_r] in spans.as_slice().array_windows() {
                // println!("\t{span_l:?} - {span_r:?}");
                if span_l.1 == span_l.0
                    // Is left edge an S 
                    || (edge.contains(&Point {
                        x: span_l.0,
                        y: y as i32 + 1,
                    }) && edge.contains(&Point {
                        x: span_l.1,
                        y: y as i32 - 1,
                    }))
                    || (edge.contains(&Point {
                        x: span_l.0,
                        y: y as i32 - 1,
                    }) && edge.contains(&Point {
                        x: span_l.1,
                        y: y as i32 + 1,
                    }))
                {
                    outside = !outside;
                }
                if outside {
                    let n = usize::try_from(span_r.0 - span_l.1 - 1).unwrap();
                    // println!("\t Adding {n}");
                    this_row += n;
                }
            }
            inside += this_row;
            // println!("Row {y} has {this_row} m3 inside");
        }
        inside
    }
}

struct Instruction {
    dir: Dir,
    metres: u32,
}

impl Instruction {
    fn parse_hex(s: &str) -> Self {
        let metres = u32::from_str_radix(&s[0..5], 16).unwrap();
        let dir = match &s[5..] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            other => panic!("Invalid hexadecimal direction '{other}'"),
        };
        Self { dir, metres }
    }
}

impl Instruction {
    fn parse(s: &str) -> (Self, Self) {
        let parts = s.split(' ').collect::<Vec<_>>();
        let [dir, metres, color]: [&str; 3] = parts.try_into().unwrap();

        (
            Self {
                dir: Dir::parse(dir),
                metres: metres.parse().unwrap(),
            },
            Self::parse_hex(&color[2..8]),
        )
    }
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn parse(s: &str) -> Self {
        match s.chars().next().unwrap() {
            'U' => Self::Up,
            'D' => Self::Down,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input_file = include_str!("../example.txt");
        let input: (Vec<_>, Vec<_>) = input_file.lines().map(Instruction::parse).unzip();
        let trench = Trench::dig_from(&input.0);
        trench.visualize();
        println!();
        assert_eq!(trench.width, 7, "width is wrong");
        assert_eq!(trench.height, 10, "height is wrong");
        assert_eq!(trench.edge.len(), 38);
        let inside_trench = trench.count_inside();
        assert_eq!(inside_trench, 62 - 38);
    }
}
