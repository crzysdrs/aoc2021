use crate::Day;
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::ops::Add;

#[derive(Clone, PartialEq)]
pub struct SnailFish(SnailFishVal, SnailFishVal);

impl std::fmt::Display for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl std::fmt::Debug for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl std::fmt::Display for SnailFishVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishVal::Val(v) => write!(f, "{}", v),
            SnailFishVal::Pair(v) => write!(f, "{}", v),
        }
    }
}

impl std::fmt::Debug for SnailFishVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishVal::Val(v) => write!(f, "{}", v),
            SnailFishVal::Pair(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SnailFishVal {
    Val(u32),
    Pair(Box<SnailFish>),
}

impl Add<Self> for SnailFish {
    type Output = SnailFish;
    fn add(self, rhs: SnailFish) -> Self::Output {
        let n = SnailFish(
            SnailFishVal::Pair(Box::new(self)),
            SnailFishVal::Pair(Box::new(rhs)),
        );
        n.reduce().last().unwrap()
    }
}

impl std::iter::Sum<Self> for SnailFish {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let start = iter.next().unwrap();
        let r = iter.fold(start, |mut state, rhs| {
            //println!("{:?} + {:?}", state, rhs);
            state = state + rhs.clone();
            //println!("= {:?}", state);
            state
        });
        r
    }
}

enum Action {
    Stop(SnailFishVal),
    Continue(SnailFishVal),
}

impl SnailFishVal {
    fn add_right(&mut self, val: u32) {
        match self {
            SnailFishVal::Val(b) => *b += val,
            SnailFishVal::Pair(sn) => {
                sn.1.add_right(val);
            }
        }
    }

    fn add_left(&mut self, val: u32) {
        match self {
            SnailFishVal::Val(b) => *b += val,
            SnailFishVal::Pair(sn) => {
                sn.0.add_left(val);
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailFishVal::Val(a) => *a as usize,
            SnailFishVal::Pair(sn) => sn.magnitude(),
        }
    }
}

struct SnailFishReduce {
    sn: Option<SnailFish>,
}
#[derive(Copy, Clone)]
enum Pass {
    Explode,
    Split,
}
impl Iterator for SnailFishReduce {
    type Item = SnailFish;
    fn next(&mut self) -> Option<Self::Item> {
        fn inner_reduce(
            pass: Pass,
            this: SnailFishVal,
            l: Option<&mut SnailFishVal>,
            r: Option<&mut SnailFishVal>,
            depth: usize,
        ) -> Action {
            match this {
                SnailFishVal::Pair(sn) => match *sn {
                    SnailFish(SnailFishVal::Val(a), SnailFishVal::Val(b)) if depth >= 4 => {
                        l.map(|l| l.add_right(a));
                        r.map(|r| r.add_left(b));
                        return Action::Stop(SnailFishVal::Val(0));
                    }
                    SnailFish(a, mut b) => {
                        let a = inner_reduce(pass, a, l, Some(&mut b), depth + 1);
                        let mut a = match a {
                            Action::Stop((v)) => {
                                return Action::Stop(
                                    (SnailFishVal::Pair(Box::new(SnailFish(v, b)))),
                                )
                            }
                            Action::Continue(v) => v,
                        };

                        let b = inner_reduce(pass, b, Some(&mut a), r, depth + 1);
                        let b = match b {
                            Action::Stop((v)) => {
                                return Action::Stop(
                                    (SnailFishVal::Pair(Box::new(SnailFish(a, v)))),
                                )
                            }
                            Action::Continue(v) => v,
                        };

                        return Action::Continue(SnailFishVal::Pair(Box::new(SnailFish(a, b))));
                    }
                },
                SnailFishVal::Val(a) if a >= 10 && matches!(pass, Pass::Split) => {
                    return Action::Stop(
                        (SnailFishVal::Pair(Box::new(SnailFish(
                            SnailFishVal::Val(a / 2),
                            SnailFishVal::Val(a / 2 + a % 2),
                        )))),
                    );
                }
                SnailFishVal::Val(_) => return Action::Continue(this),
            }
        }
        self.sn.take().map(|sn| {
            let mut this = SnailFishVal::Pair(Box::new(sn));
            //println!("Before {:?}", this);
            let reduced = inner_reduce(Pass::Explode, this, None, None, 0);
            let reduced = match reduced {
                Action::Continue(v) => inner_reduce(Pass::Split, v, None, None, 0),
                _ => reduced,
            };
            let cont = matches!(reduced, Action::Stop(_));
            let new = match reduced {
                Action::Continue(v) | Action::Stop(v) => match v {
                    SnailFishVal::Pair(s) => *s,
                    _ => panic!(),
                },
            };
            //println!("After {:?}", new);
            if cont {
                self.sn = Some(new.clone());
            }
            new
        })
    }
}
impl SnailFish {
    fn magnitude(&self) -> usize {
        match self {
            SnailFish(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
    fn reduce(self) -> impl Iterator<Item = SnailFish> {
        SnailFishReduce { sn: Some(self) }
    }
}

impl std::str::FromStr for SnailFish {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '[' | ',' => {}
                '0'..='9' => stack.push(SnailFishVal::Val(c.to_digit(10).unwrap())),
                ']' => {
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();
                    stack.push(SnailFishVal::Pair(Box::new(SnailFish(l, r))))
                }
                _ => panic!(),
            }
        }
        assert_eq!(stack.len(), 1);
        let top = stack.pop().unwrap();

        match top {
            SnailFishVal::Pair(sn) => Ok(*sn),
            _ => panic!(),
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 18;
    type Input = Vec<SnailFish>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .flatten()
            .map(|l| l.parse::<SnailFish>().unwrap())
            .collect())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let r: SnailFish = v.iter().cloned().sum();
        r.magnitude()
    }

    fn p2(v: &Self::Input) -> Self::Sol2 {
        v.iter()
            .cartesian_product(v.iter())
            .map(|(x, y)| (x.clone() + y.clone()).magnitude())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            "[[[[[9,8],1],2],3],4]"
                .parse::<SnailFish>()
                .unwrap()
                .reduce()
                .next()
                .unwrap(),
            "[[[[0,9],2],3],4]".parse::<SnailFish>().unwrap()
        );
        assert_eq!(
            "[7,[6,[5,[4,[3,2]]]]]"
                .parse::<SnailFish>()
                .unwrap()
                .reduce()
                .next()
                .unwrap(),
            "[7,[6,[5,[7,0]]]]".parse::<SnailFish>().unwrap()
        );
        assert_eq!(
            "[[6,[5,[4,[3,2]]]],1]"
                .parse::<SnailFish>()
                .unwrap()
                .reduce()
                .next()
                .unwrap(),
            "[[6,[5,[7,0]]],3]".parse::<SnailFish>().unwrap()
        );
        assert_eq!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
                .parse::<SnailFish>()
                .unwrap()
                .reduce()
                .next()
                .unwrap(),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<SnailFish>()
                .unwrap()
                .reduce()
                .next()
                .unwrap(),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );

        let input = "[1,1]\n\
[2,2]\n\
[3,3]\n\
[4,4]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(
            input.iter().cloned().sum::<SnailFish>(),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<SnailFish>()
                .unwrap()
        );

        let input = "[1,1]\n\
[2,2]\n\
[3,3]\n\
[4,4]\n\
[5,5]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(
            input.iter().cloned().sum::<SnailFish>(),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<SnailFish>()
                .unwrap()
        );

        let input = "[1,1]\n\
[2,2]\n\
[3,3]\n\
[4,4]\n\
[5,5]\n\
[6,6]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(
            input.iter().cloned().sum::<SnailFish>(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<SnailFish>()
                .unwrap()
        );

        assert_eq!(
            "[[1,2],[[3,4],5]]"
                .parse::<SnailFish>()
                .unwrap()
                .magnitude(),
            143
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailFish>()
                .unwrap()
                .magnitude(),
            3488
        );
        let l = "[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<SnailFish>()
            .unwrap();
        let r = "[1,1]".parse::<SnailFish>().unwrap();

        assert_eq!(l + r, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap());

        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
[7,[5,[[3,8],[1,4]]]]\n\
[[2,[2,2]],[8,[8,1]]]\n\
[2,9]\n\
[1,[[[9,3],9],[[9,0],[0,7]]]]\n\
[[[5,[7,4]],7],1]\n\
[[[[4,2],2],6],[8,7]]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(
            input.iter().take(2).cloned().sum::<SnailFish>(),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
        assert_eq!(
            input.iter().take(3).cloned().sum::<SnailFish>(),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
        assert_eq!(
            input.iter().take(4).cloned().sum::<SnailFish>(),
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
        assert_eq!(
            input.iter().take(5).cloned().sum::<SnailFish>(),
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );

        assert_eq!(
            input.iter().cloned().sum::<SnailFish>(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
        assert_eq!(Solution::p1(&input), 3488);

        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
[[[5,[2,8]],4],[5,[[9,9],0]]]\n\
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
[[[[5,4],[7,7]],8],[[8,3],8]]\n\
[[9,3],[[9,9],[6,[4,9]]]]\n\
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 4140);
        //assert_eq!(Solution::p2(&input), 26984457539);
        //unimplemented!()
    }
}
