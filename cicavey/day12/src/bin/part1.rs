use day12::part1::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result)
}
