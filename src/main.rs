mod sonar;

use sonar::depth_increase;

fn main() {
    let day01_input: Vec<u32> = include_str!("../data/day01.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Day 1");
    println!("Simple depth count: {}", depth_increase(&day01_input, 1));
    println!("Windowed depth count: {}", depth_increase(&day01_input, 3));
    println!();
}
