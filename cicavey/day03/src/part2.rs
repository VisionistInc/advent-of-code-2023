use euclid::{point2, Point2D};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AoC {}

// #[derive(Debug, Clone, PartialEq)]
// struct Grid {
//     w: i32,
//     h: i32,
//     data: HashMap<Point2D<i32, AoC>, GridState>,
// }

#[derive(Debug, Clone, PartialEq)]
struct PartNum {
    val: u64,
    valid: bool,
    x: i32,
    y: i32,
    len: i32,
}

pub fn process(input: &str) -> String {
    let re = Regex::new(r"(\d+|[^.])").unwrap();

    let mut sym_grid: HashMap<Point2D<i32, AoC>, &str> = HashMap::new();
    let mut parts = Vec::new();

    let mut y = 0i32;
    for line in input.lines() {
        re.captures_iter(line).for_each(|c| {
            let m = c.get(0).unwrap();
            let res = m.as_str().parse::<u64>();
            if res.is_ok() {
                let part = PartNum {
                    val: res.unwrap(),
                    valid: false,
                    x: m.start() as i32,
                    y: y,
                    len: m.len() as i32,
                };
                parts.push(part);
            } else {
                let p = point2(m.start() as i32, y);
                sym_grid.insert(p, m.as_str());
            }
        });
        y += 1;
    }

    let mut gear_mapping: HashMap<Point2D<i32, AoC>, Vec<u64>> = HashMap::new();

    for p in parts.iter_mut() {
        // figure out if each part is valid by checking all adjacent

        for y in p.y - 1..=p.y + 1 {
            for x in p.x - 1..=(p.x + p.len) {
                let check = point2(x, y);
                if sym_grid.contains_key(&check) {
                    let sym = sym_grid.get(&check).unwrap();
                    if *sym == "*" {
                        // this gear is adjacent to a number
                        gear_mapping.entry(check).or_insert(Vec::new()).push(p.val);
                    }
                }
            }
        }
    }

    let ratio_sum: u64 = gear_mapping
        .iter()
        .filter(|e| e.1.len() == 2)
        .map(|e| e.1.iter().product::<u64>())
        .sum();

    return format!("{}", ratio_sum);
}
