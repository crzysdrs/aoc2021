use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input = Vec<u32>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|v| {
                Ok(v?.parse::<u32>().unwrap()
                )
            })
            .collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        v.windows(2)
            .map(|vs| vs[1] > vs[0])
            .filter(|v| *v)
            .count()
            
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let v = v.windows(3)
            .map(|vs| vs.iter().sum())
            .collect::<Vec<u32>>();
        v.windows(2)
            .map(|vs| vs[1] > vs[0])
            .filter(|v| *v)
            .count()
           
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "199\n\
                     200\n\
                     208\n\
                     210\n\
                     200\n\
                     207\n\
                     240\n\
                     269\n\
                     260\n\
                     263";
        
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 7);
        assert_eq!(Solution::p2(&input), 5);
    }
}
