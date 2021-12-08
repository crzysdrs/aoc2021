use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}

pub struct SevenSeg {
    signals: [HashSet<char>; 10],
    output: [String; 4],
}

fn sort_string(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.sort();
    chars.iter().collect::<String>()
}

impl Day for Solution {
    const DAY: u32 = 8;
    type Input = Vec<SevenSeg>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                let sides = l.split("|").collect::<Vec<_>>();
                let seven_seg = SevenSeg {
                    signals: sides[0]
                        .split_whitespace()
                        .map(|x| x.chars().collect())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    output: sides[1]
                        .split_whitespace()
                        .map(|x| sort_string(x))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                };
                Ok(seven_seg)
            })
            .collect::<Result<Vec<_>, _>>()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        v.iter()
            .flat_map(|x| x.output.iter())
            .filter(|x| {
                [
                    2, /* Digit 1*/
                    4, /* Digit 4*/
                    3, /* Digit 7*/
                    7, /* Digit 8*/
                ]
                .contains(&x.len())
            })
            .count()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        //https://www.wikiwand.com/en/Seven-segment_display
        // Using the Segment Naming conventions from wikipedia.
        let digit: [HashSet<char>; 10] = [
            "abcdef", "bc", "abged", "abgcd", "fgbc", "afgcd", "afgecd", "abc", "abcdefg", "abfgcd",
        ]
        .map(|x| x.chars().collect());

        v.iter()
            .map(|x| {
                let mut found: [Option<(&HashSet<char>)>; 10] = [None; 10];
                // We know the lengths of certain Seven Segments are unique,
                // we store those found entries here.
                found[7] = Some((x.signals.iter().find(|x| x.len() == 3).unwrap()));
                found[1] = Some((x.signals.iter().find(|x| x.len() == 2).unwrap()));
                found[4] = Some((x.signals.iter().find(|x| x.len() == 4).unwrap()));
                found[8] = Some((x.signals.iter().find(|x| x.len() == 7).unwrap()));

                // We know that the segments for 7 minus 1 are going to uniquely identify 'A'
                let is_seg_a = found[7].unwrap() - found[1].unwrap();
                assert_eq!(is_seg_a.len(), 1);
                let seg_a = is_seg_a.into_iter().nth(0).unwrap();

                // Compute the frequencies of segments within the input to uniquely identify several segments.
                let segs_seen = ('a'..='g')
                    .map(|s| (x.signals.iter().filter(|v| v.contains(&s)).count(), s))
                    .collect::<HashMap<usize, char>>();

                // C, E, F are all in certain seven segments with a specific count.
                let seg_c = segs_seen[&9];
                let seg_e = segs_seen[&4];
                let seg_f = segs_seen[&6];

                // We now have enough info to compute the other segments, based on the few digits
                // we do know and the segments we've determined.
                let is_seg_b = found[1].unwrap() - &HashSet::from([seg_c]);
                assert_eq!(is_seg_b.len(), 1);
                let seg_b = is_seg_b.into_iter().nth(0).unwrap();

                let is_seg_g = found[4].unwrap() - &HashSet::from([seg_f, seg_b, seg_c]);
                assert_eq!(is_seg_g.len(), 1);
                let seg_g = is_seg_g.into_iter().nth(0).unwrap();
                let is_seg_d =
                    found[8].unwrap() - &HashSet::from([seg_a, seg_b, seg_c, seg_e, seg_f, seg_g]);
                assert_eq!(is_seg_d.len(), 1);
                let seg_d = is_seg_d.into_iter().nth(0).unwrap();

                // Build the mapping from our segment names to the target segment names
                let seg_map = HashMap::from([
                    ('a', seg_a),
                    ('b', seg_b),
                    ('c', seg_c),
                    ('d', seg_d),
                    ('e', seg_e),
                    ('f', seg_f),
                    ('g', seg_g),
                ]);

                // Compute the target digit strings from our segment names to their map.
                let digit_map = digit
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let new = x
                            .iter()
                            .map(|s| seg_map.get(s).unwrap())
                            .collect::<String>();
                        (sort_string(&new), i)
                    })
                    .collect::<HashMap<_, _>>();

                // Match the output strings and compute the final result
                let res =
                    x.output
                        .iter()
                        .map(|v| digit_map.get(v).unwrap())
                        .fold(0, |mut state, v| {
                            state = state * 10 + v;
                            state
                        });

                res
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 26);
        assert_eq!(Solution::p2(&input), 61229);
        //unimplemented!()
    }
}
