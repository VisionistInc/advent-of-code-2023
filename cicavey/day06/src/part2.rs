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

    let t = lines[0]
        .splitn(2, ":")
        .skip(1)
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let d = lines[1]
        .splitn(2, ":")
        .skip(1)
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    return format!("{}", ways(t, d));
}
