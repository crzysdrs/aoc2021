use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reg {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Reg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "w" => Reg::W,
            "x" => Reg::X,
            "y" => Reg::Y,
            "z" => Reg::Z,
            _ => return Err(()),
        };

        Ok(r)
    }
}

#[derive(Debug)]
pub enum Op {
    Imm(i32),
    Reg(Reg),
}

impl Op {
    fn val(&self, alu: &Alu) -> i32 {
        match self {
            Op::Imm(v) => *v,
            Op::Reg(r) => *alu.reg(r),
        }
    }

    fn reg(&self) -> Option<Reg> {
        match self {
            Op::Reg(reg) => Some(*reg),
            _ => None,
        }
    }
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = if let Ok(r) = s.parse() {
            Op::Reg(r)
        } else {
            Op::Imm(s.parse().unwrap())
        };
        Ok(op)
    }
}

#[derive(Debug)]
pub enum Instr {
    Mul(Reg, Op),
    Eq(Reg, Op),
    Add(Reg, Op),
    Div(Reg, Op),
    Mod(Reg, Op),
    Inp(Reg),
}

#[derive(Debug, Eq, PartialEq)]
enum AluErr {
    NeedInput(Reg),
}

impl Instr {
    fn run<I>(&self, alu: &mut Alu, mut input: I) -> Result<(), AluErr>
    where
        I: Iterator<Item = i32>,
    {
        match self {
            Instr::Mul(l, r) => *alu.reg_mut(&l) = alu.reg(&l) * r.val(alu),
            Instr::Add(l, r) => *alu.reg_mut(&l) = alu.reg(&l) + r.val(alu),
            Instr::Div(l, r) => *alu.reg_mut(&l) = alu.reg(&l) / r.val(alu),
            Instr::Mod(l, r) => *alu.reg_mut(&l) = alu.reg(&l) % r.val(alu),
            Instr::Eq(l, r) => *alu.reg_mut(&l) = if *alu.reg(&l) == r.val(alu) { 1 } else { 0 },
            Instr::Inp(d) => {
                *alu.reg_mut(d) = if let Some(i) = input.next() {
                    i
                } else {
                    return Err(AluErr::NeedInput(*d));
                }
            }
        }
        Ok(())
    }
    fn reads(&self) -> Vec<Reg> {
        let regs = match self {
            Instr::Mul(l, r)
            | Instr::Add(l, r)
            | Instr::Div(l, r)
            | Instr::Mod(l, r)
            | Instr::Eq(l, r) => [Some(*l), r.reg()].to_vec(),
            Instr::Inp(_d) => [].to_vec(),
        };
        regs.into_iter().flatten().collect()
    }
    fn writes(&self) -> Vec<Reg> {
        let regs = match self {
            Instr::Mul(l, _r)
            | Instr::Add(l, _r)
            | Instr::Div(l, _r)
            | Instr::Mod(l, _r)
            | Instr::Eq(l, _r) => [Some(*l)].to_vec(),
            Instr::Inp(d) => [Some(*d)].to_vec(),
        };
        regs.into_iter().flatten().collect()
    }
}

impl FromStr for Instr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut terms = s.split_whitespace();
        let instr = match terms.next().unwrap() {
            opcode @ ("mul" | "eql" | "add" | "div" | "mod") => {
                let l = terms.next().unwrap().parse().unwrap();
                let r = terms.next().unwrap().parse().unwrap();
                match opcode {
                    "mul" => Instr::Mul(l, r),
                    "eql" => Instr::Eq(l, r),
                    "add" => Instr::Add(l, r),
                    "div" => Instr::Div(l, r),
                    "mod" => Instr::Mod(l, r),
                    _ => unreachable!(),
                }
            }
            "inp" => Instr::Inp(terms.next().unwrap().parse().unwrap()),
            s => {
                panic!("{}", s)
            }
        };

        Ok(instr)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Alu {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Default for Alu {
    fn default() -> Alu {
        Alu {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }
}

impl Alu {
    fn reg(&self, r: &Reg) -> &i32 {
        match r {
            Reg::X => &self.x,
            Reg::Y => &self.y,
            Reg::Z => &self.z,
            Reg::W => &self.w,
        }
    }
    fn reg_mut(&mut self, r: &Reg) -> &mut i32 {
        match r {
            Reg::X => &mut self.x,
            Reg::Y => &mut self.y,
            Reg::Z => &mut self.z,
            Reg::W => &mut self.w,
        }
    }

    fn run<I>(&mut self, instr: &[Instr], mut input: I) -> Result<(), AluErr>
    where
        I: Iterator<Item = i32>,
    {
        instr
            .iter()
            .try_for_each(|instr| instr.run(self, &mut input))
    }
}

fn digits(v: &Vec<Instr>, max: bool) -> u64 {
    for x in v {
        println!("{:?}", x);
    }
    //Split up all instructions into sets separated by input requests
    let instr_ranges = v
        .iter()
        .enumerate()
        .filter(|(_, x)| matches!(x, Instr::Inp(_)))
        .map(|(i, _)| i..)
        .collect::<Vec<_>>();

    //Keep track of alu.z value after each step, and digits that led to it.
    let mut prev_z = HashMap::new();
    let mut alu = Alu::default();
    let _ = alu.run(&v, std::iter::empty());
    prev_z.insert(alu, 0u64);

    for (i, range) in instr_ranges.iter().enumerate() {
        let mut valid_z = HashMap::new();
        for d in 1..10 {
            for (alu, digits) in &prev_z {
                let mut alu = alu.clone();
                let result = alu.run(&v[range.clone()], [d].into_iter());
                let store_z = match result {
                    Err(AluErr::NeedInput(d)) => {
                        // we can set a register to a default state, since we are going to overwrite it.
                        *alu.reg_mut(&d) = 0;
                        true
                    }
                    Ok(_) if alu.z == 0 => true,
                    _ => false,
                };
                //println!("{:?}", result);
                if store_z {
                    let mut new_digits = *digits;
                    new_digits *= 10;
                    new_digits += d as u64;

                    let old_digits = valid_z.entry(alu).or_insert_with(|| new_digits);
                    use std::cmp::Ordering;
                    // Keep only max/min digits value for a target z value
                    let replace = match (*old_digits).cmp(&new_digits) {
                        Ordering::Less => max,
                        Ordering::Equal => false,
                        Ordering::Greater => !max,
                    };
                    if replace {
                        *old_digits = new_digits;
                    }
                }
            }
        }
        prev_z = valid_z;
        println!("{} Prev Z: {:?}", i, prev_z.len());
    }

    let remain = prev_z.iter().map(|(_alu, digits)| digits);
    *if max { remain.max() } else { remain.min() }.unwrap()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 24;
    type Input = Vec<Instr>;
    type Sol1 = u64;
    type Sol2 = u64;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        let mut instr: Vec<_> = r
            .lines()
            .flatten()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<_>>();

        // Useless instructions
        instr.retain(|i| match i {
            Instr::Div(_, Op::Imm(1)) | Instr::Mul(_, Op::Imm(1)) | Instr::Add(_, Op::Imm(0)) => {
                false
            }
            _ => true,
        });

        // Hoist instructions
        for _ in 0..10 {
            for i in 1..instr.len() {
                let target = instr.iter().enumerate().take(i).rev().take_while(
                |(_j, target)| {
                    // Can't move input instructions relative to each other
                    // !matches!((&instr[i], target), (Instr::Inp(_), Instr::Inp(_)))
                    // Instead... don't move inputs, we prefer this behavior anyway for caching.
                    !matches!(&instr[i], Instr::Inp(_))
                        // Can't move instruction that reads from anything that will be written
                        && !instr[i].reads().iter().any(|r| target.writes().contains(r))
                        // Can't move instruction that writes to anything read/written
                        && !instr[i].writes().iter().any(|w| target.reads().contains(w) || target.writes().contains(w))
                }
            ).map(|(j, _)| j).next();
                if let Some(t) = target {
                    println!("Swapped {}:{:?} {}:{:?}", t, instr[t], i, instr[i]);
                    instr.swap(t, i);
                }
            }
        }

        Ok(instr)
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        digits(v, true)
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        digits(v, false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "inp x\n\
                     mul x -1";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        for x in 0..10 {
            let mut alu = Alu::default();
            assert_eq!(alu.run(&input, [x].into_iter()), Ok(()));
            assert_eq!(-x, alu.x);
        }

        let input = "inp z\n\
                     inp x\n\
                     mul z 3\n\
                     eql z x";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        for x in 1..100 {
            for y in 1..100 {
                let mut alu = Alu::default();
                assert_eq!(alu.run(&input, [x, y].into_iter()), Ok(()));
                assert_eq!(alu.z, if 3 * x == y { 1 } else { 0 });
            }
        }

        let input = "inp w\n\
                     add z w\n\
                     mod z 2\n\
                     div w 2\n\
                     add y w\n\
                     mod y 2\n\
                     div w 2\n\
                     add x w\n\
                     mod x 2\n\
                     div w 2\n\
                     mod w 2";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        for x in 1..100 {
            let mut alu = Alu::default();
            assert_eq!(alu.run(&input, [x].into_iter()), Ok(()));
            assert_eq!(
                [alu.w, alu.x, alu.y, alu.z],
                [
                    (0b1000 & x) >> 3,
                    (0b100 & x) >> 2,
                    (0b10 & x) >> 1,
                    0b1 & x
                ]
            );
        }
        //unimplemented!()
    }
    #[test]
    fn p1() {
        let input = std::fs::read("input/day24.txt").unwrap();
        let input = Solution::process_input(std::io::BufReader::new(input.as_slice())).unwrap();

        assert_eq!(Solution::p1(&input), 39494195799979);
    }
    #[test]
    fn p2() {
        let input = std::fs::read("input/day24.txt").unwrap();
        let input = Solution::process_input(std::io::BufReader::new(input.as_slice())).unwrap();

        assert_eq!(Solution::p2(&input), 13161151139617);
    }
}
