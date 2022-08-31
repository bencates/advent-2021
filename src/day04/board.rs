use std::str::FromStr;

#[derive(Debug)]
pub struct Board([[u8; 5]; 5]);

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>; // FIXME

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rows = [[0; 5]; 5];

        for (row, line) in rows.iter_mut().zip(input.lines()) {
            for (square, num) in row.iter_mut().zip(line.split_whitespace()) {
                *square = num.parse()?;
            }
        }

        Ok(Self(rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_board() {
        let board: Board =
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19"
                .parse()
                .unwrap();

        assert_eq!(
            board.0,
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ]
        );
    }
}
