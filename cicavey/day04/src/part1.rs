use std::collections::HashSet;
use std::iter::FromIterator;

pub fn process(input: &str) -> String {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let a: Vec<_> = line.split(":").collect();
        let card: u64 = a[0]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let b: Vec<_> = a[1].split("|").collect();

        let winning: Vec<_> = b[0]
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        let winning_set: HashSet<u64> = HashSet::from_iter(winning.iter().cloned());

        let play: Vec<_> = b[1]
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        let play_set: HashSet<u64> = HashSet::from_iter(play.iter().cloned());

        let win_count = play_set.intersection(&winning_set).count() as u64;

        if win_count > 0 {
            let prod = 1 << (win_count - 1);
            sum += prod;
        }
    }

    return format!("{}", sum);
}
