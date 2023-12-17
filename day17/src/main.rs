use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

fn main() {
    let grid = Grid::parse(include_str!("../input.txt"));
    let a1 = grid.q1();
    println!("Q1: {a1}");
}

struct Grid {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let tiles: Vec<_> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|n| n.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn at(&self, Point { x, y }: Point) -> Option<u32> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some(self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    fn q1(&self) -> u32 {
        // Initialize the data structures for Djikstra's algorithm.

        let mut tentative = HashMap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                tentative.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    u32::MAX,
                );
            }
        }
        let mut curr = Point::default();
        let mut visited: HashSet<Point> = HashSet::new();
        tentative.insert(curr, 0);
        let target = Point {
            x: self.width as i32 - 1,
            y: self.height as i32 - 1,
        };

        // Get all valid neighbours of this point
        // Neighbours are up/right/left/down of the point,
        // but in the grid, and not already explored.

        while !visited.contains(&target) {
            let unvisited_neighbours: Vec<_> = [
                (
                    Point {
                        x: curr.x,
                        y: curr.y + 1,
                    },
                    Dir::Down,
                ),
                (
                    Point {
                        x: curr.x,
                        y: curr.y - 1,
                    },
                    Dir::Up,
                ),
                (
                    Point {
                        x: curr.x + 1,
                        y: curr.y,
                    },
                    Dir::Right,
                ),
                (
                    Point {
                        x: curr.x - 1,
                        y: curr.y,
                    },
                    Dir::Left,
                ),
            ]
            .into_iter()
            .filter(|(p, _d)| self.at(*p).is_some())
            .filter(|(p, _d)| !visited.contains(p))
            .collect();
            for (node, dir) in unvisited_neighbours {
                let through_this_node = self.at(curr).unwrap() + *tentative.get(&curr).unwrap();
                let prev_best = *tentative.get(&node).unwrap();
                tentative.insert(node, min(prev_best, through_this_node));
            }
            // eprintln!("Visited {curr:?}");
            visited.insert(curr);
            curr = tentative
                .iter()
                .filter_map(|(node, cost)| {
                    if visited.contains(&node) {
                        None
                    } else {
                        Some((cost, *node))
                    }
                })
                .min_by(|(c1, _), (c2, _)| c1.cmp(c2))
                .unwrap()
                .1;
        }
        *tentative.get(&target).unwrap()
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let g = Grid::parse("../example.txt");
        let actual = g.q1();
        let expected = 102;
        assert_eq!(actual, expected);
    }
}
