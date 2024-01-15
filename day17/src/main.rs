use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn main() {
    let grid = Grid::parse(include_str!("../input.txt"));
    println!("Q1: {}", grid.q1());
}

#[derive(Debug)]
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

    fn q1(&self) -> u32 {
        let mut solutions = HashMap::new();
        let mut visited: HashMap<Node, u32> = HashMap::new();
        let mut tentative = PriorityQueue::new();
        tentative.push(
            Node {
                point: Default::default(),
                moves_in_straight_line: 0,
                current_direction: Dir::Down,
            },
            Priority { cost: 0 },
        );
        tentative.push(
            Node {
                point: Default::default(),
                moves_in_straight_line: 0,
                current_direction: Dir::Right,
            },
            Priority { cost: 0 },
        );
        // Get highest-priority item
        while let Some((curr, priority)) = tentative.pop() {
            let cost = priority.cost;
            // Are we at the final node?
            if curr.point.x == self.width - 1 && curr.point.y == self.height - 1 {
                solutions.insert(curr, cost);
            }
            // You can enter the final node from above or from left.
            // Once we've checked both of them, exit.
            if solutions.len() >= 2 {
                return solutions.values().copied().min().unwrap();
            }
            for neighbour in curr.neighbours(self.width, self.height) {
                // Don't visit the same node twice.
                if visited.contains_key(&neighbour) {
                    continue;
                }

                let cost_through_here = cost + self.at(neighbour.point);
                let min_cost = if let Some(previous_cost) =
                    tentative.get_priority(&neighbour).map(|p| p.cost)
                {
                    cost_through_here.min(previous_cost)
                } else {
                    cost_through_here
                };
                tentative.push(neighbour, Priority { cost: min_cost });
            }
            visited.insert(curr, cost);
        }
        panic!("Finished all tentative nodes but never found a terminal node")
    }

    fn at(&self, Point { x, y }: Point) -> u32 {
        self.tiles[y][x]
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Priority {
    cost: u32,
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Node {
    point: Point,
    moves_in_straight_line: u8,
    current_direction: Dir,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} ({})",
            self.point, self.current_direction, self.moves_in_straight_line
        )
    }
}

impl Node {
    /// Returns true if there's a directed edge from self to next in the graph for Q1.
    fn neighbours(&self, width: usize, height: usize) -> Vec<Self> {
        const MAX_MOVES_IN_SAME_DIR: u8 = 3;
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .filter_map(|next_direction| {
                let (next_dir, moves_in_straight_line) =
                    match (self.current_direction, next_direction) {
                        // Same direction
                        (Dir::Up, Dir::Up)
                        | (Dir::Down, Dir::Down)
                        | (Dir::Left, Dir::Left)
                        | (Dir::Right, Dir::Right) => {
                            // Cannot go in the same direction forever.
                            if self.moves_in_straight_line >= MAX_MOVES_IN_SAME_DIR {
                                return None;
                            }
                            (self.current_direction, self.moves_in_straight_line + 1)
                        }
                        // Cannot go backwards.
                        (Dir::Up, Dir::Down)
                        | (Dir::Down, Dir::Up)
                        | (Dir::Left, Dir::Right)
                        | (Dir::Right, Dir::Left) => return None,
                        // Remaining cases are all turns.
                        (_, next_direction) => (next_direction, 1),
                    };
                Node::move_along(self.point, next_dir, moves_in_straight_line, width, height)
            })
            .collect()
    }

    fn move_along(
        mut current: Point,
        next_dir: Dir,
        moves_in_straight_line: u8,
        width: usize,
        height: usize,
    ) -> Option<Node> {
        match next_dir {
            Dir::Left if current.x > 0 => {
                current.x -= 1;
            }
            Dir::Right if current.x < width - 1 => {
                current.x += 1;
            }
            Dir::Up if current.y > 0 => {
                current.y -= 1;
            }
            Dir::Down if current.y < height - 1 => {
                current.y += 1;
            }
            _ => return None,
        };
        Some(Self {
            current_direction: next_dir,
            moves_in_straight_line,
            point: current,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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
    fn test_q1_normal() {
        let g = Grid::parse(include_str!("../example.txt"));
        let actual = g.q1();
        let expected = 102;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_reddit_example() {
        let g = Grid::parse(include_str!("../example_from_reddit.txt"));
        let actual = g.q1();
        let expected = 17;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_q1_tiny() {
        let g = Grid::parse(include_str!("../tiny_example.txt"));
        let actual = g.q1();
        let expected = 11;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_priority() {
        // Lower cost = greater priority.
        assert!(Priority { cost: 0 } > Priority { cost: 4 });
        let mut pq = PriorityQueue::new();
        pq.push("Adam", Priority { cost: 0 });
        pq.push("Jordan", Priority { cost: 4 });
        assert_eq!(pq.pop().unwrap().0, "Adam");
    }
}
