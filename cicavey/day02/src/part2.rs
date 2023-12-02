use std::{cmp::max, collections::HashMap};

pub fn process(input: &str) -> String {
    let cube_counts = HashMap::from([("red", 12u64), ("green", 13), ("blue", 14)]);
    let mut sum = 0u64;
    for line in input.lines() {
        let a: Vec<&str> = line.splitn(2, ": ").collect();
        let b: Vec<&str> = a[0].split(" ").collect();
        let game_id: u64 = b[1].parse().unwrap();

        let raw_cube_sets: Vec<&str> = a[1].split(";").map(|s| s.trim()).collect();

        let cube_sets: Vec<_> = raw_cube_sets
            .iter()
            .map(|cs| {
                let mut m = HashMap::new();
                cs.split(",").map(|s| s.trim()).for_each(|s| {
                    let p: Vec<_> = s.split(" ").collect();
                    let count: u64 = p[0].parse().unwrap();
                    m.insert(p[1], count);
                });
                return m;
            })
            .collect();

        let mut min_counts = HashMap::from([("red", 0u64), ("green", 0), ("blue", 0)]);

        cube_sets.iter().for_each(|cs| {
            cs.iter().for_each(|(k, v)| {
                min_counts.insert(k, max(*v, min_counts[k]));
            });
        });

        let power: u64 = min_counts.into_values().product();
        sum += power;
    }

    return format!("{}", sum);
}
