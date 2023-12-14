fn main() {
    let input = include_str!("../input.txt");
    let mut grid = Grid::parse(input);
    grid.tilt_north();
    let a1 = grid.total_load();
    grid.print();
    println!("Q1: {a1}");
}

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

    /// Tilt column number `x` (0-indexed) to the north.
    fn tilt_column_north(&mut self, x: usize) {
        for y in 1..self.height {
            if self.at(Point { x, y }) == Tile::RoundRock {
                let mut curr = Point { x, y };
                for i in 1..=y {
                    let above_curr = Point { x, y: y - i };
                    if self.at(above_curr) == Tile::Empty {
                        self.set(above_curr, Tile::RoundRock);
                        self.set(curr, Tile::Empty);
                        curr = above_curr;
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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
}
