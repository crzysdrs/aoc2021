use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

fn get_val(arr: &Vec<Vec<u32>>, x: i32, y: i32) -> Option<&u32> {
    if x < 0 || y < 0 {
        return None;
    } else {
        arr.get(y as usize)
            .and_then(|row: &Vec<u32>| row.get(x as usize))
    }
}

#[allow(unused)]
fn print(arr: &Vec<Vec<u32>>) {
    for row in arr {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn octo_step(_n: usize, width: usize, v: &mut Vec<Vec<u32>>) -> usize {
    //println!("Step {}", n);
    //print(&v);
    v.iter_mut()
        .flat_map(|v| v.iter_mut())
        .for_each(|x| *x += 1);
    let mut step_flashed = HashSet::new();
    loop {
        let flashed = v
            .iter()
            .flat_map(|v| v.iter())
            .enumerate()
            .filter(|(_i, x)| **x > 9)
            .map(|(i, _)| ((i % width) as i32, (i / width) as i32))
            .filter(|(x, y)| !step_flashed.contains(&(*x, *y)))
            .collect::<Vec<_>>();

        for f in &flashed {
            step_flashed.insert((f.0, f.1));
            let mut adj = vec![];
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    } else {
                        adj.push((f.0 + x, f.1 + y));
                    }
                }
            }
            adj.iter().for_each(|(x, y)| {
                if get_val(&v, *x, *y).is_some() {
                    v[*y as usize][*x as usize] += 1;
                }
            })
        }

        if flashed.len() == 0 {
            break;
        }
    }
    step_flashed.iter().for_each(|(x, y)| {
        v[*y as usize][*x as usize] = 0;
    });
    step_flashed.len()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 11;
    type Input = Vec<Vec<u32>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                Ok(l?
                    .chars()
                    .map(|x| (x.to_string().parse().unwrap()))
                    .collect())
            })
            .collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let mut v = v.clone();
        let width = v.iter().map(|v| v.len()).max().unwrap();

        let step = 100;
        let mut total_flashes = 0;
        for n in 0..step {
            total_flashes += octo_step(n + 1, width, &mut v);
        }
        total_flashes
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let mut v = v.clone();
        let width = v.iter().map(|v| v.len()).max().unwrap();

        for n in 0.. {
            if octo_step(n, width, &mut v) == width * width {
                return n + 1;
            }
        }
        panic!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "5483143223\n\
2745854711\n\
5264556173\n\
6141336146\n\
6357385478\n\
4167524645\n\
2176841721\n\
6882881134\n\
4846848554\n\
5283751526";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 1656);
        assert_eq!(Solution::p2(&input), 195);
        //unimplemented!()
    }
}
