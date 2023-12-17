use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::parse(input);
    let a1 = grid.energized((Point::default(), Dir::Right)).len();
    assert_eq!(a1, 7434);
    println!("Q1: {a1}");
    let a2 = grid.max_energy();
    println!("Q2: {a2}");
}

#[derive(Clone, Copy)]
enum Tile {
    /// .
    Empty,
    /// /
    MirrorForwards,
    /// \
    MirrorBackwards,
    /// |
    SplitUpDown,
    /// -
    SplitLeftRight,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let tiles: Vec<_> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn at(&self, Point { x, y }: Point) -> Option<Tile> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some(self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    fn max_energy(&self) -> usize {
        let mut starts = HashSet::new();
        starts.extend((0..self.width).map(|x| (Point { x: x as i32, y: 0 }, Dir::Down)));
        starts.extend((0..self.width).map(|x| {
            (
                Point {
                    x: x as i32,
                    y: self.height as i32 - 1,
                },
                Dir::Up,
            )
        }));
        starts.extend((0..self.height).map(|y| (Point { x: 0, y: y as i32 }, Dir::Right)));
        starts.extend((0..self.height).map(|y| {
            (
                Point {
                    x: self.width as i32 - 1,
                    y: y as i32,
                },
                Dir::Left,
            )
        }));
        starts
            .into_iter()
            .map(|start| self.energized(start).len())
            .max()
            .unwrap()
    }

    fn energized(&self, start: (Point, Dir)) -> HashSet<Point> {
        let mut visited = HashSet::new();
        let mut paths = HashSet::from([start]);
        while !paths.is_empty() {
            for (position, dir) in std::mem::take(&mut paths) {
                visited.insert((position, dir));
                let next_position = position.move_along(dir);
                let new_paths = match (self.at(next_position), dir) {
                    // Continue through empty space.
                    (Some(Tile::Empty), dir) => vec![(next_position, dir)],
                    // Mirrors change your direction.
                    (Some(Tile::MirrorForwards), Dir::Up) => vec![(next_position, Dir::Right)],
                    (Some(Tile::MirrorForwards), Dir::Down) => vec![(next_position, Dir::Left)],
                    (Some(Tile::MirrorForwards), Dir::Left) => vec![(next_position, Dir::Down)],
                    (Some(Tile::MirrorForwards), Dir::Right) => vec![(next_position, Dir::Up)],
                    (Some(Tile::MirrorBackwards), Dir::Up) => vec![(next_position, Dir::Left)],
                    (Some(Tile::MirrorBackwards), Dir::Down) => vec![(next_position, Dir::Right)],
                    (Some(Tile::MirrorBackwards), Dir::Left) => vec![(next_position, Dir::Up)],
                    (Some(Tile::MirrorBackwards), Dir::Right) => vec![(next_position, Dir::Down)],
                    // Splitters make two paths if you hit them head-on,
                    // or you pass through if you hit them the other way.
                    (Some(Tile::SplitUpDown), d @ (Dir::Up | Dir::Down)) => {
                        vec![(next_position, d)]
                    }
                    (Some(Tile::SplitUpDown), _) => {
                        vec![(next_position, Dir::Up), (next_position, Dir::Down)]
                    }
                    (Some(Tile::SplitLeftRight), d @ (Dir::Left | Dir::Right)) => {
                        vec![(next_position, d)]
                    }
                    (Some(Tile::SplitLeftRight), _) => {
                        vec![(next_position, Dir::Left), (next_position, Dir::Right)]
                    }
                    // Out of bounds, path ends.
                    (None, _) => vec![],
                }
                .into_iter()
                .filter(|posdir| !visited.contains(posdir));
                paths.extend(new_paths);
            }
        }
        visited.into_iter().map(|(point, _dir)| point).collect()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
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
    fn move_along(self, dir: Dir) -> Self {
        let Self { x, y } = self;
        match dir {
            Dir::Up => Self { x, y: y - 1 },
            Dir::Down => Self { x, y: y + 1 },
            Dir::Left => Self { x: x - 1, y },
            Dir::Right => Self { x: x + 1, y },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::MirrorForwards,
            '\\' => Self::MirrorBackwards,
            '|' => Self::SplitUpDown,
            '-' => Self::SplitLeftRight,
            other => panic!("unrecognized char {other}"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::MirrorForwards => '/',
            Tile::MirrorBackwards => '\\',
            Tile::SplitUpDown => '|',
            Tile::SplitLeftRight => '-',
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        char::from(*self).fmt(f)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows: Vec<_> = self
            .tiles
            .iter()
            .map(|row| row.iter().copied().map(char::from).collect::<String>())
            .collect();
        write!(f, "{}", rows.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let g = Grid::parse(include_str!("../example.txt"));
        println!("{g}");
        assert_eq!(g.width, 10);
        assert_eq!(g.height, 10);
    }
    #[test]
    fn test_q1() {
        let g = Grid::parse(include_str!("../example.txt"));
        let actual = g.energized((Point::default(), Dir::Right)).len();
        assert_eq!(actual, 46);
    }
    #[test]
    fn test_q2() {
        let g = Grid::parse(include_str!("../example.txt"));
        let actual = g.max_energy();
        assert_eq!(actual, 51);
    }
}
