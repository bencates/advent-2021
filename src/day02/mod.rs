mod command;

use command::Command;
use std::{fmt, str::FromStr};

pub struct Course(Vec<Command>);

impl FromStr for Course {
    type Err = command::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let commands = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(commands))
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (incorrect_position, incorrect_depth) = self.run_incorrectly();
        let (correct_position, correct_depth) = self.run_correctly();

        writeln!(
            f,
            "Incorrect location: {}",
            incorrect_position * incorrect_depth
        )?;
        writeln!(f, "Correct location: {}", correct_position * correct_depth)?;

        Ok(())
    }
}

impl Course {
    fn run_incorrectly(&self) -> (i32, i32) {
        let mut position = 0;
        let mut depth = 0;

        for command in self.0.iter() {
            match command {
                Command::Up(unit) => depth -= unit,
                Command::Down(unit) => depth += unit,
                Command::Forward(unit) => position += unit,
            }
        }

        (position, depth)
    }

    fn run_correctly(&self) -> (i32, i32) {
        let mut position = 0;
        let mut aim = 0;
        let mut depth = 0;

        for command in self.0.iter() {
            match command {
                Command::Up(unit) => aim -= unit,
                Command::Down(unit) => aim += unit,
                Command::Forward(unit) => {
                    position += unit;
                    depth += unit * aim;
                }
            }
        }

        (position, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_course() -> Course {
        Course(vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ])
    }

    #[test]
    fn test_running_course_commands_incorrectly() {
        let course = sample_course();
        assert_eq!(course.run_incorrectly(), (15, 10));
    }

    #[test]
    fn test_running_course_commands_correctly() {
        let course = sample_course();
        assert_eq!(course.run_correctly(), (15, 60));
    }
}
