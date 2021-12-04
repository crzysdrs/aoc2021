use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Debug, Clone)]
pub struct Bingo {
    input: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone)]
pub struct Board {
    squares: Vec<Vec<u32>>,
}

impl Board {
    fn is_bingo(&self, input: &[u32]) -> bool {
        let any_row = self
            .squares
            .iter()
            .any(|v| v.iter().all(|v| input.contains(&v)));
        let any_col =
            (0..5).any(|col| (0..5).all(|col_i| input.contains(&self.squares[col_i][col])));
        any_row || any_col
    }
    fn unmarked(&self, input: &[u32]) -> u32 {
        self.squares
            .iter()
            .flat_map(|v| v.iter())
            .filter(|x| !input.contains(&x))
            .sum()
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 4;
    type Input = Bingo;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines();
        let input: Vec<_> = lines
            .by_ref()
            .next()
            .unwrap()?
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        lines.next();

        let mut boards = vec![];
        loop {
            let board: Vec<Vec<_>> = lines
                .by_ref()
                .take(5)
                .map(|x| {
                    x.unwrap()
                        .split_whitespace()
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();

            boards.push(Board { squares: board });
            if lines.next().is_none() {
                break;
            }
        }

        Ok(Bingo { input, boards })
    }
    fn p1(bingo: &Self::Input) -> Self::Sol1 {
        for i in 0..bingo.input.len() {
            for b in &bingo.boards {
                let input = &bingo.input[..i];
                if b.is_bingo(input) {
                    return b.unmarked(input) * input.iter().last().unwrap();
                }
            }
        }
        panic!("Never had a bingo")
    }
    fn p2(bingo: &Self::Input) -> Self::Sol2 {
        let mut boards = bingo.boards.clone();
        for i in 0..bingo.input.len() {
            let input = &bingo.input[..i];
            if boards.len() == 1 && boards[0].is_bingo(&input) {
                return boards[0].unmarked(input) * input.iter().last().unwrap();
            }
            boards.retain(|b| !b.is_bingo(&input));
        }
        panic!("No last board found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
\n\
22 13 17 11  0\n\
 8  2 23  4 24\n\
21  9 14 16  7\n\
 6 10  3 18  5\n\
 1 12 20 15 19\n\
\n\
 3 15  0  2 22\n\
 9 18 13 17  5\n\
19  8  7 25 23\n\
20 11 10 24  4\n\
14 21 16 12  6\n\
\n\
14 21 17 24  4\n\
10 16 15  9 19\n\
18  8 23 26 20\n\
22 11 13  6  5\n\
2  0 12  3  7";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 4512);
        assert_eq!(Solution::p2(&input), 1924);
    }
}
