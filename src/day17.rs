use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::ops::RangeInclusive;

use regex::Regex;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 17;
    type Input = (RangeInclusive<i32>, RangeInclusive<i32>);
    type Sol1 = i32;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let re = Regex::new(r"(-?[0-9]+)\.\.(-?[0-9]+)").unwrap();

        let l = r.lines().flatten().next().unwrap();

        let m = re.captures_iter(&l).collect::<Vec<_>>();

        Ok((
            m[0][1].parse::<i32>().unwrap()..=m[0][2].parse::<i32>().unwrap(),
            m[1][1].parse::<i32>().unwrap()..=m[1][2].parse::<i32>().unwrap(),
        ))
    }
    fn p1(input: &Self::Input) -> Self::Sol1 {
        let mut max = 0;

        let mut x_vels = vec![];
        let mut y_vels = vec![];

        for x in input.0.clone() {
            for y in input.1.clone() {
                for n in 1..10000 {
                    if n % 2 == 0 {
                        continue;
                    }
                    if x % n == 0 {
                        let vel_x = (n + 1) / 2 + x / n;
                        x_vels.push(vel_x);
                    }
                    if y % n == 0 {
                        let vel_y = (n + 1) / 2 + y / n;
                        y_vels.push(vel_y);
                        y_vels.push(-vel_y);
                    }
                }
            }
        }

        let max_y = y_vels.iter().copied().max();
        for vel_x in x_vels.iter().copied() {
            for vel_y in max_y {
                let init_v = (vel_x, vel_y);
                let mut v = init_v;
                let mut pos = (0, 0);
                let maybe_max = (0..1000)
                    .map(|_s| {
                        pos.0 += v.0;
                        pos.1 += v.1;

                        if v.0 > 0 {
                            v.0 -= 1;
                        } else if v.0 < 0 {
                            v.0 += 1;
                        }
                        v.1 -= 1;

                        pos
                    })
                    .take_while(|pos| !(input.0.contains(&pos.0) && input.1.contains(&pos.1)))
                    .max_by_key(|pos| pos.1);

                if input.0.contains(&pos.0) && input.1.contains(&pos.1) {
                    let new_y = maybe_max.map(|m| m.1).unwrap_or(0);
                    if new_y > max {
                        max = new_y;
                    }
                }
            }
        }

        max
    }
    fn p2(input: &Self::Input) -> Self::Sol2 {
        let mut x_vels = vec![];
        let mut y_vels = vec![];

        for x in input.0.clone() {
            for y in input.1.clone() {
                for n in 1..1000 {
                    if n % 2 == 0 {
                        continue;
                    }
                    if x % n != 0 {
                        let vel_x = (n + 1) / 2 + x / n;
                        x_vels.push(vel_x);
                    }
                    if y % n != 0 {
                        let vel_y = (n + 1) / 2 + y / n;
                        y_vels.push(vel_y);
                        y_vels.push(-vel_y);
                    }
                }
            }
        }

        x_vels.sort();
        x_vels.dedup();

        y_vels.sort();
        y_vels.dedup();

        let mut hits = vec![];
        for vel_x in x_vels.iter().copied() {
            for vel_y in y_vels.iter().copied() {
                let init_v = (vel_x, vel_y);
                let mut v = init_v;
                let mut pos = (0, 0);
                (0..1000)
                    .map(|_s| {
                        pos.0 += v.0;
                        pos.1 += v.1;

                        if v.0 > 0 {
                            v.0 -= 1;
                        } else if v.0 < 0 {
                            v.0 += 1;
                        }
                        v.1 -= 1;

                        pos
                    })
                    .take_while(|pos| !(input.0.contains(&pos.0) && input.1.contains(&pos.1)))
                    .for_each(|_| {});

                if input.0.contains(&pos.0) && input.1.contains(&pos.1) {
                    hits.push(init_v);
                }
            }
        }
        //println!("{:?}", hits);
        hits.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "target area: x=20..30, y=-10..-5";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 45);
        assert_eq!(Solution::p2(&input), 112);
    }
}
