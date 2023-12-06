use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let q1: usize = parse_q1(input).iter().map(Race::ways_to_beat).product();
    println!("Q1: {q1}");
    let q2 = parse_q2(input).ways_to_beat();
    println!("Q2: {q2}");
}

#[derive(Debug)]
struct Race {
    duration: u64,
    best_distance: u64,
}

impl Race {
    fn ways_to_beat(&self) -> usize {
        (1u64..self.duration)
            .into_par_iter()
            .filter(|acceleration| {
                let time_remaining = self.duration - acceleration;
                let distance_travelled = acceleration * time_remaining;
                distance_travelled > self.best_distance
            })
            .count()
    }
}

fn parse_q1(s: &str) -> Vec<Race> {
    fn parse_line(s: &str) -> Vec<u64> {
        s.split_once(':')
            .unwrap()
            .1
            .trim()
            .split_ascii_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    }
    let mut lines = s.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            duration: time,
            best_distance: distance,
        })
        .collect()
}

fn parse_q2(s: &str) -> Race {
    let mut lines = s.lines();
    let time = lines.next().unwrap().split_once(':').unwrap().1.trim();
    let dist = lines.next().unwrap().split_once(':').unwrap().1.trim();
    fn join_bad_kerning(s: &str) -> u64 {
        s.chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap()
    }
    Race {
        best_distance: join_bad_kerning(dist),
        duration: join_bad_kerning(time),
    }
}
