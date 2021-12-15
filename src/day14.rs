use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Polymer {
    template: String,
    pairs: HashMap<(char, char), char>,
}
pub struct Solution {}

fn run(v: &Polymer, steps: usize) -> usize {
    fn merge(dst: &mut HashMap<char, usize>, src: &HashMap<char, usize>) {
        // Merge the summation of both hashmaps together.
        src.iter().for_each(|(k, v)| {
            *dst.entry(*k).or_insert(0) += v;
        });
    }
    fn dp(
        computed: &mut HashMap<((char, char), usize), HashMap<char, usize>>,
        lookup: (char, char),
        depth: usize,
    ) -> HashMap<char, usize> {
        // Try to see if we have computed this subproblem already.
        if let Some(r) = computed.get(&(lookup, depth)) {
            return r.clone();
        }
        // Lookup the inserted character for this problem
        let (insert, _) = computed.get(&(lookup, 1)).unwrap().iter().nth(0).unwrap();
        let insert = insert.clone();

        //Compute both the left
        let mut r1 = dp(computed, (lookup.0, insert), depth - 1);
        // and right subproblems
        let r2 = dp(computed, (insert, lookup.1), depth - 1);

        // Merge the results
        merge(&mut r1, &r2);
        // Don't forget to add the inserted character.
        *r1.entry(insert).or_insert(0) += 1;

        //Save the subproblem result.
        computed.insert((lookup, depth), r1.clone());

        return r1;
    }

    let mut computed = HashMap::new();

    // Compute the base case, where we are just inserting the single character at depth 1.
    v.pairs.iter().for_each(|((c1, c2), o)| {
        computed.insert(((*c1, *c2), 1), [(*o, 1)].into_iter().collect());
    });

    let chars = v.template.chars().collect::<Vec<_>>();

    let mut count = chars
        .windows(2)
        .map(|cs| {
            // Split up the original problem into each subproblem of two chars.
            dp(&mut computed, (cs[0], cs[1]), steps)
        })
        .fold(HashMap::new(), |mut state, count| {
            // merge all the results
            merge(&mut state, &count);
            state
        });

    // Don't forget to include the count of the original characters in the problem.
    chars.iter().for_each(|c| {
        *count.entry(*c).or_insert(0) += 1;
    });

    let min = count.iter().min_by_key(|(_, v)| *v).unwrap();
    let max = count.iter().max_by_key(|(_, v)| *v).unwrap();
    max.1 - min.1
}

impl Day for Solution {
    const DAY: u32 = 14;
    type Input = Polymer;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines().map(|l| l.unwrap());

        let template = lines.by_ref().next().unwrap();
        lines.by_ref().next();
        let pairs = lines
            .map(|l| {
                let (l, r) = l.split_once(" -> ").unwrap();
                (
                    (l.chars().nth(0).unwrap(), l.chars().nth(1).unwrap()),
                    r.chars().next().unwrap(),
                )
            })
            .collect();
        Ok(Polymer { template, pairs })
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        run(v, 10)
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        run(v, 40)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "NNCB\n\
\n\
CH -> B\n\
HH -> N\n\
CB -> H\n\
NH -> C\n\
HB -> C\n\
HC -> B\n\
HN -> C\n\
NN -> C\n\
BH -> H\n\
NC -> B\n\
NB -> B\n\
BN -> B\n\
BB -> N\n\
BC -> B\n\
CC -> N\n\
CN -> C";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 1588);
        assert_eq!(Solution::p2(&input), 2188189693529);
        //unimplemented!()
    }
}
