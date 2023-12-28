use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::NodeIndex, Directed, Graph};

const Q1_MAX_MOVES_IN_STRAIGHT_LINE: u8 = 3;

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

    fn at(&self, Point { x, y }: Point) -> Option<u32> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some(self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    fn q1(&self) -> u32 {
        let (graph, starts, ends) = self.build_graph(Node::edges_q1, Q1_MAX_MOVES_IN_STRAIGHT_LINE);
        solve(graph, starts, ends)
    }

    /// Returns the graph, starts, and ends.
    fn build_graph<F>(
        &self,
        has_edge: F,
        max_moves_in_straight_line: u8,
    ) -> (Graph<Node, u32, Directed>, Vec<NodeIndex>, Vec<NodeIndex>)
    where
        F: Fn(&Node, Node, u8) -> bool,
    {
        let mut g = Graph::new();
        let nodes: Vec<(NodeIndex, Node)> = (0..self.width)
            .cartesian_product(0..self.height)
            .cartesian_product(0u8..max_moves_in_straight_line)
            .cartesian_product([Dir::Up, Dir::Down, Dir::Left, Dir::Right])
            .map(|(((x, y), moves_since_turn), current_direction)| Node {
                point: Point {
                    x: x as i32,
                    y: y as i32,
                },
                moves_in_straight_line: moves_since_turn,
                current_direction,
            })
            .map(|node| {
                let i = g.add_node(node);
                (i, node)
            })
            .collect();

        let mut starts = Vec::new();
        let mut ends = Vec::new();
        for (i, node0) in &nodes {
            if node0.point.x == 0
                && node0.point.y == 0
                && matches!(node0.current_direction, Dir::Right | Dir::Down)
            {
                starts.push(*i);
            }
            if node0.point.x == self.width as i32 - 1 && node0.point.y == self.height as i32 - 1 {
                ends.push(*i);
            }
            for (j, node1) in &nodes {
                if has_edge(node0, *node1, max_moves_in_straight_line) {
                    g.add_edge(*i, *j, self.at(node1.point).unwrap());
                }
            }
        }
        (g, starts, ends)
    }
}

fn solve(graph: Graph<Node, u32>, starts: Vec<NodeIndex>, ends: Vec<NodeIndex>) -> u32 {
    let distances_for_each_start: Vec<_> = starts
        .iter()
        .map(|start| dijkstra(&graph, *start, None, |e| *e.weight()))
        .collect();
    distances_for_each_start
        .iter()
        .map(|distances| {
            let x: u32 = distances
                .iter()
                .filter_map(|(node_id, cost)| {
                    if ends.contains(node_id) {
                        Some(*cost)
                    } else {
                        None
                    }
                })
                .min()
                .unwrap();
            x
        })
        .min()
        .unwrap()
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
    fn edges_q1(&self, next: Node, max_moves_in_straight_line: u8) -> bool {
        match (self.current_direction, next.current_direction) {
            // Same direction
            (Dir::Up, Dir::Up)
            | (Dir::Down, Dir::Down)
            | (Dir::Left, Dir::Left)
            | (Dir::Right, Dir::Right) => {
                // Cannot go in the same direction forever.
                if self.moves_in_straight_line > max_moves_in_straight_line {
                    return false;
                }
                // The next step must be 1 step forwards.
                if self.moves_in_straight_line + 1 != next.moves_in_straight_line {
                    return false;
                }
                // The next location must actually be 1 step along the current direction.

                match self.current_direction {
                    Dir::Up => self.point.x == next.point.x && self.point.y - 1 == next.point.y,
                    Dir::Down => self.point.x == next.point.x && self.point.y + 1 == next.point.y,
                    Dir::Left => self.point.y == next.point.y && self.point.x - 1 == next.point.x,
                    Dir::Right => self.point.y == next.point.y && self.point.x + 1 == next.point.x,
                }
            }
            // Cannot go backwards.
            (Dir::Up, Dir::Down)
            | (Dir::Down, Dir::Up)
            | (Dir::Left, Dir::Right)
            | (Dir::Right, Dir::Left) => false,
            // Remaining cases are all turns.
            turn => {
                if next.moves_in_straight_line != 0 {
                    return false;
                }
                match turn {
                    (_, Dir::Left) => {
                        self.point.x - 1 == next.point.x && self.point.y == next.point.y
                    }
                    (_, Dir::Right) => {
                        self.point.x + 1 == next.point.x && self.point.y == next.point.y
                    }
                    (_, Dir::Up) => {
                        self.point.x == next.point.x && self.point.y - 1 == next.point.y
                    }
                    (_, Dir::Down) => {
                        self.point.x == next.point.x && self.point.y + 1 == next.point.y
                    }
                }
            }
        }
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
    x: i32,
    y: i32,
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
}
