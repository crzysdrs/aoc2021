use crate::Day;
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Players {
    p1: usize,
    p2: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Player {
    space: usize,
    score: usize,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 21;
    type Input = Players;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines().flatten();
        let p1 = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let p2 = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Ok(Players {
            p1: p1 - 1,
            p2: p2 - 1,
        })
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let mut players = [
            Player {
                score: 0,
                space: v.p1,
            },
            Player {
                score: 0,
                space: v.p2,
            },
        ];

        let mut dice = (1usize..101).cycle();
        let mut die_rolls = 0;
        'done: loop {
            for mut p in &mut players {
                let d = dice.by_ref().take(3).sum::<usize>();
                die_rolls += 3;
                p.space += d;
                p.space %= 10;

                p.score += p.space + 1;
                if p.score >= 1000 {
                    break 'done;
                }
            }
        }

        let lose = players.iter().find(|p| p.score < 1000).unwrap();
        lose.score * die_rolls
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let cart = (1usize..=3)
            .cartesian_product(1..=3)
            .cartesian_product(1..=3)
            .map(|r| r.0 .0 + r.0 .1 + r.1)
            .collect::<Vec<_>>();

        let cart = cart.iter().fold(HashMap::new(), |mut state, n| {
            *state.entry(*n).or_insert(0) += 1;
            state
        });

        fn run(
            cart: &HashMap<usize, usize>,
            mut players: (Player, Player),
            turn: usize,
            roll: usize,
        ) -> (usize, usize) {
            let pick = turn % 2 == 0;
            let mut p = if pick { &mut players.0 } else { &mut players.1 };

            p.space += roll;
            p.space %= 10;
            p.score += p.space + 1;

            let final_wins = if p.score >= 21 {
                if pick {
                    (1, 0)
                } else {
                    (0, 1)
                }
            } else {
                let mut new = (0, 0);
                for (r, c) in cart.iter() {
                    let last = run(cart, players.clone(), turn + 1, *r);
                    new.0 += last.0 * *c;
                    new.1 += last.1 * *c;
                }
                new
            };
            final_wins
        }
        let players = (
            Player {
                score: 0,
                space: v.p1,
            },
            Player {
                score: 0,
                space: v.p2,
            },
        );
        let mut new = (0, 0);
        for (r, c) in cart.iter() {
            let last = run(&cart, players.clone(), 0, *r);
            new.0 += last.0 * *c;
            new.1 += last.1 * *c;
        }
        //println!("{:?}", new);
        if new.0 > new.1 {
            new.0
        } else {
            new.1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "Player 1 starting position: 4\n\
Player 2 starting position: 8";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 739785);
        assert_eq!(Solution::p2(&input), 444356092776315);
        //unimplemented!()
    }
}
