use crate::Day;
use cgmath::{Point2, Vector2};
use itertools::Itertools;
use petgraph::algo::dijkstra::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
#[allow(unused_imports)]
use std::collections::*;
use std::fmt::Display;
use std::io::Result as IoResult;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub enum Amp {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amp {
    fn energy(&self) -> usize {
        match self {
            Amp::Amber => 1,
            Amp::Bronze => 10,
            Amp::Copper => 100,
            Amp::Desert => 1000,
        }
    }
}

impl FromStr for Amp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amp = match s {
            "A" => Amp::Amber,
            "B" => Amp::Bronze,
            "C" => Amp::Copper,
            "D" => Amp::Desert,
            _ => return Err(()),
        };
        Ok(amp)
    }
}

impl std::fmt::Display for Amp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let a = match self {
            Amp::Amber => 'A',
            Amp::Bronze => 'B',
            Amp::Copper => 'C',
            Amp::Desert => 'D',
        };
        write!(f, "{}", a)
    }
}

#[derive(Debug, Clone)]
pub enum Space {
    IllegalFloor,
    Floor(Option<Amp>),
    Room(Option<Amp>),
    Wall,
}
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            Space::IllegalFloor | Space::Floor(None) | Space::Room(None) => ".".to_string(),
            Space::Floor(Some(a)) | Space::Room(Some(a)) => format!("{}", a),
            Space::Wall => "#".to_string(),
        };
        write!(f, "{}", s)
    }
}
impl Space {
    fn walkable(&self) -> bool {
        // This is a path (even if it may currently be blocked)
        match self {
            Space::Wall => false,
            _ => true,
        }
    }

    fn stand(&self) -> bool {
        // An amp can stand here
        match self {
            Space::Room(_) | Space::Floor(_) => true,
            _ => false,
        }
    }

    fn blocked(&self) -> bool {
        // Indicates that pathfinding algorithm cannot pass through this square
        match self {
            Space::Room(Some(_)) | Space::Floor(Some(_)) | Space::Wall => true,
            _ => false,
        }
    }
}

impl FromStr for Space {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space = match s {
            "#" => Space::Wall,
            "." => Space::Floor(None),
            " " => return Err(()),
            c => {
                if let Ok(amp) = c.parse() {
                    Space::Room(Some(amp))
                } else {
                    return Err(());
                }
            }
        };

        Ok(space)
    }
}

#[derive(Debug, Clone)]
pub struct Burrow {
    rooms: HashMap<Amp, Vec<Point2<i32>>>,
    burrow: HashMap<Point2<i32>, Space>,
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let max_x = self.burrow.keys().max_by_key(|p| p.x).unwrap().x as usize + 1;
        let max_y = self.burrow.keys().max_by_key(|p| p.y).unwrap().y as usize + 1;
        let mut buf = vec![" ".to_string(); max_x * max_y];

        self.burrow
            .iter()
            .for_each(|(p, v)| buf[p.x as usize + p.y as usize * max_x] = format!("{}", v));

        buf.chunks_exact(max_x).try_for_each(|x| {
            x.iter().try_for_each(|c| write!(f, "{}", c))?;
            writeln!(f)
        })
    }
}
impl Burrow {
    fn all_good(&self) -> bool {
        self.rooms.iter().all(|(amp, rooms)| {
            rooms.iter().all(|r| match self.burrow.get(r).unwrap() {
                Space::Room(Some(room_amp)) if room_amp == amp => true,
                _ => false,
            })
        })
    }
    fn move_amp(&mut self, from: Point2<i32>, to: Point2<i32>) {
        let amp = match self.burrow.get_mut(&from).unwrap() {
            Space::Room(amp) | Space::Floor(amp) => {
                let amp = amp.take();
                amp.unwrap()
            }
            _ => panic!("invalid location"),
        };

        match self.burrow.get_mut(&to).unwrap() {
            Space::Room(loc) | Space::Floor(loc) if loc.is_none() => {
                *loc = Some(amp);
            }
            _ => panic!("can't insert amp"),
        }
    }
    fn can_move(&self, amp: Amp, from: Point2<i32>, to: Point2<i32>) -> bool {
        let from_space = self.burrow.get(&from).unwrap();
        let is_room = match from_space {
            Space::Room(_) => true,
            _ => false,
        };

        if is_room {
            let your_rooms = self.rooms.get(&amp).unwrap();
            if *your_rooms.iter().last().unwrap() == from
                || (your_rooms.contains(&from)
                    && your_rooms
                        .iter()
                        .all(|rp| match self.burrow.get(&rp).unwrap() {
                            Space::Room(Some(a)) if *a == amp => true,
                            _ => false,
                        }))
            {
                return false;
            }
        }

        self.burrow
            .get(&to)
            .map(|b| match b {
                Space::Room(_) => {
                    let rooms = self.rooms.get(&amp).unwrap();
                    !is_room
                        && rooms.contains(&to)
                        && rooms.iter().all(|rp| match self.burrow.get(rp).unwrap() {
                            Space::Room(a) => a.map(|a| a == amp).unwrap_or(true),
                            _ => panic!(),
                        })
                        && rooms
                            .iter()
                            .rev()
                            .filter(|rp| match self.burrow.get(rp).unwrap() {
                                Space::Room(a) => a.is_none(),
                                _ => panic!(),
                            })
                            .next()
                            .map(|rp| *rp == to)
                            .unwrap_or(false)
                }
                Space::Floor(a) => is_room && a.is_none(),
                _ => false,
            })
            .unwrap_or(false)
    }
}
impl FromStr for Burrow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut y = 0i32;
        let mut burrow = s
            .lines()
            .flat_map(|l| {
                let iter = l
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .map(|(x, c)| (Point2::new(x as i32, y), String::from(c).parse().unwrap()))
                    .collect::<Vec<_>>();
                y += 1;
                iter.into_iter()
            })
            .collect::<HashMap<_, _>>();

        let illegal = burrow
            .iter()
            .flat_map(|(p, b)| match b {
                Space::Floor(_)
                    if matches!(burrow.get(&(p + Vector2::new(0, 1))), Some(Space::Room(_))) =>
                {
                    Some(*p)
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        illegal.iter().for_each(|p| {
            burrow.insert(*p, Space::IllegalFloor);
        });

        let mut rooms = burrow
            .iter()
            .flat_map(|(p, b)| match b {
                Space::Room(_) => Some(*p),
                _ => None,
            })
            .collect::<Vec<_>>();
        rooms.sort_by_key(|p| (p.x, p.y));

        let mut groups = rooms.iter().group_by(|p| p.x);

        let mut rooms = groups
            .into_iter()
            .map(|(_key, group)| group.cloned().collect::<Vec<_>>())
            .zip([Amp::Amber, Amp::Bronze, Amp::Copper, Amp::Desert].into_iter())
            .map(|(x, y)| (y, x))
            .collect();

        Ok(Burrow { rooms, burrow })
    }
}

fn run(v: Burrow) -> usize {
    println!("{:?}", v);
    println!("{}", v);

    let mut ungraph = UnGraph::<Point2<i32>, _>::new_undirected();
    let node_indices = v
        .burrow
        .iter()
        .map(|(p, _b)| (*p, ungraph.add_node(*p)))
        .collect::<HashMap<_, _>>();

    node_indices.iter().for_each(|(p, n1)| {
        let b = v.burrow.get(&p).unwrap();
        if b.walkable() {
            let dirs = [
                Vector2::new(1, 0),
                Vector2::new(0, 1),
                Vector2::new(-1, 0),
                Vector2::new(0, -1),
            ];

            for d in dirs {
                let p2 = p + d;
                if let Some(b2) = v.burrow.get(&p2) {
                    if b2.walkable() {
                        let n2 = node_indices.get(&p2).unwrap();
                        ungraph.add_edge(*n1, *n2, 1);
                    }
                }
            }
        }
    });

    let dists = node_indices
        .iter()
        .flat_map(|(p, ni)| {
            v.burrow
                .get(p)
                .filter(|b| b.stand())
                .map(|b| (*p, dijkstra(&ungraph, *ni, None, |_| 1)))
        })
        .collect::<HashMap<_, _>>();

    let mut states = HashMap::new();

    fn dfs(
        states: &mut HashMap<Vec<(Point2<i32>, Amp)>, usize>,
        burrow: Burrow,
        node_indices: &HashMap<Point2<i32>, NodeIndex>,
        graph: &UnGraph<Point2<i32>, usize>,
        dist: &HashMap<Point2<i32>, HashMap<NodeIndex, usize>>,
        energy: usize,
        best: Option<usize>,
    ) -> Option<usize> {
        if let Some(b) = best {
            if energy > b {
                return None;
            }
        }
        //println!("{:?}", energy);
        if burrow.all_good() {
            return Some(energy);
        }
        let burrow = &burrow;
        let mut amps = burrow
            .burrow
            .iter()
            .flat_map(|(p, x)| match x {
                Space::Floor(amp) | Space::Room(amp) => amp.map(|a| (*p, a)),
                _ => None,
            })
            .collect::<Vec<(Point2<_>, Amp)>>();

        amps.sort_by_key(|(p, a)| (p.x, p.y, *a));

        if states.get(&amps).map(|e| *e <= energy).unwrap_or(false) {
            return None;
        }
        states.insert(amps.clone(), energy);

        amps.into_iter()
            .flat_map(|(amp_pos, amp)| {
                let mut legal_endpoint: Vec<_> = burrow
                    .burrow
                    .iter()
                    .filter(|(p, _b)| burrow.can_move(amp, amp_pos, **p))
                    .collect();
                legal_endpoint.sort_by_key(|(p, _)| dist.get(&amp_pos).unwrap().get(&node_indices[&p]).unwrap());
                    
                let mut reachable = HashSet::new();

                petgraph::visit::depth_first_search(
                    &graph,
                    Some(node_indices[&amp_pos]),
                    |event| {
                        use petgraph::visit::{Control, DfsEvent};
                        match event {
                            DfsEvent::Discover(n, _) => {
                                if graph[n] == amp_pos {
                                    Control::Continue
                                } else if burrow.burrow[&graph[n]].blocked() {
                                    Control::<()>::Prune
                                } else {
                                    reachable.insert(n);
                                    Control::Continue
                                }
                            }
                            _ => Control::Continue,
                        }
                    },
                );
                //println!("{}", burrow);
                //println!("{:?} {:?} {:#?}", amp_pos, amp, reachable.iter().map(|n| graph[*n]).collect::<Vec<_>>());
                //println!("{:#?}", legal_endpoint);

                legal_endpoint
                    .into_iter()
                    .filter(move |(p, _b)| reachable.contains(&node_indices[p]))
                    .map(move |(p, b)| (amp_pos, amp, p, b))
            })
            .scan(best, move |mut best, (amp_pos, amp, p, b)| {
                let mut burrow = burrow.clone();
                burrow.move_amp(amp_pos, *p);
                let found = dfs(
                    states,
                    burrow,
                    node_indices,
                    graph,
                    dist,
                    energy
                        + amp.energy()
                            * dist.get(&amp_pos).unwrap().get(&node_indices[&p]).unwrap(),
                    *best,
                );
                match (found, &best) {
                    (Some(f), None) => {
                        *best = found;
                    }
                    (Some(f), Some(b)) if *b > f => {
                        *best = found;
                        println!("Best {:?}", best);
                    }
                    _ => {}
                }
                Some(*best)
            })
            .flatten()
            .min()
    }

    dfs(
        &mut states,
        v.clone(),
        &node_indices,
        &ungraph,
        &dists,
        0,
        None,
    )
    .unwrap()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 23;
    type Input = Burrow;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(mut r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut s = String::new();
        r.read_to_string(&mut s)?;

        Ok(s.parse().unwrap())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        run(v.clone())
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let input = std::fs::read_to_string("input/day23p2.txt").unwrap();
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        run(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = concat!(
            "#############\n",
            "#...........#\n",
            "###B#C#B#D###\n",
            "  #A#D#C#A#  \n",
            "  #########  "
        );
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 12521);
    }
    #[test]
    fn test2() {
        let input = concat!(
            "#############\n",
            "#...........#\n",
            "###B#C#B#D###\n",
            "  #D#C#B#A#  \n",
            "  #D#B#A#C#  \n",
            "  #A#D#C#A#  \n",
            "  #########  "
        );
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(run(input), 44169);
        //assert_eq!(Solution::p2(&input), 26984457539);
        //unimplemented!()
    }
}
