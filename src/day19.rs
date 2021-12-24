use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use cgmath::{Vector3, Point3};
use rayon::prelude::*;

#[derive(Debug,Clone)]
pub struct Scanner {
    idx: usize,
    offset: Vector3<i32>,
    pts: Vec<Point3<i32>>,
}

impl Scanner {
    fn permutes(&self) -> Vec<Vec<Point3<i32>>> {
        let all = self.pts.iter().map(|p| {
            let permutes = [
                [p.x, p.y, p.z],
                [p.x, p.z, p.y],
                [p.y, p.x, p.z],
                [p.y, p.z, p.x],
                [p.z, p.y, p.x],
                [p.z, p.x, p.y]
            ];

            let base : Vec<_> = permutes.into_iter().map(|p| Point3::from(p)).collect();
            let mut result = vec![];
            result.extend(base.iter().flat_map(|p| {
                [
                    Point3::from([p.x, p.y, p.z]),
                    Point3::from([p.x, p.y, -p.z]),
                    Point3::from([p.x, -p.y, p.z]),
                    Point3::from([p.x, -p.y, -p.z]),
                    Point3::from([-p.x, p.y, p.z]),
                    Point3::from([-p.x, p.y, -p.z]),
                    Point3::from([-p.x, -p.y, p.z]),
                    Point3::from([-p.x, -p.y, -p.z]),
                ].into_iter()                    
            }));
            result
        }).collect::<Vec<_>>();

        let mut done = vec![vec![Point3::from([0,0,0]); self.pts.len()]; 48];
        
        for (i, permuted) in all.iter().enumerate() {
            for (j, p) in permuted.iter().enumerate() {
                done[j][i] = *p;
            }
        }

        for i in 0..done.len() {
            done[i].sort_by_key(|p| (p.x, p.y, p.z));
        }
        done
    }
}

fn find_map(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut scanners = scanners.to_vec();        
    let mut map = vec![scanners.pop().unwrap()];
    use std::sync::RwLock;
    let mut checked = RwLock::new(HashSet::new());
    'map_done : loop {
        let found = map.par_iter().map(|s| {
            scanners.iter().map(|r| {
                let key = (s.idx, r.idx);
                if checked.read().unwrap().get(&key).is_some() {
                    return None;
                }
                let mut pts = s.pts.as_slice();
                let found = 'found : loop {
                    let permutes = r.permutes();
                    for p in pts {
                        for list in &permutes {
                            for p2 in list {
                                let offset = p2 - p;
                                let offsetted = pts.iter().map(|p| p + offset).collect::<Vec<_>>();
                                if list.iter().flat_map(|p| offsetted.iter().find(|p2| p == *p2)).count() >= 12 {
                                    break 'found Some(Scanner {
                                        pts: list.iter().map(|p| p - offset).collect(),
                                        offset,
                                        idx: r.idx,
                                    })
                                }
                            }
                        }
                    }
                    pts = &pts[1..];
                    if pts.len() < 12 {
                        checked.write().unwrap().insert(key);
                        break None;
                    }
                };
                found
            }).filter(|x| x.is_some()).next().flatten()
        }).find_first(|x| x.is_some()).flatten();
        
        if let Some(f) = found {
            scanners.retain(|x| x.idx != f.idx);
            println!("{:?}", f);
            map.push(f);
            if scanners.is_empty() {
                break 'map_done;
            }
        }
    }
    map
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 19;
    type Input = Vec<Scanner>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines();
        let mut scanners = vec![];
        let mut idx = 0;
        loop {
            if lines.by_ref().next().is_some() {
                let mut pts : Vec<_> = lines.by_ref().flatten().take_while(|l| !l.is_empty()).map(|l| {
                    let mut p = l.split(",").map(|p| p.parse::<i32>().unwrap());
                    Point3{ x: p.next().unwrap(), y: p.next().unwrap(), z: p.next().unwrap()}
                }).collect();
                pts.sort_by_key(|p| (p.x, p.y, p.z));
                scanners.push(Scanner {
                    idx,
                    offset: Vector3::from([0,0,0]),
                    pts
                });
                idx += 1;
            } else {
                break;
            }
        }
        Ok(scanners)
    }
    fn p1(scanners: &Self::Input) -> Self::Sol1 {
        let map = find_map(scanners);
        let mut all = map.into_iter().flat_map(|x| x.pts).collect::<Vec<_>>();
        all.sort_by_key(|p| (p.x, p.y, p.z));
        all.dedup();
        all.len()
    }
    fn p2(scanners: &Self::Input) -> Self::Sol2 {
        let map = find_map(scanners);
        let points = map.iter().map(|x| Point3::from([0,0,0]) + x.offset).collect::<Vec<_>>();
        let mut max = None;
        for p1 in &points {
            for p2 in & points {
                let dist = p2 - p1;
                let manhattan = dist.x.abs() + dist.y.abs() + dist.z.abs();
                if let Some(max_val) = max {
                    if max_val < manhattan {
                        max = Some(manhattan);
                    }
                } else {
                    max = Some(manhattan);
                }
            }
        }
        max.unwrap() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        use std::io::Read;
        let mut input = std::io::Cursor::new(std::fs::read("test/day19.txt").unwrap());
        let input = Solution::process_input(std::io::BufReader::new(&mut input)).unwrap();
        assert_eq!(Solution::p1(&input), 79);
        assert_eq!(Solution::p2(&input), 3621);
        //unimplemented!()
    }
}
