fn main() {
    let input = include_str!("../input.txt");
    let data = parse(input);
    let q1: i32 = data
        .into_iter()
        .map(all_deltas)
        .map(extrapolate_next_step)
        .sum();
    println!("Q1: {q1}");
}

type Value = i32;

type History = Vec<Value>;

fn all_deltas(h: History) -> Vec<History> {
    let mut out = vec![h];
    let mut all_zeroes = out.last().unwrap().iter().all(|&x| x == 0);
    while !all_zeroes {
        let curr = out.last().unwrap();
        let deltas: Vec<_> = (1..curr.len()).map(|i| curr[i] - curr[i - 1]).collect();
        all_zeroes = deltas.iter().all(|&x| x == 0);
        out.push(deltas);
    }
    out
}

fn extrapolate_next_step(h: Vec<History>) -> Value {
    let mut next = 0;
    let mut row = h.len() - 1;
    while row > 0 {
        next = h[row].last().unwrap() + next;
        row -= 1;
    }
    h[row].last().unwrap() + next
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
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ],
            vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
                vec![0, 0, 0],
            ],
            vec![
                vec![10, 13, 16, 21, 30, 45],
                vec![3, 3, 5, 9, 15],
                vec![0, 2, 4, 6],
                vec![2, 2, 2],
                vec![0, 0],
            ],
        ];
        assert_eq!(all_ds, expected_all_ds);
        let all_extrapolateds: Vec<_> = all_ds.into_iter().map(extrapolate_next_step).collect();
        assert_eq!(all_extrapolateds, vec![18, 28, 68]);
    }
}
