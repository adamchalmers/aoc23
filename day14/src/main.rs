use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::parse(input);
    // Q1
    let mut grid1 = grid.clone();
    grid1.tilt_north();
    let a1 = grid1.total_load();
    println!("Q1: {a1}");
    // Q2
    let mut grid2 = grid;
    grid2.spin(1_000_000_000);
    let a2 = grid2.total_load();
    println!("Q2: {a2}");
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Tile::parse).collect())
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn at(&self, p: Point) -> Tile {
        self.tiles[p.y][p.x]
    }
    fn set(&mut self, p: Point, tile: Tile) {
        self.tiles[p.y][p.x] = tile;
    }

    fn tilt_column_north(&mut self, x: usize) {
        for y in 1..self.height {
            if self.at(Point { x, y }) == Tile::RoundRock {
                let mut curr = Point { x, y };
                for i in 1..=y {
                    let next = Point { x, y: y - i };
                    if self.at(next) == Tile::Empty {
                        self.set(next, Tile::RoundRock);
                        self.set(curr, Tile::Empty);
                        curr = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_column_south(&mut self, x: usize) {
        for y in (0..self.height - 1).rev() {
            if self.at(Point { x, y }) == Tile::RoundRock {
                let mut curr = Point { x, y };
                for i in 1..(self.height - y) {
                    let next = Point { x, y: y + i };
                    if self.at(next) == Tile::Empty {
                        self.set(next, Tile::RoundRock);
                        self.set(curr, Tile::Empty);
                        curr = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_column_west(&mut self, y: usize) {
        for x in 1..self.width {
            if self.at(Point { x, y }) == Tile::RoundRock {
                let mut curr = Point { x, y };
                for i in 1..=x {
                    let next = Point { x: x - i, y };
                    if self.at(next) == Tile::Empty {
                        self.set(next, Tile::RoundRock);
                        self.set(curr, Tile::Empty);
                        curr = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_column_east(&mut self, y: usize) {
        for x in (0..self.width - 1).rev() {
            if self.at(Point { x, y }) == Tile::RoundRock {
                let mut curr = Point { x, y };
                for i in 1..(self.width - x) {
                    let next = Point { x: x + i, y };
                    if self.at(next) == Tile::Empty {
                        self.set(next, Tile::RoundRock);
                        self.set(curr, Tile::Empty);
                        curr = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        for x in 0..self.width {
            self.tilt_column_north(x);
        }
    }
    fn tilt_south(&mut self) {
        for x in 0..self.width {
            self.tilt_column_south(x);
        }
    }
    fn tilt_west(&mut self) {
        for y in 0..self.height {
            self.tilt_column_west(y);
        }
    }
    fn tilt_east(&mut self) {
        for y in 0..self.height {
            self.tilt_column_east(y);
        }
    }

    /// Return where the pattern starts repeating, and what the period of repetition is.
    fn calculate_period(mut self, n: usize) -> (usize, usize) {
        let mut seen: HashMap<Vec<Vec<Tile>>, usize> = HashMap::default();
        for i in 0..n {
            if let Some(first) = seen.get(&self.tiles) {
                let period = i - first;
                return (*first, period);
            }
            seen.insert(self.tiles.clone(), i);
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
        panic!("Never converges");
    }

    fn spin(&mut self, n: usize) {
        let (cycle_start, period) = dbg!(self.clone().calculate_period(n));
        let number_of_cycles = cycle_start + ((n - cycle_start) % period);
        for _ in 0..number_of_cycles {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            let row: Vec<_> = self.tiles[y].iter().map(ToString::to_string).collect();
            println!("{}", row.join(""));
        }
    }

    fn total_load(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.width {
            for x in 0..self.height {
                if self.at(Point { x, y }) == Tile::RoundRock {
                    sum += self.height - y;
                }
            }
        }
        sum
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum Tile {
    Empty,
    RoundRock,
    CubeRock,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::RoundRock => 'O',
                Tile::CubeRock => '#',
            }
        )
    }
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '#' => Self::CubeRock,
            'O' => Self::RoundRock,
            '.' => Self::Empty,
            other => panic!("unexpected tile {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input = include_str!("../example.txt");
        let mut grid = Grid::parse(input);
        grid.tilt_north();
        grid.print();
        use Tile::*;
        let expected_after_tilt = vec![
            vec![
                RoundRock, RoundRock, RoundRock, RoundRock, Empty, CubeRock, Empty, RoundRock,
                Empty, Empty,
            ],
            vec![
                RoundRock, RoundRock, Empty, Empty, CubeRock, Empty, Empty, Empty, Empty, CubeRock,
            ],
            vec![
                RoundRock, RoundRock, Empty, Empty, RoundRock, CubeRock, CubeRock, Empty, Empty,
                RoundRock,
            ],
            vec![
                RoundRock, Empty, Empty, CubeRock, Empty, RoundRock, RoundRock, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, CubeRock, Empty,
            ],
            vec![
                Empty, Empty, CubeRock, Empty, Empty, Empty, Empty, CubeRock, Empty, CubeRock,
            ],
            vec![
                Empty, Empty, RoundRock, Empty, Empty, CubeRock, Empty, RoundRock, Empty, RoundRock,
            ],
            vec![
                Empty, Empty, RoundRock, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                CubeRock, Empty, Empty, Empty, Empty, CubeRock, CubeRock, CubeRock, Empty, Empty,
            ],
            vec![
                CubeRock, Empty, Empty, Empty, Empty, CubeRock, Empty, Empty, Empty, Empty,
            ],
        ];
        assert_eq!(grid.tiles, expected_after_tilt);
        let actual_load = grid.total_load();
        assert_eq!(actual_load, 136);
    }
    #[test]
    fn test_q2() {
        let input = include_str!("../example.txt");
        let mut grid = Grid::parse(input);
        grid.spin(1_000_000_000);
        grid.print();
        assert_eq!(grid.total_load(), 64);
    }
}
