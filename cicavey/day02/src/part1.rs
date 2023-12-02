use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let cube_counts = HashMap::from([("red", 12u64), ("green", 13), ("blue", 14)]);
    let mut sum: u64 = 0;
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

        let valid = cube_sets
            .iter()
            .map(|cs| {
                return cs.iter().map(|(k, v)| v <= &cube_counts[k]).all(|a| a);
            })
            .all(|a| a);

        if valid {
            sum += game_id;
        }

        // println!("{}, {:?}", game_id, valid)
    }

    return format!("{}", sum);
}
