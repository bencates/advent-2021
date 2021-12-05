use std::str::FromStr;

pub struct Report {
    data: Vec<u32>,
    data_size: u32,
    gamma_rate: u32,
    epsilon_rate: u32,
    oxygen_generator_rating: u32,
    co2_scrubber_rating: u32,
}

impl Report {
    pub fn new(data: &[u32]) -> Report {
        let data_size = data
            .iter()
            .map(|num| u32::BITS - num.leading_zeros())
            .max()
            .unwrap_or(0);

        let gamma_rate = gamma_rate(data, data_size);
        let epsilon_rate = gamma_rate ^ u32::MAX >> u32::BITS - data_size;

        let oxygen_generator_rating = oxygen_generator_rating(data, 1 << data_size - 1);
        let co2_scrubber_rating = co2_scrubber_rating(data, 1 << data_size - 1);

        Report {
            data: data.to_vec(),
            data_size,
            gamma_rate,
            epsilon_rate,
            oxygen_generator_rating,
            co2_scrubber_rating,
        }
    }

    pub fn power_consumption(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }

    pub fn life_support_rating(&self) -> u32 {
        self.oxygen_generator_rating * self.co2_scrubber_rating
    }
}

fn gamma_rate(data: &[u32], data_size: u32) -> u32 {
    let half_len = data.len() / 2;

    (0..data_size)
        .map(|i| 1 << i)
        .filter(|&bitmask| data.iter().filter(|&num| num & bitmask != 0).count() >= half_len)
        .sum()
}

fn oxygen_generator_rating(data: &[u32], bitmask: u32) -> u32 {
    if data.len() == 1 {
        data[0]
    } else {
        let (zeros, ones): (Vec<u32>, Vec<u32>) = data.iter().partition(|&num| num & bitmask == 0);

        if ones.len() >= zeros.len() {
            oxygen_generator_rating(&ones[..], bitmask >> 1)
        } else {
            oxygen_generator_rating(&zeros[..], bitmask >> 1)
        }
    }
}

fn co2_scrubber_rating(data: &[u32], bitmask: u32) -> u32 {
    if data.len() == 1 {
        data[0]
    } else {
        let (zeros, ones): (Vec<u32>, Vec<u32>) = data.iter().partition(|&num| num & bitmask == 0);

        if ones.len() >= zeros.len() {
            co2_scrubber_rating(&zeros[..], bitmask >> 1)
        } else {
            co2_scrubber_rating(&ones[..], bitmask >> 1)
        }
    }
}

impl FromStr for Report {
    type Err = std::num::ParseIntError;

    fn from_str(report: &str) -> Result<Self, Self::Err> {
        let data = report
            .lines()
            .map(|line| u32::from_str_radix(line, 2))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(&data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[test]
    fn it_parses_the_file() {
        let report: Report =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
                .parse()
                .unwrap();

        assert_eq!(report.data, SAMPLE_DATA);
    }

    #[test]
    fn it_calculates_the_power_consumption() {
        let report = Report::new(&SAMPLE_DATA);

        assert_eq!(report.gamma_rate, 0b10110);
        assert_eq!(report.epsilon_rate, 0b01001);

        assert_eq!(report.power_consumption(), 198);
    }

    #[test]
    fn it_calculates_the_life_support_rating() {
        let report = Report::new(&SAMPLE_DATA);

        assert_eq!(report.oxygen_generator_rating, 0b10111);
        assert_eq!(report.co2_scrubber_rating, 0b01010);

        assert_eq!(report.life_support_rating(), 230);
    }
}
