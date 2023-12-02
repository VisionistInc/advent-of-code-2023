pub fn process(input: &str) -> String {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let first_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

        let first = line.chars().nth(first_idx).unwrap();
        let last = line.chars().nth(last_idx).unwrap();

        let v: u64 = format!("{}{}", first, last).parse().unwrap();
        sum += v
    }

    return format!("{}", sum);
}
