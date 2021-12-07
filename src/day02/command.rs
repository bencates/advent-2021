use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let mut cmd = cmd.split_whitespace();

        let action = cmd.next().ok_or(ParseError())?;
        let unit = cmd.next().ok_or(ParseError())?;

        let unit: i32 = unit.parse()?;

        match action {
            "up" => Ok(Command::Up(unit)),
            "down" => Ok(Command::Down(unit)),
            "forward" => Ok(Command::Forward(unit)),
            _ => Err(ParseError()),
        }
    }
}

#[derive(Debug)]
pub struct ParseError();

impl From<std::num::ParseIntError> for ParseError {
    fn from(_error: std::num::ParseIntError) -> Self {
        Self()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_COURSE: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_parsing_course_commands() {
        let commands: Vec<Command> = SAMPLE_COURSE.iter().map(|c| c.parse().unwrap()).collect();

        assert_eq!(
            commands,
            [
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]
        )
    }
}
