use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    cards: String,
    order: Vec<usize>,
    bet: u64,
    base: u8,
}

static ORDER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

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
            base: Self::base(cards) as u8,
        }
    }

    fn base(cards: &str) -> HandType {
        let mut counts: HashMap<char, u8> = HashMap::new();
        for c in cards.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let j = *counts.get(&'J').unwrap_or(&0);

        // CORRECT
        if j == 4 || j == 5 {
            return HandType::FiveOfAKind;
        }

        // CORRECT 5 of a kind
        if counts.len() == 1 {
            return HandType::FiveOfAKind;
        }

        // CORRECT
        if counts.values().any(|c| *c == 4) {
            if j == 1 {
                return HandType::FiveOfAKind; // promote to 5 of a kind
            }
            return HandType::FourOfAKind;
        }

        // correct
        if counts.len() == 2 {
            // if one of the sets if J, you really have 5 of a kind
            if j != 0 {
                return HandType::FiveOfAKind;
            }
            // full house
            return HandType::FullHouse;
        }

        // now it gets trickier

        // AAA B C
        if counts.values().any(|c| *c == 3) {
            // normal 3 of a kind
            if j == 0 {
                return HandType::ThreeOfAKind;
            }

            if j == 1 || j == 3 {
                return HandType::FourOfAKind;
            }

            // shouldn't get here
            return HandType::ThreeOfAKind;
        }

        if counts.len() == 3 {
            // AA BB C
            if j == 0 {
                return HandType::TwoPair;
            }

            // converts one pair to triple, so full
            // AA BB J
            if j == 1 {
                return HandType::FullHouse;
            }

            // AA JJ C
            if j == 2 {
                return HandType::FourOfAKind;
            }

            // two pair
            return HandType::TwoPair;
        }

        if counts.values().any(|c| *c == 2) {
            if j == 0 {
                // no jokers, 1 pair
                return HandType::OnePair;
            }

            if j == 1 {
                // one joker + 1 pair = 3
                return HandType::ThreeOfAKind;
            }

            if j == 2 {
                return HandType::ThreeOfAKind;
            }

            return HandType::OnePair;
        }

        // one joker means you at least have a pair at this point
        if j == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
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
