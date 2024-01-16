use std::collections::HashSet;

fn main() {
    let input_file = include_str!("../input.txt");
    let input: Vec<InputRow> = input_file.lines().map(InputRow::parse).collect();
    let trench = Trench::dig_from(input);
    let trench_size = trench.edge.len();
    trench.visualize();
    println!("initial trench size is {trench_size}");
    println!("it's {} by {}", trench.width, trench.height);
    let filled = flood(
        Point { x: 1, y: 1 },
        trench.edge.into_iter().collect(),
        trench.width,
        trench.height,
    );
    println!("Total size once filled: {}", filled.len() + trench_size);
}

struct Trench {
    edge: Vec<Point>,
    width: u16,
    height: u16,
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

fn flood(start: Point, mut seen: HashSet<Point>, width: u16, height: u16) -> Vec<Point> {
    let mut discovered = HashSet::new();
    let mut fringe = vec![start];

    // Generate a list of points adjacent to the given point,
    // if they're within the grid bounds.
    let neighbours = |p: Point| {
        let out = [
            Point { x: p.x + 1, ..p },
            Point { x: p.x - 1, ..p },
            Point { y: p.y + 1, ..p },
            Point { y: p.y - 1, ..p },
        ];
        out.into_iter()
            .filter(|p| p.x >= 0 && p.x < width as i32 && p.y >= 0 && p.y < height as i32)
    };

    while let Some(curr) = fringe.pop() {
        for neighbour in neighbours(curr).filter(|p| !seen.contains(p)) {
            fringe.push(neighbour);
        }
        seen.insert(curr);
        discovered.insert(curr);
    }

    discovered.into_iter().collect()
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
    fn points_along(self, dir: Dir, n: u16) -> impl Iterator<Item = Point> {
        (0..n).map(move |i| self.advance(dir, i.into()))
    }
}

impl Trench {
    fn dig_from(instructions: Vec<InputRow>) -> Self {
        let (mut edge, _curr) = instructions.into_iter().fold(
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
        let height = (max_y - min_y + 1) as u16;
        let width = (max_x - min_x + 1) as u16;
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
}

struct InputRow {
    dir: Dir,
    metres: u16,
    #[allow(dead_code)] // Part 2 I assume
    color: String,
}

impl InputRow {
    fn parse(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        let [dir, metres, color]: [&str; 3] = parts.try_into().unwrap();
        Self {
            dir: Dir::parse(dir),
            metres: metres.parse().unwrap(),
            color: color.to_owned(),
        }
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
        let input: Vec<InputRow> = input_file.lines().map(InputRow::parse).collect();
        let trench = Trench::dig_from(input);
        trench.visualize();
        println!();
        assert_eq!(trench.width, 7, "width is wrong");
        assert_eq!(trench.height, 10, "height is wrong");
        assert_eq!(trench.edge.len(), 38);
        let start = Point { x: 1, y: 1 };
        let inside_trench = flood(
            start,
            trench.edge.iter().copied().collect(),
            trench.width,
            trench.height,
        );
        let mut x = trench.edge;
        x.extend(inside_trench);
        let filled_trench = Trench {
            edge: x,
            width: trench.width,
            height: trench.height,
        };
        filled_trench.visualize();
        println!();
        assert_eq!(filled_trench.edge.len(), 62);
    }
}