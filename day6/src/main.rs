fn main() {
    let input = include_str!("../input.txt");
    let record = parse(input);
    let q1: usize = record.iter().map(Race::ways_to_beat).product();
    println!("Q1: {q1}");
    let q2 = parse_q2(input).ways_to_beat();
    println!("Q2: {q2}");
}

fn parse_line(s: &str) -> Vec<usize> {
    s.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap())
        .collect()
}

fn parse(s: &str) -> Vec<Race> {
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

#[derive(Copy, Clone, Debug)]
struct Race {
    duration: usize,
    best_distance: usize,
}

fn parse_q2(s: &str) -> Race {
    let mut lines = s.lines();
    let time = lines.next().unwrap().split_once(':').unwrap().1.trim();
    let dist = lines.next().unwrap().split_once(':').unwrap().1.trim();
    Race {
        best_distance: join_bad_kerning(dist),
        duration: join_bad_kerning(time),
    }
}

fn join_bad_kerning(s: &str) -> usize {
    s.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap()
}

impl Race {
    fn ways_to_beat(&self) -> usize {
        (1..self.duration)
            .filter(|acceleration| {
                let time_remaining = self.duration - acceleration;
                let distance_travelled = acceleration * time_remaining;
                distance_travelled > self.best_distance
            })
            .count()
    }
}
