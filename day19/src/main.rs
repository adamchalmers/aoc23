use rustc_hash::FxHashMap as HashMap;

fn main() {
    let input = Input::parse(include_str!("../input.txt"));
    let q1: u32 = input.simulate().map(Part::sum).sum();
    println!("{q1}");
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut workflows = HashMap::default();
        let mut lines = input.lines();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }
            let wf = Workflow::parse(line);
            workflows.insert(wf.name.clone(), wf);
        }
        let parts: Vec<_> = lines.map(Part::parse).collect();
        Self { workflows, parts }
    }

    /// Returns all parts that are accepted
    fn simulate(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter().filter(|part| self.accepts_part(part))
    }

    fn accepts_part(&self, part: &Part) -> bool {
        let mut curr: &Workflow = self.workflows.get("in").expect("could not find workflow");
        'outer: loop {
            for rule in &curr.rules {
                // Didn't match, try the next rule.
                if !rule.matches(part) {
                    continue;
                }

                if let Some(x) = is_terminal(&rule.dst) {
                    return x;
                }
                curr = self
                    .workflows
                    .get(&rule.dst)
                    .expect("could not find workflow");
                continue 'outer;
            }
            if let Some(x) = is_terminal(&curr.default) {
                return x;
            }
            curr = self
                .workflows
                .get(&curr.default)
                .expect("could not find workflow");
        }
    }
}

fn is_terminal(s: &str) -> Option<bool> {
    if s == "A" {
        Some(true)
    } else if s == "R" {
        Some(false)
    } else {
        None
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(s: &str) -> Self {
        let mut slf = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for (component, val) in s[1..s.len() - 1]
            .split(',')
            .map(|x| x.split_once('=').unwrap())
        {
            let val = val.parse().unwrap();
            match component {
                "x" => {
                    slf.x = val;
                }
                "m" => {
                    slf.m = val;
                }
                "a" => {
                    slf.a = val;
                }
                "s" => {
                    slf.s = val;
                }
                other => panic!("invalid component {other}"),
            }
        }
        slf
    }
    /// Add all 4 components.
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

impl Workflow {
    fn parse(s: &str) -> Self {
        let (name, rest) = s.split_once('{').unwrap();
        let mut rule_parts: Vec<_> = rest[0..rest.len() - 1].split(',').collect();
        let default = rule_parts.pop().unwrap().to_owned();
        let rules = rule_parts.into_iter().map(Rule::parse).collect();
        Self {
            name: name.to_owned(),
            rules,
            default,
        }
    }
}

#[derive(Debug)]
struct Rule {
    lhs: String,
    cmp: CmpType,
    rhs: u32,
    dst: String,
}

#[derive(Debug)]
enum CmpType {
    Greater,
    Lesser,
}

impl Rule {
    fn parse(s: &str) -> Self {
        let lhs = s[0..1].to_owned();
        let cmp = match &s[1..2] {
            ">" => CmpType::Greater,
            "<" => CmpType::Lesser,
            other => panic!("bad comparison operator {other}"),
        };
        let (num, dst) = s[2..].split_once(':').unwrap();
        let rhs = num.parse().unwrap();
        Self {
            lhs,
            cmp,
            rhs,
            dst: dst.to_owned(),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let lhs = match self.lhs.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            other => panic!("invalid rule LHS: {other}"),
        };
        match self.cmp {
            CmpType::Greater => lhs > self.rhs,
            CmpType::Lesser => lhs < self.rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input = Input::parse(include_str!("../example.txt"));
        let actual: Vec<u32> = input.simulate().map(Part::sum).collect();
        let expected = vec![7540, 4623, 6951];
        assert_eq!(actual, expected);
    }
}
