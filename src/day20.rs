use crate::Day;
use cgmath::Point2;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Clone)]
pub struct Enhance {
    enhance: Vec<bool>,
    img: HashMap<Point2<i32>, bool>,
}

fn points(p: Point2<i32>) -> Vec<Point2<i32>> {
    [
        (p.x - 1, p.y - 1),
        (p.x, p.y - 1),
        (p.x + 1, p.y - 1),
        (p.x - 1, p.y),
        (p.x, p.y),
        (p.x + 1, p.y),
        (p.x - 1, p.y + 1),
        (p.x, p.y + 1),
        (p.x + 1, p.y + 1),
    ]
    .into_iter()
    .map(|(x, y)| Point2::new(x, y))
    .collect()
}
fn offset(eh: &Enhance, p: Point2<i32>, s: usize) -> bool {
    let outside = match (s % 2 == 1, eh.enhance[0]) {
        (_, false) => false,
        (true, true) => true,
        (false, true) => false,
    };

    let offset = points(p)
        .into_iter()
        .map(|p| eh.img.get(&p).unwrap_or(&outside))
        .fold(0, |mut state, p| {
            state <<= 1;
            if *p {
                state |= 1;
            }
            state
        });

    eh.enhance[offset]
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 20;
    type Input = Enhance;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines().flatten();
        let enhance = lines.next().unwrap();
        let enhance: Vec<_> = enhance.chars().map(|x| x == '#').collect();

        let img = lines
            .skip(1)
            .enumerate()
            .flat_map(move |(y, l)| {
                l.chars()
                    .map(|x| x == '#')
                    .enumerate()
                    //.filter(|(_, v)| *v)
                    .map(move |(x, v)| (Point2::new(x as i32, y as i32), v))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect();

        assert_eq!(enhance.len(), 512);
        Ok(Enhance { enhance, img })
    }
    fn p1(eh: &Self::Input) -> Self::Sol1 {
        let mut eh = eh.clone();
        for s in 0..2 {
            //assert!(eh.img.values().all(|x| *x));
            let new: HashMap<_, _> = eh
                .img
                .keys()
                .flat_map(|p| points(*p))
                .map(|p| (p, offset(&eh, p, s)))
                .collect();

            eh.img = new;
        }
        //assert!(eh.img.values().all(|x| *x));
        eh.img.values().filter(|x| **x).count()
    }
    fn p2(eh: &Self::Input) -> Self::Sol2 {
        let mut eh = eh.clone();
        for s in 0..50 {
            //assert!(eh.img.values().all(|x| *x));
            let new: HashMap<_, _> = eh
                .img
                .keys()
                .flat_map(|p| points(*p))
                .map(|p| (p, offset(&eh, p, s)))
                .collect();

            eh.img = new;
        }
        //assert!(eh.img.values().all(|x| *x));
        eh.img.values().filter(|x| **x).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
\n\
#..#.\n\
#....\n\
##..#\n\
..#..\n\
..###";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 35);
        assert_eq!(Solution::p2(&input), 3351);
        //unimplemented!()
    }
}
