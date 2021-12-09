use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

fn get_val(arr: &Vec<Vec<usize>>, x: i32, y: i32) -> Option<&usize> {
    if x < 0 || y < 0 {
        return None;
    } else {
        arr.get(y as usize)
            .and_then(|row: &Vec<usize>| row.get(x as usize))
    }
}

fn lows(v: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut low = vec![];
    for y in 0..v.len() as i32 {
        let row = &v[y as usize];
        for x in 0..row.len() as i32 {
            let search = [
                get_val(&v, x - 1, y),
                get_val(&v, x, y + 1),
                get_val(&v, x, y - 1),
                get_val(&v, x + 1, y),
            ];
            let m = search.into_iter().flat_map(|x| x).min();

            if m.unwrap() > &v[y as usize][x as usize] {
                low.push((x as usize, y as usize));
            }
        }
    }
    low
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9;
    type Input = Vec<Vec<usize>>;
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
        lows(v)
            .into_iter()
            .map(|(x, y)| get_val(v, x as i32, y as i32).unwrap() + 1)
            .sum()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let lows = lows(v);

        let mut basins = lows
            .into_iter()
            .map(|(x, y)| {
                let mut search_basin = vec![(x, y)];
                let mut searched = HashSet::new();
                while let Some(s) = search_basin.pop() {
                    if searched.contains(&s) {
                        continue;
                    } else {
                        searched.insert(s);
                    }
                    let l = get_val(v, s.0 as i32, s.1 as i32).unwrap();

                    let search = [
                        (s.0 as i32 - 1, s.1 as i32),
                        (s.0 as i32, s.1 as i32 + 1),
                        (s.0 as i32, s.1 as i32 - 1),
                        (s.0 as i32 + 1, s.1 as i32),
                    ];
                    search_basin.extend(
                        search
                            .iter()
                            .flat_map(|s| get_val(v, s.0 as i32, s.1 as i32).map(|v| (s, v)))
                            .filter(|(_, v)| **v < 9 && *v > l)
                            .map(|(p, _)| (p.0 as usize, p.1 as usize)),
                    );
                }
                searched.into_iter().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        basins.sort_by_key(|v| v.len());
        basins.reverse();
        basins.iter().take(3).map(|v| v.len()).fold(1, |x, y| x * y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "2199943210\n\
3987894921\n\
9856789892\n\
8767896789\n\
9899965678";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 15);
        assert_eq!(Solution::p2(&input), 1134);
    }
}
