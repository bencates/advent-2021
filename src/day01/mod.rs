use std::{fmt, num::ParseIntError, str::FromStr};

pub struct Sonar(Vec<u32>);

impl FromStr for Sonar {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let soundings = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(soundings))
    }
}

impl fmt::Display for Sonar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Simple depth count: {}", depth_increase(&self.0, 1))?;
        writeln!(f, "Windowed depth count: {}", depth_increase(&self.0, 3))?;

        Ok(())
    }
}

pub fn depth_increase(soundings: &[u32], window_size: usize) -> usize {
    let sums: Vec<u32> = soundings
        .windows(window_size)
        .map(|group| group.iter().sum())
        .collect();

    sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn depth_increase_count_matches_sample() {
        assert_eq!(depth_increase(&SAMPLE_DATA, 1), 7);
    }

    #[test]
    fn depth_increase_with_window_matches_sample() {
        assert_eq!(depth_increase(&SAMPLE_DATA, 3), 5);
    }
}
