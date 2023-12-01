use std::cmp::Reverse;

pub fn run(input: &str, log: fn(String)) {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let first_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

        let first = line.chars().nth(first_idx).unwrap();
        let last = line.chars().nth(last_idx).unwrap();

        let v: u64 = format!("{}{}", first, last).parse().unwrap();
        sum += v
    }

    log(format!("{}", sum));

    let mut sum2: u64 = 0;
    let d = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let values: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for line in input.lines() {
        let mut f: Vec<(usize, usize)> = d
            .iter()
            .map(|v| line.find(v).unwrap_or(line.len() + 1))
            .enumerate()
            .filter(|t| t.1 != line.len() + 1)
            .collect();
        f.sort_by_key(|v| v.1);

        let mut l: Vec<(usize, usize)> = d
            .iter()
            .map(|v| line.rfind(v).unwrap_or(line.len() + 1))
            .enumerate()
            .filter(|t| t.1 != line.len() + 1)
            .collect();
        l.sort_by_key(|v| Reverse(v.1));

        let v = values[f[0].0] * 10 + values[l[0].0];

        sum2 += v;
        // break;
    }

    log(format!("{}", sum2));
}
