mod course;
mod sonar;
mod submarine;

use course::Command;
use sonar::depth_increase;
use submarine::{DiagnosticReport, Submarine};

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
    println!(
        "Correct location (product): {}",
        submarine2.position() * submarine2.depth()
    );
    println!();

    let diagnostics: DiagnosticReport = include_str!("../data/day03.txt").parse().unwrap();

    println!("Day 3");
    println!("Power consumption: {}", diagnostics.power_consumption());
    println!("Life support rating: {}", diagnostics.life_support_rating());
    println!();
}
