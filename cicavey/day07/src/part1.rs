use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    cards: String,
    order: Vec<usize>,
    bet: u64,
    base: u8,
}

static ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

impl Hand {
    pub fn new(cards: &str, bet: u64) -> Self {
        let v: Vec<_> = cards
            .chars()
            .map(|c| ORDER.iter().position(|o| *o == c).unwrap())
            .collect();

        Self {
            cards: cards.to_string(),
            order: v,
            bet: bet,
            base: Self::base(cards),
        }
    }

    fn base(cards: &str) -> u8 {
        let mut counts: HashMap<char, u8> = HashMap::new();
        for c in cards.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        // 5 of a kind
        if counts.len() == 1 {
            return 6;
        }

        if counts.values().any(|c| *c == 4) {
            return 5;
        }

        if counts.len() == 2 {
            // full house
            return 4;
        }

        if counts.values().any(|c| *c == 3) {
            return 3;
        }

        if counts.len() == 3 {
            // two pair
            return 2;
        }

        if counts.values().any(|c| *c == 2) {
            return 1;
        }

        0
    }
}

pub fn process(input: &str) -> String {
    let mut hands = vec![];

    for line in input.lines() {
        let h: Vec<_> = line.split_whitespace().collect();
        let bet: u64 = h[1].parse().unwrap();

        let hand = Hand::new(h[0], bet);

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        return if a.base == b.base {
            for i in 0..5 {
                let o = b.order[i].cmp(&a.order[i]);
                if o == Ordering::Equal {
                    continue;
                }
                return o;
            }
            Ordering::Equal
        } else {
            a.base.cmp(&b.base)
        };
    });

    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1) as u64 * v.bet)
        .sum();
    return format!("{}", total);
}
