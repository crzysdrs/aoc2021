use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input = Vec<i32>;
    type Sol1 = i32;
    type Sol2 = i32;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .next()
            .unwrap()?
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let max = *v.iter().max().unwrap();
        let min = *v.iter().min().unwrap();

        let min = (min..max)
            .map(|i| (i, v.iter().map(|x| (x - i).abs()).sum()))
            .min_by_key(|(_, s)| *s)
            .unwrap();

        min.1
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let max = *v.iter().max().unwrap();
        let min = *v.iter().min().unwrap();

        let min = (min..max)
            .map(|i| {
                (
                    i,
                    v.iter()
                        .map(|x| {
                            let n = (x - i).abs();
                            n * (n + 1) / 2
                        })
                        .sum(),
                )
            })
            .min_by_key(|(_, s)| *s)
            .unwrap();

        min.1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 37);
        assert_eq!(Solution::p2(&input), 206);
        //unimplemented!()
    }
}
