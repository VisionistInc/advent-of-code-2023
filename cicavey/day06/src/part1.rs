fn ways(t: u64, d: u64) -> u64 {
    let mut ways = 0u64;
    for hold in 0..=t {
        let speed = hold;
        let rem_t = t - hold;
        let travel_dist = speed * rem_t;

        if travel_dist > d {
            ways += 1
        }
    }
    return ways;
}

pub fn process(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let time: Vec<_> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let dist: Vec<_> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let p: u64 = time.iter().zip(dist).map(|(t, d)| ways(*t, d)).product();

    return format!("{}", p);
}
