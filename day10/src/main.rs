use std::collections::{HashMap, HashSet};

fn main() {
    let (grid, start) = Grid::parse(include_str!("../input.txt"), Tile::NorthSouth);
    let (a1, this_loop) = grid.loop_containing(start);
    println!("Q1: {a1}");
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    /// Returns the start index.
    fn parse(s: &str, start_is: Tile) -> (Grid, Point) {
        let mut g = Grid(
            s.lines()
                .map(|line| line.chars().map(Tile::parse).collect())
                .collect(),
        );
        let start = find_start(&g);
        g.0[start.y][start.x] = start_is;
        (g, start)
    }
    fn at(&self, Point { x, y }: Point) -> Option<Tile> {
        let row = self.0.get(y)?;
        row.get(x).copied()
    }

    /// Find the set of points that are in the same loop, the loop which contains `start`.
    /// Also return the maximum distance from `start` of any point in the loop.
    fn loop_containing(&self, start: Point) -> (u64, HashMap<Point, Tile>) {
        let mut dist = 0;
        let mut frontier = vec![start];
        // Map each seen node to its distance from the start.
        let mut seen: HashMap<Point, Tile> = HashMap::new();
        while !frontier.is_empty() {
            // Find each neighbour of the frontier, and mark the frontier's distance.
            let mut next_frontier = Vec::new();
            for frontier_node in frontier.drain(0..) {
                seen.insert(frontier_node, self.at(frontier_node).unwrap());
                for neighbour in self.neighbours_of(frontier_node) {
                    if !seen.contains_key(&neighbour) {
                        next_frontier.push(neighbour);
                    }
                }
            }
            frontier.extend(next_frontier);
            dist += 1;
        }
        (dist - 1, seen)
    }

    fn neighbours_of(&self, p: Point) -> Vec<Point> {
        let Some(tile) = self.at(p) else {
            return Vec::new();
        };
        match tile {
            Tile::NorthSouth => vec![p.north(), p.south()],
            Tile::EastWest => vec![p.east(), p.west()],
            Tile::NorthEast => vec![p.north(), p.east()],
            Tile::NorthWest => vec![p.north(), p.west()],
            Tile::SouthWest => vec![p.south(), p.west()],
            Tile::SouthEast => vec![p.south(), p.east()],
            Tile::Ground => Vec::new(),
            Tile::Start => panic!("Start should have been replaced by now"),
        }
    }
    // fn not_ground_at(&self, p: Point) -> bool {
    //     self.at(p).map(|t| t.not_ground()).unwrap_or_default()
    // }
    // fn ground_at_neither(&self, p: Point, q: Point) -> bool {
    //     self.not_ground_at(p) && self.not_ground_at(q)
    // }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    // fn northeast(&self) -> Self {
    //     Self {
    //         x: self.x + 1,
    //         y: self.y - 1,
    //     }
    // }
    // fn northwest(&self) -> Self {
    //     Self {
    //         x: self.x - 1,
    //         y: self.y - 1,
    //     }
    // }
    // fn southeast(&self) -> Self {
    //     Self {
    //         x: self.x + 1,
    //         y: self.y + 1,
    //     }
    // }
    // fn southwest(&self) -> Self {
    //     Self {
    //         x: self.x - 1,
    //         y: self.y + 1,
    //     }
    // }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn find_start(g: &Grid) -> Point {
    for y in 0..g.0.len() {
        for x in 0..g.0[0].len() {
            let p = Point { x, y };
            if g.at(p) == Some(Tile::Start) {
                return p;
            }
        }
    }
    panic!("Never found start");
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Tile {
    /// | is a vertical pipe connecting north and south.
    NorthSouth,
    /// - is a horizontal pipe connecting east and west.
    EastWest,
    /// L is a 90-degree bend connecting north and east.
    NorthEast,
    /// J is a 90-degree bend connecting north and west.
    NorthWest,
    /// 7 is a 90-degree bend connecting south and west.
    SouthWest,
    /// F is a 90-degree bend connecting south and east.
    SouthEast,
    /// . is ground; there is no pipe in this tile.
    Ground,
    /// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            other => panic!("Unexpected input character {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        for (input, start_is, expected_q1) in [
            (include_str!("../example1.txt"), Tile::SouthEast, 4),
            (include_str!("../example2.txt"), Tile::SouthEast, 8),
        ] {
            let (grid, start) = Grid::parse(input, start_is);
            let (actual_q1, this_loop) = grid.loop_containing(start);
            assert_eq!(actual_q1, expected_q1);
            dbg!(this_loop);
        }
    }
}
