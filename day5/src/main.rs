use rayon::prelude::*;

type Id = usize;

fn main() {
    let input = Input::parse(include_str!("../input.txt"));
    // Q1
    let seeds = input.seeds.clone();
    println!("Q1: {}", lowest_location(seeds, &input));

    // Q2
    let seeds: Vec<_> = input
        .seed_ranges()
        .into_par_iter()
        .flat_map(|(start, len)| {
            vec![0; len]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + start)
                .collect::<Vec<_>>()
        })
        .collect();
    println!("Brute-force checking each of {} seeds", seeds.len());
    println!("Q2: {}", lowest_location(seeds, &input));
}

fn lowest_location(mut seeds: Vec<Id>, input: &Input) -> Id {
    for map_id in 0..7 {
        seeds = seeds
            .iter()
            .map(|seed| input.maps[map_id].do_map(*seed))
            .collect();
    }
    *seeds.iter().min().unwrap()
}

#[derive(Debug)]
struct Input {
    seeds: Vec<Id>,
    maps: Vec<Map>,
}

impl Input {
    fn seed_ranges(&self) -> Vec<(Id, Id)> {
        (0..self.seeds.len() / 2)
            .map(|i| (self.seeds[2 * i], self.seeds[2 * i + 1]))
            .collect()
    }

    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let seeds: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|num| num.parse::<Id>().unwrap())
            .collect();
        lines.next();

        let mut maps = Vec::new();
        for _ in 0..7 {
            lines.next().unwrap();
            maps.push(Map::parse(&mut lines));
        }

        Input { seeds, maps }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn do_map(&self, input: Id) -> Id {
        for row in &self.rows {
            if row.contains(input) {
                return row.destination_range_start + (input - row.source_range_start);
            }
        }
        input
    }
    fn parse<'a, I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut rows = Vec::new();
        let mut line = lines.next().unwrap();
        while !line.trim().is_empty() {
            rows.push(Row::parse(line));
            line = lines.next().unwrap();
        }
        Self { rows }
    }
}

#[derive(Debug)]
struct Row {
    destination_range_start: Id,
    source_range_start: Id,
    range_length: Id,
}

impl Row {
    fn contains(&self, input: Id) -> bool {
        (self.source_range_start..self.source_range_start + self.range_length).contains(&input)
    }
    fn parse(s: &str) -> Self {
        let v = s
            .split_ascii_whitespace()
            .map(|num| num.parse::<Id>().unwrap())
            .collect::<Vec<_>>();
        let [destination_range_start, source_range_start, range_length]: [Id; 3] =
            v.try_into().unwrap();
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}
