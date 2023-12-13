use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let q1 = Universe::parse(input, 2).q1();
    println!("Q1: {q1}");
    let q2 = Universe::parse(input, 1_000_000).q1();
    println!("Q2: {q2}");
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Point>,
}

impl Universe {
    fn q1(&self) -> usize {
        let mut total_distances = 0;
        for i in 0..self.galaxies.len() {
            for j in (i + 1)..self.galaxies.len() {
                total_distances += self.galaxies[i].distance_to(self.galaxies[j]);
            }
        }
        total_distances
    }
    fn parse(s: &str, expansion_factor: usize) -> Self {
        let mut galaxies_before_expansion = HashSet::new();
        let lines = s.lines().collect::<Vec<_>>();
        let mut width = 0;
        let height = lines.len();
        for (y, row) in lines.into_iter().enumerate() {
            width = row.len();
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    galaxies_before_expansion.insert(Point { x, y });
                }
            }
        }
        let expanding_rows: Vec<_> = (0..height)
            .filter(|y| {
                let row_has_galaxy =
                    (0..width).any(|x| galaxies_before_expansion.contains(&Point { x, y: *y }));
                !row_has_galaxy
            })
            .collect();
        let expanding_cols: Vec<_> = (0..width)
            .filter(|x| {
                let col_has_galaxy =
                    (0..height).any(|y| galaxies_before_expansion.contains(&Point { x: *x, y }));
                !col_has_galaxy
            })
            .collect();
        let galaxies = galaxies_before_expansion
            .into_iter()
            .map(|mut galaxy| {
                galaxy.x += expanding_cols.iter().filter(|x| x < &&galaxy.x).count()
                    * (expansion_factor - 1);
                galaxy.y += expanding_rows.iter().filter(|y| y < &&galaxy.y).count()
                    * (expansion_factor - 1);
                galaxy
            })
            .collect();
        Self { galaxies }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance_to(&self, other: Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_expansion() {
        let input = include_str!("../example.txt");
        let universe = Universe::parse(input, 2);
        let expected_galaxies = vec![
            Point { x: 4, y: 0 },
            Point { x: 9, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 8, y: 5 },
            Point { x: 1, y: 6 },
            Point { x: 12, y: 7 },
            Point { x: 9, y: 10 },
            Point { x: 0, y: 11 },
            Point { x: 5, y: 11 },
        ];
        let mut actual: Vec<_> = universe.galaxies;
        actual.sort_by_key(|p| (p.y, p.x));
        assert_eq!(expected_galaxies, actual);
    }

    #[test]
    fn test_distances() {
        let input = include_str!("../example.txt");
        for (expansion_factor, expected) in [(2, 374), (10, 1030), (100, 8410)] {
            let universe = Universe::parse(input, expansion_factor);
            assert_eq!(universe.q1(), expected);
        }
    }
}
