// only 12 red cubes, 13 green cubes, and 14 blue cubes?
fn main() {
    let input = include_str!("../input.txt");
    let games: Vec<_> = input.lines().map(parse_game).collect();
    eprintln!("Q1: {}", q1(&games));
    eprintln!("Q2: {}", q2(&games));
}

/// Determine which games would have been possible if the bag had been loaded with only
/// 12 red cubes, 13 green cubes, and 14 blue cubes.
/// What is the sum of the IDs of those games?
fn q1(games: &[Game]) -> usize {
    let possible_games = games.iter().filter(|game| {
        game.cubes.iter().all(|cube| {
            cube <= &Cubes {
                r: 12,
                g: 13,
                b: 14,
            }
        })
    });
    possible_games.map(|game| game.id).sum()
}

/// For each game, find the minimum set of cubes that must have been present.
/// What is the sum of the power of these sets?
fn q2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.cubes
                .iter()
                .copied()
                .reduce(Cubes::max)
                .unwrap()
                .power()
        })
        .sum()
}

#[derive(Default, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Cubes {
    /// Red
    r: u8,
    /// Green
    g: u8,
    /// Blue
    b: u8,
}

impl Cubes {
    /// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
    fn power(self) -> u32 {
        u32::from(self.r) * u32::from(self.g) * u32::from(self.b)
    }

    fn new(color: &str, number: u8) -> Self {
        match color {
            "red" => Self {
                r: number,
                ..Default::default()
            },
            "green" => Self {
                g: number,
                ..Default::default()
            },
            "blue" => Self {
                b: number,
                ..Default::default()
            },
            other => panic!("Invalid color {other}"),
        }
    }

    fn max(self, rhs: Self) -> Self {
        use std::cmp::max;
        Self {
            r: max(self.r, rhs.r),
            g: max(self.g, rhs.g),
            b: max(self.b, rhs.b),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

impl std::ops::Add for Cubes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::iter::Sum for Cubes {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or_default()
    }
}

fn parse_game(line: &str) -> Game {
    let (header, game) = line.split_once(": ").unwrap();
    let game_id = header.split_once(' ').unwrap().1.parse().unwrap();
    let cubes = game
        .split("; ")
        .map(|s| {
            s.split(", ")
                .map(|reveal| {
                    let (number, color) = reveal.split_once(' ').unwrap();
                    Cubes::new(color, number.parse().unwrap())
                })
                .sum()
        })
        .collect();
    Game { id: game_id, cubes }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_q1() {
        let games: Vec<_> = EXAMPLE.lines().map(parse_game).collect();
        assert_eq!(8, q1(&games));
    }
    #[test]
    fn test_q2() {
        let games: Vec<_> = EXAMPLE.lines().map(parse_game).collect();
        assert_eq!(2286, q2(&games));
    }
}
