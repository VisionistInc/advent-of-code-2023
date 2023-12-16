fn hash(s: &str) -> u64 {
    s.chars()
        .fold(0u64, |acc, c| ((acc + (c as u64)) * 17) % 256)
}

pub fn process(input: &str) -> String {
    return format!("{}", input.split(",").map(hash).sum::<u64>());
}
