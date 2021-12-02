use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub enum Dir {
    Forward,
    Down,
    Up,
}

pub struct Cmd {
    dir: Dir,
    val: i32,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input = Vec<Cmd>;
    type Sol1 = i32;
    type Sol2 = i32;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                let mut split = l.split(" ");

                let dir = match split.next().unwrap() {
                    "forward" => Dir::Forward,
                    "down" => Dir::Down,
                    "up" => Dir::Up,
                    _ => panic!(),
                };
                let v = split.next().unwrap().parse::<i32>().unwrap();
                Ok(Cmd { dir, val: v })
            })
            .collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let mut x = 0;
        let mut y = 0;
        v.iter().for_each(|cmd| {
            match cmd.dir {
                Dir::Up => y -= cmd.val,
                Dir::Down => y += cmd.val,
                Dir::Forward => x += cmd.val,
            }
            if y < 0 {
                y = 0;
            }
        });

        x * y
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let mut aim = 0;
        let mut x = 0;
        let mut y = 0;
        v.iter().for_each(|cmd| {
            match cmd.dir {
                Dir::Up => {
                    aim -= cmd.val;
                }
                Dir::Down => {
                    aim += cmd.val;
                }
                Dir::Forward => {
                    x += cmd.val;
                    y += aim * cmd.val;
                }
            }
            if y < 0 {
                y = 0;
            }
        });

        x * y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "forward 5\n\
                 down 5\n\
                 forward 8\n\
                 up 3\n\
                 down 8\n\
                 forward 2";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 150);
        assert_eq!(Solution::p2(&input), 900);
    }
}
