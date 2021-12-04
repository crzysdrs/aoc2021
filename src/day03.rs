use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 3;
    type Input = Vec<String>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines().map(|l| Ok(l?.to_string())).collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let len = v.len();
        let mut gamma = "".to_string();
        let bits = v[0].len();
        for i in 0..bits {
            let count = v
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .count();
            println!("{} {}", count, len);
            if count >= len - count {
                gamma.push_str("1");
            } else {
                gamma.push_str("0");
            }
        }
        let gamma = usize::from_str_radix(&gamma, 2).unwrap();
        println!("gamma: {} {:b}", gamma, gamma);
        let epsilon = (!gamma) & ((1 << bits) - 1);
        println!("epsilon {} {:b}", epsilon, epsilon);
        epsilon * gamma
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        fn filter_common(v: &Vec<String>, bits: usize, most: bool) -> usize {
            let mut v = v.clone();
            for i in 0..bits {
                let count = v
                    .iter()
                    .filter(|x| x.chars().nth(i).unwrap() == '1')
                    .count();
                println!("{} {}", count, v.len());
                let len = v.len();
                let bit = if count >= len - count { most } else { !most };
                let bit = if bit { '1' } else { '0' };
                v.retain(|x| x.chars().nth(i).unwrap() == bit);
                if v.len() == 1 {
                    break;
                }
            }
            assert_eq!(v.len(), 1);
            usize::from_str_radix(&v[0], 2).unwrap()
        }
        let bits = v[0].len();
        let o2 = filter_common(v, bits, true);
        let co2 = filter_common(v, bits, false);
        println!("o2: {}, co2 {}", o2, co2);
        o2 * co2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "00100\n\
11110\n\
10110\n\
10111\n\
10101\n\
01111\n\
00111\n\
11100\n\
10000\n\
11001\n\
00010\n\
01010";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 198);
        //assert_eq!(Solution::p2(&input), 230);
        //unimplemented!()
    }
}
