use std::collections::HashSet;
use std::iter::FromIterator;

pub fn process(input: &str) -> String {
    let mut card_wins = vec![];

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

        card_wins.push(win_count);
    }

    let mut card_counts = vec![1u64; card_wins.len()];

    for i in 0..card_counts.len() {
        // println!("card {} x {}, wins: {}", i, card_counts[i], card_wins[i]);
        for _k in 0..card_counts[i] {
            for j in i + 1..=i + card_wins[i] as usize {
                // println!("inc {}", j);
                if (j > (card_counts.len() - 1)) {
                    break;
                }
                card_counts[j] += 1;
            }
        }
    }
    return format!("{}", card_counts.iter().sum::<u64>());
}
