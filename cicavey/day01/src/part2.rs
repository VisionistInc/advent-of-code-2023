use std::cmp::Reverse;

pub fn process(input: &str) -> String {
    let mut sum: u64 = 0;
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

        sum += v;
    }

    return format!("{}", sum);
}
