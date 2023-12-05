fn main() {
    let input = Input::parse(include_str!("../input.txt"));
    // Q1
    let mut seeds = input.seeds.clone();
    for map_id in 0..7 {
        seeds = seeds
            .iter()
            .map(|seed| input.maps[map_id].do_map(*seed))
            .collect();
    }
    let lowest = seeds.iter().min().unwrap();
    println!("Q1: {lowest}");
}

type Id = u64;

#[derive(Debug)]
struct Input {
    seeds: Vec<Id>,
    maps: Vec<Map>,
}

impl Input {
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
