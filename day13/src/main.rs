fn main() {
    let grids = parse_grids(include_str!("../input.txt"));
    // let grids = parse_grids(include_str!("../example2.txt"));
    eprintln!("Loaded {} grids", grids.len());
    let q1: usize = grids
        .iter()
        .map(|grid| match Reflection::of_q1(grid).unwrap() {
            Reflection::Col(x) => x,
            Reflection::Row(x) => 100 * x,
        })
        .sum();
    println!("Q1: {q1}");
    let q2: usize = grids
        .into_iter()
        .enumerate()
        .map(|(i, grid)| {
            let (answer, x, y) = Reflection::of_q2(i, grid);
            println!("Smudge at ({x},{y})");
            match answer {
                Reflection::Col(x) => x,
                Reflection::Row(x) => 100 * x,
            }
        })
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
    fn of_q1(grid: &Grid) -> Option<Self> {
        // Try every row
        let height = grid.height();
        for y in 0..height - 1 {
            let mut rows_different = false;
            let mut a = y as isize;
            let mut b = y as isize + 1;
            while b < (height as isize) && a >= 0 {
                if grid.row(a as usize) != grid.row(b as usize) {
                    rows_different = true;
                    break;
                }
                a -= 1;
                b += 1;
            }
            if !rows_different {
                return Some(Self::Row(y + 1));
            }
        }

        // Try every column
        let width = grid.width();
        for x in 0..width - 1 {
            let mut cols_different = false;
            let mut a = x as isize;
            let mut b = x as isize + 1;
            while b < (width as isize) && a >= 0 {
                if grid.col(a as usize) != grid.col(b as usize) {
                    cols_different = true;
                    break;
                }
                a -= 1;
                b += 1;
            }
            if !cols_different {
                return Some(Self::Col(x + 1));
            }
        }
        None
    }

    fn of_q2(i: usize, mut grid: Grid) -> (Self, usize, usize) {
        // Answer cannot be this.
        let answer_q1 = Self::of_q1(&grid).unwrap();
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                // Smudge the grid.
                grid.smudge(x, y);

                // Was this smudge right?
                match Self::of_q1(&grid) {
                    Some(answer) if answer != answer_q1 => {
                        return (answer, x + 1, y + 1);
                    }
                    _ => {
                        // If not, unsmudge it.
                        grid.smudge(x, y);
                    }
                }
            }
        }
        panic!("Failed to find any new line of reflection in grid {i}");
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
            let actual = Reflection::of_q1(&map).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
