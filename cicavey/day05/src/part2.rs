use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
struct ElfRange {
    src: u64,
    dst: u64,
    len: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct Mapping {
    src: String,
    dst: String,
    ranges: Vec<ElfRange>,
}

impl Mapping {
    fn map(&self, id: u64) -> u64 {
        for r in &self.ranges {
            if id >= r.src && id < r.src + r.len {
                return r.dst + (id - r.src);
            }
        }
        id
    }
}

pub fn process(input: &str) -> String {
    let mapre = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut lines = input.lines();

    let seed_line = lines.next().unwrap();
    let seeds: Vec<_> = seed_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // skip blank
    lines.next();

    let mut mappings = vec![];

    while let Some(line) = lines.next() {
        let caps = mapre.captures(line).unwrap();

        let src = caps.get(1).unwrap().as_str();
        let dst = caps.get(2).unwrap().as_str();

        let mut mapping = Mapping {
            src: src.to_string(),
            dst: dst.to_string(),
            ranges: vec![],
        };

        while let Some(sub) = lines.next() {
            if sub.is_empty() {
                break;
            }
            let range_def: Vec<_> = sub
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            let er = ElfRange {
                src: range_def[1],
                dst: range_def[0],
                len: range_def[2],
            };

            mapping.ranges.push(er);
        }

        mappings.push(mapping);
    }

    let mut min_loc = u64::MAX;

    for (&start, &len) in seeds.iter().tuples() {
        let loc = (start..(start + len))
            .into_par_iter() // parallel go brrrrrrrrr
            .map(|seed| {
                return mappings
                    .iter()
                    .fold(seed, |location, map| map.map(location));
            })
            .min()
            .unwrap();
        min_loc = min(min_loc, loc);
    }

    return format!("{}", min_loc);
}
