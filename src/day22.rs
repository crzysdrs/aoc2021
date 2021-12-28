use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Cube {
    on: bool,
    x: Range<i64>,
    y: Range<i64>,
    z: Range<i64>,
}

fn intersect(s: &Range<i64>, o: &Range<i64>) -> Option<Range<i64>> {
    assert!(s.contains(&o.start) || o.contains(&s.start));
    let start = if s.contains(&o.start) {
        o.start
    } else {
        s.start
    };
    assert!(o.end == s.end || s.contains(&o.end) || o.contains(&s.end));
    let end = if s.contains(&o.end) { o.end } else { s.end };
    Some(start..end)
}

impl Cube {
    fn intersect(&self, other: &Cube) -> bool {
        self.x.start < other.x.end
            && self.x.end > other.x.start
            && self.y.start < other.y.end
            && self.y.end > other.y.start
            && self.z.start < other.z.end
            && self.z.end > other.z.start
    }

    fn volume(&self) -> usize {
        ((self.x.end - self.x.start) * (self.y.end - self.y.start) * (self.z.end - self.z.start))
            as usize
    }
    fn count(&self) -> usize {
        if self.on {
            self.volume()
        } else {
            0
        }
    }

    fn sub(self, other: &Cube) -> Vec<Cube> {
        fn ranges(outer: &Range<i64>, inner: &Range<i64>) -> Vec<Range<i64>> {
            [
                (outer.start..inner.start),
                (inner.start..inner.end),
                (inner.end..outer.end),
            ]
            .into_iter()
            .filter(|r| !r.is_empty())
            .collect::<Vec<_>>()
        }
        assert!(self.intersect(other) && other.intersect(&self));
        let mut cubes = vec![];
        for x in ranges(&self.x, &other.x) {
            for y in ranges(&self.y, &other.y) {
                for z in ranges(&self.z, &other.z) {
                    cubes.push(Cube {
                        on: self.on,
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    });
                }
            }
        }

        assert_eq!(
            cubes.iter().map(|x| x.volume()).sum::<usize>(),
            self.volume()
        );
        cubes.retain(|c| !(c.x == other.x && c.y == other.y && c.z == other.z));
        assert_eq!(
            cubes.iter().map(|x| x.volume()).sum::<usize>(),
            self.volume() - other.volume(),
        );
        cubes
    }
    fn intersection(&self, other: &Cube) -> Option<Cube> {
        if self.intersect(other) {
            let ranges = &[
                (&self.x, &other.x),
                (&self.y, &other.y),
                (&self.z, &other.z),
            ]
            .iter()
            .map(|(s, o)| intersect(s, o).unwrap())
            .collect::<Vec<_>>();

            Some(Cube {
                on: self.on,
                x: ranges[0].clone(),
                y: ranges[1].clone(),
                z: ranges[2].clone(),
            })
        } else {
            None
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 22;
    type Input = Vec<Cube>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .flatten()
            .map(|l| {
                let (on, suffix) = if let Some(suffix) = l.strip_prefix("on ") {
                    (true, suffix)
                } else {
                    (false, l.strip_prefix("off ").unwrap())
                };
                let mut nums = suffix.split(',').flat_map(move |v| {
                    v.chars()
                        .skip(2)
                        .collect::<String>()
                        .split("..")
                        .into_iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                });
                Cube {
                    on,
                    x: nums.next().unwrap()..(nums.next().unwrap() + 1),
                    y: nums.next().unwrap()..(nums.next().unwrap() + 1),
                    z: nums.next().unwrap()..(nums.next().unwrap() + 1),
                }
            })
            .collect())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        // // ORIGINAL PART 1 Solution
        // use cgmath::Point3;
        // let mut map = HashMap::new();
        // v.iter()
        //     .filter(|c| {
        //         [c.x.clone(), c.y.clone(), c.z.clone()]
        //             .into_iter()
        //             .all(|r| {
        //                 r.start >= -50 && r.start <= 50 && r.end >= -51 && r.end <= 51
        //             })
        //     })
        //     .for_each(|c| {
        //         c.x.clone()
        //             .cartesian_product(c.y.clone())
        //             .cartesian_product(c.z.clone())
        //             .map(|p| Point3::new(p.0 .0, p.0 .1, p.1))
        //             .for_each(|p| {
        //                 map.insert(p, c.on);
        //             })
        //     });

        // map.values().filter(|x| **x).count()

        let v = v
            .to_vec()
            .into_iter()
            .filter(|c| {
                [c.x.clone(), c.y.clone(), c.z.clone()]
                    .into_iter()
                    .all(|r| r.start >= -50 && r.start <= 50 && r.end >= -51 && r.end <= 51)
            })
            .collect();
        Self::p2(&v)
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let mut map: Vec<Cube> = vec![];
        let v = v.to_vec();
        for (s, c) in v.into_iter().enumerate() {
            println!("{:?}", s);
            if map.len() == 0 {
                map.push(c);
            } else {
                let mut cast = vec![c];
                while let Some(c) = cast.pop() {
                    let found = map
                        .iter()
                        .find_map(|x| if c.intersect(&x) { Some(x) } else { None })
                        .cloned();
                    if let Some(f) = found {
                        //println!("Found Intersection");
                        map.retain(|c| !(c.x == f.x && c.y == f.y && c.z == f.z));
                        //println!("{}", map.len());
                        let i = c.intersection(&f).unwrap();
                        //println!("Search {:?}", c);
                        //println!("{:?}", i);
                        cast.extend(c.sub(&i));
                        //println!("{:?}", cast);
                        map.extend(f.sub(&i));
                        map.push(i);
                        //println!("New Map {:?}", map.len());
                    } else {
                        map.push(c);
                    }
                }
            }
        }
        println!("{:?}", map.len());
        map.iter().map(|x| x.count()).sum::<usize>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "on x=-20..26,y=-36..17,z=-47..7\n\
on x=-20..33,y=-21..23,z=-26..28\n\
on x=-22..28,y=-29..23,z=-38..16\n\
on x=-46..7,y=-6..46,z=-50..-1\n\
on x=-49..1,y=-3..46,z=-24..28\n\
on x=2..47,y=-22..22,z=-23..27\n\
on x=-27..23,y=-28..26,z=-21..29\n\
on x=-39..5,y=-6..47,z=-3..44\n\
on x=-30..21,y=-8..43,z=-13..34\n\
on x=-22..26,y=-27..20,z=-29..19\n\
off x=-48..-32,y=26..41,z=-47..-37\n\
on x=-12..35,y=6..50,z=-50..-2\n\
off x=-48..-32,y=-32..-16,z=-15..-5\n\
on x=-18..26,y=-33..15,z=-7..46\n\
off x=-40..-22,y=-38..-28,z=23..41\n\
on x=-16..35,y=-41..10,z=-47..6\n\
off x=-32..-23,y=11..30,z=-14..3\n\
on x=-49..-5,y=-3..45,z=-29..18\n\
off x=18..30,y=-20..-8,z=-3..13\n\
on x=-41..9,y=-7..43,z=-33..15\n\
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
on x=967..23432,y=45373..81175,z=27513..53682";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 590784);

        let input = "on x=-5..47,y=-31..22,z=-19..33\n\
on x=-44..5,y=-27..21,z=-14..35\n\
on x=-49..-1,y=-11..42,z=-10..38\n\
on x=-20..34,y=-40..6,z=-44..1\n\
off x=26..39,y=40..50,z=-2..11\n\
on x=-41..5,y=-41..6,z=-36..8\n\
off x=-43..-33,y=-45..-28,z=7..25\n\
on x=-33..15,y=-32..19,z=-34..11\n\
off x=35..47,y=-46..-34,z=-11..5\n\
on x=-14..36,y=-6..44,z=-16..29\n\
on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
on x=36731..105352,y=-21140..28532,z=16094..90401\n\
on x=30999..107136,y=-53464..15513,z=8553..71215\n\
on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
on x=-726..56291,y=-62629..13224,z=18033..85226\n\
on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
on x=-57817..18248,y=49321..117703,z=5745..55881\n\
on x=14781..98692,y=-1341..70827,z=15753..70151\n\
on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
on x=16383..101554,y=4615..83635,z=-44907..18747\n\
off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
off x=2032..69770,y=-71013..4824,z=7471..94418\n\
on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
off x=18020..114017,y=-48931..32606,z=21474..89843\n\
off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
off x=-27365..46395,y=31009..98017,z=15428..76570\n\
off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&input), 2758514936282235);
        //unimplemented!()
    }
}
