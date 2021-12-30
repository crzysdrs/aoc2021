use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum Square {
    Right,
    Down,
    Empty,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c = match self {
            Square::Right => '>',
            Square::Down => 'v',
            Square::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Floor {
    width: usize,
    tiles: Vec<Square>,
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.tiles.chunks_exact(self.width).try_for_each(|c| {
            for x in c {
                write!(f, "{}", x)?;
            }
            writeln!(f, "")
        })
    }
}

impl std::fmt::Debug for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "\n{}", self)
    }
}

impl Floor {
    fn new(height: usize, width: usize) -> Floor {
        Floor {
            width,
            tiles: vec![Square::Empty; width * height],
        }
    }
    fn from(width: usize, tiles: Vec<Square>) -> Floor {
        assert_eq!(tiles.len() % width, 0);
        Floor { width, tiles }
    }
    fn tile(&self, p: &Point2<i32>) -> &Square {
        &self.tiles[self.offset(*p)]
    }
    fn tile_mut(&mut self, p: &Point2<i32>) -> &mut Square {
        let offset = self.offset(*p);
        &mut self.tiles[offset]
    }

    fn pt(&self, v: usize) -> Point2<i32> {
        Point2::new((v % self.width) as i32, (v / self.width) as i32)
    }
    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }
    fn offset(&self, p: Point2<i32>) -> usize {
        let x = ((p.x as usize % self.width) + self.width) % self.width;
        let y = ((p.y as usize % self.height()) + self.height()) % self.height();
        y * self.width + x
    }

    fn step(self) -> Floor {
        let mut new = Floor::new(self.height(), self.width);

        self.tiles.iter().enumerate().for_each(|(i, v)| match v {
            Square::Down => new.tiles[i] = Square::Down,
            Square::Right => {
                let target = self.pt(i) + Vector2::new(1, 0);
                if *self.tile(&target) == Square::Empty {
                    *new.tile_mut(&target) = *v;
                } else {
                    new.tiles[i] = *v;
                }
            }
            _ => {}
        });

        let new = new;
        let mut new_down = Floor::new(self.height(), self.width);

        new.tiles.iter().enumerate().for_each(|(i, v)| match v {
            Square::Right => new_down.tiles[i] = Square::Right,
            Square::Down => {
                let target = self.pt(i) + Vector2::new(0, 1);
                if *new.tile(&target) == Square::Empty {
                    *new_down.tile_mut(&target) = *v;
                } else {
                    new_down.tiles[i] = *v;
                }
            }
            _ => {}
        });

        new_down
    }

    fn iter(&self) -> FloorIter {
        FloorIter {
            floor: self.clone(),
        }
    }
}

struct FloorIter {
    floor: Floor,
}
impl Iterator for FloorIter {
    type Item = Floor;
    fn next(&mut self) -> Option<Self::Item> {
        self.floor = self.floor.clone().step();
        Some(self.floor.clone())
    }
}

impl std::str::FromStr for Floor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let tiles = s
            .lines()
            .flat_map(|l| {
                width = Some(l.chars().count());
                l.chars().collect::<Vec<_>>().into_iter().map(|x| match x {
                    '>' => Square::Right,
                    'v' => Square::Down,
                    '.' => Square::Empty,
                    _ => panic!(),
                })
            })
            .collect();
        Ok(Floor::from(width.unwrap(), tiles))
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 25;
    type Input = Floor;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(mut r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut buf = String::new();
        r.read_to_string(&mut buf).unwrap();
        Ok(buf.parse::<Floor>().unwrap())
    }

    fn p1(v: &Self::Input) -> Self::Sol1 {
        let floor = v.clone();
        let floor = floor.iter();

        let mut prev_floor = None;
        for (f, s) in floor.zip(1..) {
            if let Some(prev_floor) = prev_floor {
                if prev_floor == f {
                    return s;
                }
            }
            prev_floor = Some(f);
        }
        unreachable!()
    }
    fn p2(_v: &Self::Input) -> Self::Sol2 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let floor = "...>...\n\
                     .......\n\
                     ......>\n\
                     v.....>\n\
                     ......>\n\
                     .......\n\
                     ..vvv.."
            .parse::<Floor>()
            .unwrap();

        floor.tiles.iter().enumerate().for_each(|(i, _)| {
            assert_eq!(floor.pt(i), floor.pt(floor.offset(floor.pt(i))));
        });

        let mut floor = floor.iter();
        assert_eq!(
            floor.next().unwrap(),
            "..vv>..\n\
             .......\n\
             >......\n\
             v.....>\n\
             >......\n\
             .......\n\
             ....v.."
                .parse()
                .unwrap()
        );
        assert_eq!(
            floor.next().unwrap(),
            "....v>.\n\
             ..vv...\n\
             .>.....\n\
             ......>\n\
             v>.....\n\
             .......\n\
             ......."
                .parse()
                .unwrap()
        );
        assert_eq!(
            floor.next().unwrap(),
            "......>\n\
             ..v.v..\n\
             ..>v...\n\
             >......\n\
             ..>....\n\
             v......\n\
             ......."
                .parse()
                .unwrap()
        );
        assert_eq!(
            floor.next().unwrap(),
            ">......\n\
             ..v....\n\
             ..>.v..\n\
             .>.v...\n\
             ...>...\n\
             .......\n\
             v......"
                .parse()
                .unwrap()
        );
        let input = "v...>>.vv>\n\
                     .vv>>.vv..\n\
                     >>.>v>...v\n\
                     >>v>>.>.v.\n\
                     v>v.vv.v..\n\
                     >.>>..v...\n\
                     .vv..>.>v.\n\
                     v.v..>>v.v\n\
                     ....v..v.>";
        let floor = input.parse::<Floor>().unwrap();
        assert_eq!(floor.width, 10);
        assert_eq!(floor.height(), 9);
        let test_floor = floor.iter().skip(0).next().unwrap();
        assert_eq!(test_floor.width, 10);
        assert_eq!(test_floor.height(), 9);

        assert_eq!(
            floor.iter().skip(0).next().unwrap(),
            "....>.>v.>\n\
             v.v>.>v.v.\n\
             >v>>..>v..\n\
             >>v>v>.>.v\n\
             .>v.v...v.\n\
             v>>.>vvv..\n\
             ..v...>>..\n\
             vv...>>vv.\n\
             >.v.v..v.v"
                .parse()
                .unwrap()
        );

        assert_eq!(
            floor.iter().skip(9).next().unwrap(),
            "..>..>>vv.\n\
             v.....>>.v\n\
             ..v.v>>>v>\n\
             v>.>v.>>>.\n\
             ..v>v.vv.v\n\
             .v.>>>.v..\n\
             v.v..>v>..\n\
             ..v...>v.>\n\
             .vv..v>vv."
                .parse()
                .unwrap()
        );

        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();

        assert_eq!(Solution::p1(&input), 58);
        //unimplemented!()
    }
}
