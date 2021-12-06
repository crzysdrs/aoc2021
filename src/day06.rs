use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input = Vec<usize>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .next()
            .unwrap()?
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let mut v = v.clone();
        for _ in 0..80 {
            let finished = v.iter().filter(|x| **x == 0).count();
            v.iter_mut().filter(|x| **x == 0).for_each(|x| *x = 7);
            v.iter_mut().for_each(|x| *x -= 1);
            v.extend((0..finished).map(|_| 8));
        }
        v.len()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        // Algorithm computes same as p1, but more cleverly.
        let mut v = v.clone();
        v.sort();
        let mut pop: Vec<usize> = vec![0; 9];
        for x in &v {
            pop[*x as usize] += 1;
        }
        for _ in 0..256 {
            let finished = pop[0];
            pop[7] += pop[0];
            pop.rotate_left(1);
            pop[8] = finished;
        }
        pop.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "3,4,3,1,2";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 5934);
        assert_eq!(Solution::p2(&input), 26984457539);
    }
}
