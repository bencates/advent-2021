mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let sonar: day01::Sonar = include_str!("../data/day01.txt").parse().unwrap();

    println!("Day 1\n=====\n{}", sonar);

    let course: day02::Course = include_str!("../data/day02.txt").parse().unwrap();

    println!("Day 2\n=====\n{}", course);

    let diagnostics: day03::Diagnostics = include_str!("../data/day03.txt").parse().unwrap();

    println!("Day 3\n=====\n{}", diagnostics);
}
