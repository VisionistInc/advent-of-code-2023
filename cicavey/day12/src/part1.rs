use rayon::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            State::Operational => '.',
            State::Damaged => '#',
            State::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

fn det_seq(s: Vec<State>) -> Vec<usize> {
    let mut out = vec![];
    let mut cur = 0 as usize;
    for i in 1..s.len() {
        if s[i - 1] == State::Operational && s[i] == State::Damaged {
            // start
            cur = 0;
            continue;
        }
        if s[i - 1] == State::Damaged && s[i] == State::Damaged {
            cur += 1;
            continue;
        }
        if s[i - 1] == State::Damaged && s[i] == State::Operational {
            cur += 1;
            out.push(cur);
            cur = 0;
        }
    }

    if cur != 0 {
        out.push(cur + 1);
    } else if *s.last().unwrap() == State::Damaged {
        out.push(1);
    }

    return out;
}

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    mask: Vec<State>,
    offsets: Vec<usize>,
    check: Vec<usize>,
}

impl Entry {
    pub fn new(m: &str, c: Vec<usize>) -> Self {
        // convert M into offsets and

        let mask: Vec<State> = m
            .chars()
            .map(|c| match c {
                '#' => State::Damaged,
                '?' => State::Unknown,
                _ => State::Operational,
            })
            .collect();

        let offsets: Vec<_> = m
            .char_indices()
            .filter(|(_i, c)| *c == '?')
            .map(|(i, _c)| i)
            .collect();

        // let n = offsets.iter().permutations(offsets.len()).count();
        // println!("{} {}", m, n);

        return Self {
            mask: mask,
            offsets: offsets,
            check: c,
        };
    }
}

pub fn process(input: &str) -> String {
    let mut entries = vec![];

    input.lines().for_each(|l| {
        let a: Vec<_> = l.split_whitespace().collect();

        let mask = a[0];
        let constraints: Vec<_> = a[1]
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        entries.push(Entry::new(mask, constraints));
    });

    let sum: u64 = entries
        .par_iter()
        .map(|e| {
            let offsets = &e.offsets;
            let max = 1 << offsets.len();
            (0..max)
                .into_par_iter()
                .map(|v| {
                    let mut mask = e.mask.clone();
                    for offset_index in 0..offsets.len() {
                        if ((1 << offset_index) & v) != 0 {
                            mask[offsets[offset_index]] = State::Damaged;
                        } else {
                            mask[offsets[offset_index]] = State::Operational;
                        }
                    }

                    let mc = det_seq(mask);

                    if e.check == mc {
                        return 1;
                    } else {
                        return 0;
                    }
                })
                .sum::<u64>()
        })
        .sum();

    return format!("{:?}", sum);
}
