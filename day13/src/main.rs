fn main() {
    let grids = parse_grids(include_str!("../input.txt"));
    let q1: usize = grids
        .iter()
        .map(|grid| Reflection::of_q1(grid, None).unwrap().score())
        .sum();
    println!("Q1: {q1}");
    let q2: usize = grids
        .into_iter()
        .map(|grid| Reflection::of_q2(grid).score())
        .sum();
    println!("Q2: {q2}");
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

    fn smudge(&mut self, x: usize, y: usize) {
        self.0[y][x] = if self.0[y][x] == '.' { '#' } else { '.' }
    }
}

impl Reflection {
    fn of_q1(grid: &Grid, excluding: Option<&Reflection>) -> Option<Self> {
        // Try every row
        let height = grid.height();
        for y in 0..height - 1 {
            let mut rows_different = false;
            let mut a = y as isize;
            let mut b = y as isize + 1;
            while a >= 0 && b < (height as isize) {
                if grid.row(a as usize) != grid.row(b as usize) {
                    rows_different = true;
                    break;
                }
                a -= 1;
                b += 1;
            }
            if excluding != Some(&Reflection::Row(y)) && !rows_different {
                return Some(Self::Row(y));
            }
        }

        // Try every column
        let width = grid.width();
        for x in 0..width - 1 {
            let mut cols_different = false;
            let mut a = x as isize;
            let mut b = x as isize + 1;
            while a >= 0 && b < (width as isize) {
                if grid.col(a as usize) != grid.col(b as usize) {
                    cols_different = true;
                    break;
                }
                a -= 1;
                b += 1;
            }
            if excluding != Some(&Reflection::Col(x)) && !cols_different {
                return Some(Self::Col(x));
            }
        }
        None
    }

    fn of_q2(mut grid: Grid) -> Self {
        // Answer cannot be this.
        let answer_q1 = Self::of_q1(&grid, None).unwrap();
        // Try smudging every cell.
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                // Smudge the grid at this cell.
                grid.smudge(x, y);

                // Was this smudge right?
                match Self::of_q1(&grid, Some(&answer_q1)) {
                    Some(answer) => {
                        return answer;
                    }
                    _ => {
                        // If not, unsmudge it.
                        grid.smudge(x, y);
                    }
                }
            }
        }
        panic!("Failed to find any new line of reflection in grid");
    }

    fn score(&self) -> usize {
        match self {
            Reflection::Col(x) => x + 1,
            Reflection::Row(x) => 100 * (x + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        for (expected, input) in [
            (Reflection::Col(4), include_str!("../example.txt")),
            (Reflection::Row(3), include_str!("../example2.txt")),
        ] {
            let map = Grid::parse(input);
            let actual = Reflection::of_q1(&map, None).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
