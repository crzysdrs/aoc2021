use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            s if s.chars().all(|c| c.is_uppercase()) => Ok(Node::BigCave(s.to_string())),
            s => Ok(Node::SmallCave(s.to_string())),
        }
    }
}

fn dfs<'n>(count: &mut usize, nodes: &HashMap<&'n Node, Vec<&'n Node>>, path: &mut Vec<&'n Node>) {
    let cur = path.iter().last().unwrap();
    if **cur == Node::End {
        //println!("{:?}", path);
        *count += 1;
        return;
    }
    for n in nodes.get(path.iter().last().unwrap()).unwrap() {
        if match path.iter().find(|node| *node == n) {
            Some(Node::Start | Node::End | Node::SmallCave(_)) => false,
            Some(Node::BigCave(_)) => true,
            None => true,
        } {
            path.push(n);
            dfs(count, nodes, path);
            path.pop();
        }
    }
}

fn dfs2<'n>(
    count: &mut usize,
    nodes: &HashMap<&'n Node, Vec<&'n Node>>,
    path: &mut Vec<&'n Node>,
    two_visits: bool,
) {
    let cur = path.iter().last().unwrap();
    if **cur == Node::End {
        //println!("{:?}", path);
        *count += 1;
        return;
    }
    for n in nodes.get(path.iter().last().unwrap()).unwrap() {
        let mut next_visit = two_visits;
        if match path.iter().find(|node| *node == n) {
            Some(Node::Start | Node::End) => false,
            Some(Node::SmallCave(_)) => {
                if two_visits {
                    false
                } else {
                    next_visit = true;
                    true
                }
            }
            Some(Node::BigCave(_)) => true,
            None => true,
        } {
            path.push(n);
            dfs2(count, nodes, path, next_visit);
            path.pop();
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 12;
    type Input = Vec<(Node, Node)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                let (n1, n2) = l.split_once('-').unwrap();
                Ok((Node::from_str(n1).unwrap(), Node::from_str(n2).unwrap()))
            })
            .collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let nodes = v.iter().fold(
            HashMap::new(),
            |mut hm: HashMap<&Node, Vec<&Node>>, (n1, n2)| {
                hm.entry(n1).and_modify(|e| e.push(n2)).or_insert(vec![n2]);
                hm.entry(n2).and_modify(|e| e.push(n1)).or_insert(vec![n1]);
                hm
            },
        );

        let mut path = vec![&Node::Start];

        let mut count = 0;
        dfs(&mut count, &nodes, &mut path);
        count
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let nodes = v.iter().fold(
            HashMap::new(),
            |mut hm: HashMap<&Node, Vec<&Node>>, (n1, n2)| {
                hm.entry(n1).and_modify(|e| e.push(n2)).or_insert(vec![n2]);
                hm.entry(n2).and_modify(|e| e.push(n1)).or_insert(vec![n1]);
                hm
            },
        );

        let mut path = vec![&Node::Start];

        let mut count = 0;
        dfs2(&mut count, &nodes, &mut path, false);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "start-A\n\
start-b\n\
A-c\n\
A-b\n\
b-d\n\
A-end\n\
b-end";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 10);
        assert_eq!(Solution::p2(&input), 36);
        //unimplemented!()
    }
}
