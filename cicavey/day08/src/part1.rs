use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    left: String,
    lid: usize,
    right: String,
    rid: usize,
}

impl Node {
    pub fn new(name: &str, left: &str, right: &str) -> Self {
        Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            lid: 0,
            rid: 0,
        }
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

    let start = nodes.iter().position(|f| f.name == "AAA").unwrap();
    let end = nodes.iter().position(|f| f.name == "ZZZ").unwrap();
    let mut cur = start;

    let mut steps = 0u64;
    for d in directions.chars().cycle() {
        if d == 'L' {
            cur = nodes[cur].lid;
        } else {
            cur = nodes[cur].rid;
        }

        steps += 1;

        if cur == end {
            break;
        }

        // if steps > 100 {
        //     break;
        // }
    }

    return format!("{}", steps);
}
