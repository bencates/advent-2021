mod board;

use std::str::FromStr;

struct Bingo {
    hopper: std::vec::IntoIter<u8>,
}

impl FromStr for Bingo {
    type Err = std::num::ParseIntError;

    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let hopper_str = game.lines().next().unwrap();
        let hopper_data: Vec<u8> = hopper_str
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        let hopper = hopper_data.into_iter();

        Ok(Self { hopper })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn it_builds_the_hopper() {
        let bingo: Bingo = SAMPLE_INPUT.parse().unwrap();
        let numbers: Vec<u8> = bingo.hopper.take(5).collect();

        assert_eq!(numbers, [7, 4, 9, 5, 11]);
    }
}
