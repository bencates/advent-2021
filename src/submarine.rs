use crate::course::Command;

#[derive(Default, Debug)]
pub struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn run_incorrectly(&mut self, course: &[Command]) {
        for command in course {
            match command {
                Command::Up(unit) => self.depth -= unit,
                Command::Down(unit) => self.depth += unit,
                Command::Forward(unit) => self.position += unit,
            }
        }
    }

    pub fn run_correctly(&mut self, course: &[Command]) {
        for command in course {
            match command {
                Command::Up(unit) => self.aim -= unit,
                Command::Down(unit) => self.aim += unit,
                Command::Forward(unit) => {
                    self.position += unit;
                    self.depth += unit * self.aim;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_COURSE: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_running_course_commands_incorrectly() {
        let mut submarine = Submarine::default();
        submarine.run_incorrectly(&SAMPLE_COURSE);

        assert_eq!(submarine.position(), 15);
        assert_eq!(submarine.depth(), 10);
    }

    #[test]
    fn test_running_course_commands_correctly() {
        let mut submarine = Submarine::default();
        submarine.run_correctly(&SAMPLE_COURSE);

        assert_eq!(submarine.position(), 15);
        assert_eq!(submarine.depth(), 60);
    }
}
