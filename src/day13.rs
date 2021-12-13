use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub enum Fold {
    X(usize),
    Y(usize),
}

pub struct Manual {
    dots: Vec<(usize, usize)>,
    folds: Vec<Fold>,
}

impl Manual {
    fn folds(&self, folds: std::ops::Range<usize>) -> HashSet<(usize, usize)> {
        let mut paper = self.dots.iter().copied().collect::<HashSet<_>>();

        for f in &self.folds[folds] {
            match f {
                Fold::X(size) => {
                    paper = paper
                        .into_iter()
                        .map(|(x, y)| {
                            if x > *size {
                                (2 * *size - x, y)
                            } else {
                                (x, y)
                            }
                        })
                        .collect()
                }
                Fold::Y(size) => {
                    paper = paper
                        .into_iter()
                        .map(|(x, y)| {
                            if y > *size {
                                (x, 2 * *size - y)
                            } else {
                                (x, y)
                            }
                        })
                        .collect()
                }
            }
        }
        paper
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 13;
    type Input = Manual;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines();
        let dots = lines
            .by_ref()
            .map(|l| l.unwrap())
            .take_while(|l| l != "")
            .map(|l| {
                let (a, b) = l.split_once(',').unwrap();
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            })
            .collect();

        let folds = lines
            .map(|l| l.unwrap())
            .map(|l| {
                let suffix = l.strip_prefix("fold along ").unwrap();
                let (axis, value) = suffix.split_once("=").unwrap();
                let value = value.parse::<usize>().unwrap();
                match axis {
                    "x" => Fold::X(value),
                    "y" => Fold::Y(value),
                    _ => panic!(),
                }
            })
            .collect();

        Ok(Manual { dots, folds })
    }
    fn p1(manual: &Self::Input) -> Self::Sol1 {
        let paper = manual.folds(0..1);
        paper.iter().count()
    }
    fn p2(manual: &Self::Input) -> Self::Sol2 {
        let paper = manual.folds(0..manual.folds.len());

        let width = paper.iter().map(|(x, _y)| x).max().unwrap();
        let height = paper.iter().map(|(_x, y)| y).max().unwrap();

        for y in 0..=*height {
            for x in 0..=*width {
                print!("{}", if paper.contains(&(x, y)) { '#' } else { '.' })
            }
            println!()
        }
        panic!("Answer in text above.")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "6,10\n\
0,14\n\
9,10\n\
0,3\n\
10,4\n\
4,11\n\
6,0\n\
6,12\n\
4,1\n\
0,13\n\
10,12\n\
3,4\n\
3,0\n\
8,4\n\
1,10\n\
2,14\n\
8,10\n\
9,0\n\
\n\
fold along y=7\n\
fold along x=5";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 17);
        //assert_eq!(Solution::p2(&input), 26984457539);
        //unimplemented!()
    }
}
