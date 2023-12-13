fn main() {
    let grids = parse_grids(include_str!("../input.txt"));
    // let grids = parse_grids(include_str!("../example_combined.txt"));
    eprintln!("Loaded {} grids", grids.len());
    let q1: usize = grids
        .into_iter()
        .map(|grid| match Reflection::of(grid) {
            Reflection::Col(x) => x,
            Reflection::Row(x) => 100 * x,
        })
        .sum();
    println!("Q1: {q1}");
}

#[derive(Debug)]
struct Grid(Vec<Vec<char>>);

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Col(usize),
    Row(usize),
}

fn parse_grids(s: &str) -> Vec<Grid> {
    s.split("\n\n").map(Grid::parse).collect()
}

impl Grid {
    fn parse(s: &str) -> Self {
        Self(s.lines().map(|line| line.chars().collect()).collect())
    }
    fn row(&self, i: usize) -> &[char] {
        &self.0[i]
    }
    fn col(&self, i: usize) -> Vec<char> {
        (0..self.0.len()).map(|y| self.0[y][i]).collect()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
}

impl Reflection {
    fn of(grid: Grid) -> Self {
        // Try every row
        let height = grid.height();
        for y in 0..height - 1 {
            let mut i = 0;
            let mut rows_different = false;
            while y + 1 + i < height && (y as isize - i as isize >= 0) {
                if grid.row(y - i) != grid.row(y + 1 + i) {
                    rows_different = true;
                    break;
                }
                i += 1;
            }
            if !rows_different {
                return Self::Row(y + 1);
            }
        }

        // Try every column
        let width = grid.width();
        for x in 0..width - 1 {
            let mut i = 0;
            let mut cols_different = false;
            while x + i + 1 < width && (x as isize - i as isize >= 0) {
                if grid.col(x - i) != grid.col(x + 1 + i) {
                    cols_different = true;
                    break;
                }
                i += 1;
            }
            if !cols_different {
                return Self::Col(x + 1);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        for (expected, input) in [
            (Reflection::Col(5), include_str!("../example.txt")),
            (Reflection::Row(4), include_str!("../example2.txt")),
        ] {
            let map = Grid::parse(input);
            let actual = Reflection::of(map);
            assert_eq!(actual, expected);
        }
    }
}
