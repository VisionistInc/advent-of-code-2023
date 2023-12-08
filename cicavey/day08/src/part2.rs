use std::f32::consts::E;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    left: String,
    lid: usize,
    right: String,
    rid: usize,
    end: bool,
    steps: u64,
}

impl Node {
    pub fn new(name: &str, left: &str, right: &str) -> Self {
        Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            lid: 0,
            rid: 0,
            end: name.ends_with("Z"),
            steps: 0,
        }
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn process(input: &str) -> String {
    let mut nodes = vec![];

    let mut line_iter = input.lines();

    let directions = line_iter.next().unwrap();
    // skip blank
    line_iter.next();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)").unwrap();

    for line in line_iter {
        let cap = re.captures(line).unwrap();
        let src = cap.get(1).unwrap().as_str();
        let left = cap.get(2).unwrap().as_str();
        let right = cap.get(3).unwrap().as_str();

        nodes.push(Node::new(src, left, right));
    }

    for i in 0..nodes.len() {
        let left = nodes[i].left.clone();
        let right = nodes[i].right.clone();

        let lid = nodes.iter().position(|f| f.name == left).unwrap();
        let rid = nodes.iter().position(|f| f.name == right).unwrap();

        nodes[i].rid = rid;
        nodes[i].lid = lid;
    }

    let mut cur: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_i, v)| v.name.ends_with("A"))
        .map(|(i, _v)| i)
        .collect();

    // solve each path, then LCM

    for i in 0..cur.len() {
        let start = cur[i];
        let mut c = cur[i];

        let mut steps = 0u64;
        for d in directions.chars().cycle() {
            if d == 'L' {
                c = nodes[c].lid;
            } else {
                c = nodes[c].rid;
            }
            steps += 1;
            if nodes[c].end {
                break;
            }
        }
        nodes[start].steps = steps;
    }

    let lcm = cur
        .iter()
        .map(|c| nodes[*c].steps)
        .fold(1, |acc, e| lcm(acc, e));

    return format!("{}", lcm);
}
