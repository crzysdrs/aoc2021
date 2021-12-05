use crate::Day;
use cgmath::Point2;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 5;
    type Input = Vec<(Point2<i32>, Point2<i32>)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                let v = l
                    .split(" -> ")
                    .flat_map(|x| x.split(',').map(|x| x.parse().unwrap()))
                    .collect::<Vec<_>>();
                Ok((Point2::new(v[0], v[1]), Point2::new(v[2], v[3])))
            })
            .collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let mut h = HashMap::new();
        v.iter()
            .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
            .for_each(|(p1, p2)| {
                let min_x = std::cmp::min(p1.x, p2.x);
                let max_x = std::cmp::max(p1.x, p2.x);
                let min_y = std::cmp::min(p1.y, p2.y);
                let max_y = std::cmp::max(p1.y, p2.y);
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        println!("{} {}", x, y);
                        h.entry(Point2::new(x, y))
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                }
            });

        h.iter().filter(|(_, v)| **v > 1).count()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let mut h = HashMap::new();
        v.iter().for_each(|(p1, p2)| {
            let mut v = p1 - p2;
            v.x = if v.x == 0 { 0 } else { v.x / v.x.abs() };
            v.y = if v.y == 0 { 0 } else { v.y / v.y.abs() };

            let mut cur = p2.clone();
            let end = p1;
            loop {
                h.entry(cur).and_modify(|v| *v += 1).or_insert(1);
                if cur == *end {
                    break;
                }
                cur += v;
            }
        });

        h.iter().filter(|(_, v)| **v > 1).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "0,9 -> 5,9\n\
8,0 -> 0,8\n\
9,4 -> 3,4\n\
2,2 -> 2,1\n\
7,0 -> 7,4\n\
6,4 -> 2,0\n\
0,9 -> 2,9\n\
3,4 -> 1,4\n\
0,0 -> 8,8\n\
5,5 -> 8,2";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 5);
        assert_eq!(Solution::p2(&input), 12);
    }
}
