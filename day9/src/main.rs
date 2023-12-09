fn main() {
    let input = include_str!("../input.txt");
    let data = parse(input);
    let deltas: Vec<_> = data.into_iter().map(all_deltas).collect();
    let q1: i32 = deltas.iter().map(|h| extrapolate_next_step(h)).sum();
    println!("Q1: {q1}");
    let q2: i32 = deltas.iter().map(|h| extrapolate_prev_step(h)).sum();
    assert_eq!(q2, 1089);
    println!("Q2: {q2}");
}

type Value = i32;
type History = Vec<Value>;

fn all_deltas(h: History) -> Vec<History> {
    let mut rows = vec![];
    let mut next_row = h;
    let mut all_zeroes = false;
    while !all_zeroes {
        rows.push(next_row);
        let curr = rows.last().unwrap();
        next_row = (1..curr.len()).map(|i| curr[i] - curr[i - 1]).collect();
        all_zeroes = next_row.iter().all(|&x| x == 0);
    }
    rows
}

fn extrapolate_next_step(h: &[History]) -> Value {
    h.iter()
        .rev()
        .fold(0, |next, row| next + row.last().unwrap())
}

fn extrapolate_prev_step(h: &[History]) -> Value {
    h.iter()
        .rev()
        .fold(0, |next, row| row.first().unwrap() - next)
}

fn parse(s: &str) -> Vec<History> {
    s.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> History {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let data = parse(include_str!("../example.txt"));
        let all_ds: Vec<_> = data.into_iter().map(all_deltas).collect();
        let expected_all_ds = vec![
            vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3]],
            vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
            ],
            vec![
                vec![10, 13, 16, 21, 30, 45],
                vec![3, 3, 5, 9, 15],
                vec![0, 2, 4, 6],
                vec![2, 2, 2],
            ],
        ];
        assert_eq!(all_ds, expected_all_ds);
        let all_extrapolateds: Vec<_> = all_ds
            .into_iter()
            .map(|h| extrapolate_next_step(&h))
            .collect();
        assert_eq!(all_extrapolateds, vec![18, 28, 68]);
    }
}
