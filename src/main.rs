mod course;
mod sonar;
mod submarine;

use course::Command;
use sonar::depth_increase;
use submarine::Submarine;

fn main() {
    let day01_input: Vec<u32> = include_str!("../data/day01.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Day 1");
    println!("Simple depth count: {}", depth_increase(&day01_input, 1));
    println!("Windowed depth count: {}", depth_increase(&day01_input, 3));
    println!();

    let mut submarine1 = Submarine::default();
    let mut submarine2 = Submarine::default();

    let course: Vec<Command> = include_str!("../data/day02.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    submarine1.run_incorrectly(&course);
    submarine2.run_correctly(&course);

    println!("Day 2");
    println!(
        "Incorrect location (product): {}",
        submarine1.position() * submarine1.depth()
    );
    println!("{:?}", submarine2);
    println!(
        "Correct location (product): {}",
        submarine2.position() * submarine2.depth()
    );
}
