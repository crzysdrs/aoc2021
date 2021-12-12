use clap::Parser;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version = "1.0", author = "Mitch Souders <crzysdrs@gmail.com>")]
struct Opts {
    test: u32,
    part: u32,
}

trait Day
where
    Self: 'static,
{
    const DAY: u32;
    type Input;
    type Sol1: std::fmt::Display;
    type Sol2: std::fmt::Display;
    fn input() -> PathBuf {
        let input = PathBuf::from("input").join(format!("day{:02}.txt", Self::DAY));
        if !input.exists() {
            panic!("Input does not exist: {}", input.display());
        }
        input
    }
    fn run_p1() -> IoResult<()> {
        let mut buf = BufReader::new(std::fs::File::open(Self::input())?);
        let v = Self::process_input(&mut buf)?;
        println!("Solution Day {} Part 1", Self::DAY);
        println!("{}", Self::p1(&v));
        Ok(())
    }
    fn run_p2() -> IoResult<()> {
        let mut buf = BufReader::new(std::fs::File::open(Self::input())?);
        let v = Self::process_input(&mut buf)?;
        println!("Solution Day {} Part 2", Self::DAY);
        println!("{}", Self::p2(&v));
        Ok(())
    }
    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead;
    fn p1(_input: &Self::Input) -> Self::Sol1 {
        unimplemented!("Missing implementation of Day {} Part 1", Self::DAY)
    }
    fn p2(_input: &Self::Input) -> Self::Sol2 {
        unimplemented!("Missing implementation of Day {} Part 2", Self::DAY)
    }
}

type IoFn = fn() -> IoResult<()>;
macro_rules! tests {
    ($($name:ident),*) => {
        $(
            mod $name;
        )*
        fn tests() -> HashMap<u32, (IoFn, IoFn)> {
            [
                $(
                    (
                        $name::Solution::DAY,
                        ($name::Solution::run_p1 as IoFn,
                         $name::Solution::run_p2 as IoFn)
                    ),
                )*
            ].into_iter().collect()
        }
    }
}

tests!(
    template, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let sols = tests();
    if let Some(sol) = sols.get(&opts.test) {
        match opts.part {
            1 => (sol.0)()?,
            2 => (sol.1)()?,
            p => {
                let err = format!("Unknown Test (Day {} Part {})", opts.test, p);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
            }
        }
    } else {
        let err = format!("Unknown Test (Day {} Part {})", opts.test, opts.part);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
    }

    Ok(())
}
