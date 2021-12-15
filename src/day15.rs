use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 15;
    type Input = Vec<Vec<usize>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .flat_map(|l| l)
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>())
    }
    fn p1(input: &Self::Input) -> Self::Sol1 {
        let mut q: HashSet<(usize, usize)> = HashSet::new();
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                q.insert((x, y));
            }
        }

        dist.insert((0, 0), 0);

        while !q.is_empty() {
            //println!("{:?}", q.len());
            let u = q
                .iter()
                .filter(|k| dist.get(k).is_some())
                .min_by_key(|k| *dist.get(k).unwrap())
                .unwrap();
            let u = u.clone();
            q.remove(&u);

            let offsets = &[
                (Some(u.0), u.1.checked_sub(1)),
                (Some(u.0), Some(u.1 + 1)),
                (u.0.checked_sub(1), Some(u.1)),
                (Some(u.0 + 1), Some(u.1)),
            ];
            for o in offsets {
                match o {
                    (Some(x), Some(y)) if *x < input.len() && *y < input.len() => {
                        let v = (*x, *y);
                        if q.contains(&v) {
                            let alt = dist.get(&u).unwrap() + input[v.1][v.0];
                            if dist.get(&v).is_none() || alt < dist[&v] {
                                *dist.entry(v).or_insert(alt) = alt;
                                *prev.entry(v).or_insert(u) = u;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let target = (input.len() - 1, input.len() - 1);
        let mut pos = target;
        let mut path = vec![];
        while let Some(prev) = prev.get(&pos) {
            path.push(pos);
            pos = *prev;
        }

        path.reverse();
        println!("{:?}", path);
        // println!("{:?}", dist);
        // path.iter().map(|v| dist.get(v).unwrap()).sum()
        *dist.get(&target).unwrap()
    }
    fn p2(orig_input: &Self::Input) -> Self::Sol2 {
        let mut input = orig_input.clone();

        for i in 1..5 {
            input
                .iter_mut()
                .zip(orig_input.iter())
                .for_each(|(v1, v2)| {
                    v1.extend(v2.iter().map(|r| {
                        let new = r + i;
                        if new > 9 {
                            (new - 10) % 9 + 1
                        } else {
                            new
                        }
                    }))
                });
        }

        let mut new_input = input.clone();
        for i in 1..5 {
            let mut input = input.clone();
            input.iter_mut().for_each(|v1| {
                v1.iter_mut().for_each(|r| {
                    let new = *r + i;
                    *r = if new > 9 { (new - 10) % 9 + 1 } else { new }
                })
            });
            new_input.extend(input.into_iter())
        }

        //     for y in 0..new_input.len() {
        //     for x in 0..new_input.len() {
        //         print!("{:?}", new_input[y][x]);
        //     }
        //     println!();
        // }

        let input = new_input;

        let mut q: HashSet<(usize, usize)> = HashSet::new();
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                q.insert((x, y));
            }
        }

        dist.insert((0, 0), 0);

        let target = (input.len() - 1, input.len() - 1);

        while !q.is_empty() {
            // This is the slow operation, it should be a min heap or priority queue.
            let u = q
                .iter()
                .filter(|k| dist.get(k).is_some())
                .min_by_key(|k| (*dist.get(k).unwrap(), (target.0 - k.0 + target.1 - k.1)));
            if u.is_none() {
                break;
            };
            let u = u.unwrap();
            let u = u.clone();

            q.remove(&u);

            let offsets = &[
                (Some(u.0), u.1.checked_sub(1)),
                (Some(u.0), Some(u.1 + 1)),
                (u.0.checked_sub(1), Some(u.1)),
                (Some(u.0 + 1), Some(u.1)),
            ];
            for o in offsets {
                match o {
                    (Some(x), Some(y)) if *x < input.len() && *y < input.len() => {
                        let v = (*x, *y);
                        if q.contains(&v) {
                            let alt = dist.get(&u).unwrap() + input[v.1][v.0];
                            if dist.get(&v).is_none() || alt < dist[&v] {
                                *dist.entry(v).or_insert(alt) = alt;
                                *prev.entry(v).or_insert(u) = u;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut pos = target;
        let mut path = vec![];
        while let Some(prev) = prev.get(&pos) {
            path.push(pos);
            pos = *prev;
        }

        //path.reverse();
        //println!("{:?}", path);
        // println!("{:?}", dist);
        // path.iter().map(|v| dist.get(v).unwrap()).sum()
        *dist.get(&target).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "1163751742\n\
1381373672\n\
2136511328\n\
3694931569\n\
7463417111\n\
1319128137\n\
1359912421\n\
3125421639\n\
1293138521\n\
2311944581";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 40);
        assert_eq!(Solution::p2(&input), 315);
        //unimplemented!()
    }
}
