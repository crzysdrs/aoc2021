use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Debug)]
enum Chunk {
    Ok,
    Incomplete(Vec<char>),
    Illegal(char),
}

fn analyze_chunk<S>(l: S) -> Chunk
where
    S: AsRef<str>,
{
    let mut stack = vec![];

    for c in l.as_ref().chars() {
        match c {
            '<' => stack.push('>'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '>' | '}' | ')' | ']' => {
                let pop = stack.pop().unwrap();
                if c == pop {
                    continue;
                } else {
                    return Chunk::Illegal(c);
                }
            }
            _ => panic!(),
        }
    }

    if stack.len() > 0 {
        stack.reverse();
        Chunk::Incomplete(stack)
    } else {
        Chunk::Ok
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input = Vec<String>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        r.lines().map(|l| Ok(l?.to_string())).collect()
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        v.iter()
            .map(analyze_chunk)
            .map(|chunk| match chunk {
                Chunk::Ok | Chunk::Incomplete(_) => 0,
                Chunk::Illegal(c) => match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(),
                },
            })
            .sum()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let mut scores = v
            .iter()
            .map(analyze_chunk)
            .filter(|chunk| !matches!(chunk, Chunk::Ok | Chunk::Illegal(_)))
            .map(|chunk| match chunk {
                Chunk::Ok | Chunk::Illegal(_) => panic!(),
                Chunk::Incomplete(s) => {
                    let mut v = 0;
                    for c in s {
                        v *= 5;
                        v += match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!(),
                        };
                    }
                    v
                }
            })
            .collect::<Vec<_>>();
        scores.sort();
        println!("{:?}", scores);
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "[({(<(())[]>[[{[]{<()<>>\n\
[(()[<>])]({[<{<<[]>>(\n\
{([(<{}[<>[]}>{[]{[(<()>\n\
(((({<>}<{<{<>}{[]{[]{}\n\
[[<[([]))<([[{}[[()]]]\n\
[{[{({}]{}}([{[{{{}}([]\n\
{<[[]]>}<{[{[{[]{()[[[]\n\
[<(<(<(<{}))><([]([]()\n\
<{([([[(<>()){}]>(<<{{\n\
<{([{{}}[<[[[<>{}]]]>[]]";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 26397);
        assert_eq!(Solution::p2(&input), 288957);
        //unimplemented!()
    }
}
