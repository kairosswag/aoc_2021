use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use itertools::Itertools;
use parse_display::Display;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Display)]
#[display("[{x},{y}]")]
pub struct SnailfishNumber {
    x: Box<SnailfishValue>,
    y: Box<SnailfishValue>,
}

#[derive(Debug, Clone, Display)]
pub enum SnailfishValue {
    #[display("{0}")]
    Pair(SnailfishNumber),
    #[display("{0}")]
    Number(u32),
}

impl SnailfishNumber {
    fn add(&mut self, second: &SnailfishNumber) -> &SnailfishNumber {
        use SnailfishValue::*;
        self.x = Box::new(Pair(self.clone()));
        self.y = Box::new(Pair(second.clone()));
        // println!("after addition:\t {}", self);
        self.reduce();
        self
    }

    fn reduce(&mut self) {
        let mut work_done = true;
        while work_done {
            work_done = self.try_explode(0).2;

            if !work_done {
                work_done = self.try_split();
                // println!("after split:\t {}", self);
                // } else {
                // println!("after explode:\t {}", self);
            }
        }
    }

    fn try_explode(&mut self, depth: u8) -> (Option<u32>, Option<u32>, bool) {
        use SnailfishValue::*;
        if depth == 4 {
            // boom!
            let x_val = match *self.x {
                Number(val) => val,
                _ => unreachable!(),
            };
            let y_val = match *self.y {
                Number(val) => val,
                _ => unreachable!(),
            };
            return (Some(x_val), Some(y_val), true);
        }

        if let Pair(val) = &mut *self.x {
            let mut explode_res = val.try_explode(depth + 1);
            if let (Some(_), Some(_), _) = explode_res {
                self.x = Box::new(Number(0));
            }
            if let (_, Some(val), _) = explode_res {
                self.y.add_left(val);
                explode_res = (explode_res.0, None, explode_res.2)
            }

            if let (_, _, true) = explode_res {
                return explode_res;
            }
        }

        if let Pair(val) = &mut *self.y {
            let mut explode_res = val.try_explode(depth + 1);
            if let (Some(_), Some(_), _) = explode_res {
                self.y = Box::new(Number(0));
            }
            if let (Some(val), _, _) = explode_res {
                self.x.add_right(val);
                explode_res = (None, explode_res.1, explode_res.2)
            }

            explode_res
        } else {
            (None, None, false)
        }
    }

    fn try_split(&mut self) -> bool {
        use SnailfishValue::*;
        let split = match &mut *self.x {
            Number(val) => {
                if *val >= 10 {
                    let x = Box::new(Number(*val / 2));
                    let y = Box::new(Number((*val + 1) / 2));
                    *self.x = Pair(SnailfishNumber { x, y });
                    true
                } else {
                    false
                }
            }
            Pair(snail) => snail.try_split(),
        };
        if !split {
            match &mut *self.y {
                Number(val) => {
                    if *val >= 10 {
                        let x = Box::new(Number(*val / 2));
                        let y = Box::new(Number((*val + 1) / 2));
                        *self.y = Pair(SnailfishNumber { x, y });
                        true
                    } else {
                        false
                    }
                }
                Pair(snail) => snail.try_split(),
            }
        } else {
            true
        }
    }

    fn magnitude(&self) -> u64 {
        3 * self.x.magnitude() + 2 * self.y.magnitude()
    }
}

impl SnailfishValue {
    fn add_left(&mut self, by: u32) {
        use SnailfishValue::*;
        match self {
            Pair(val) => val.x.add_left(by),
            Number(val) => *val += by,
        }
    }

    fn add_right(&mut self, by: u32) {
        use SnailfishValue::*;
        match self {
            Pair(val) => val.y.add_right(by),
            Number(val) => *val += by,
        }
    }

    fn magnitude(&self) -> u64 {
        use SnailfishValue::*;
        match self {
            Pair(val) => val.magnitude(),
            Number(val) => *val as u64,
        }
    }
}

pub fn generator(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(|l| l.parse().expect("uh?")).collect()
}

impl FromStr for SnailfishNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // find the middle comma to split
        let mut open_bracket_count = 0;
        let mut found_idx = None;
        'search: for (index, char) in s.char_indices() {
            match char {
                '[' => open_bracket_count += 1,
                ']' => open_bracket_count -= 1,
                ',' => {
                    if open_bracket_count == 1 {
                        found_idx = Some(index);
                        break 'search;
                    }
                }
                _ => (),
            }
        }
        let found_idx = found_idx.ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not parse {s}"),
        ))?;

        let x = Box::new(s[1..found_idx].parse()?);
        let y = Box::new(s[found_idx + 1..s.len() - 1].parse()?);

        Ok(SnailfishNumber { x, y })
    }
}

impl FromStr for SnailfishValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not parse {s}"),
        ))? == '['
        {
            Ok(Self::Pair(s.parse()?))
        } else {
            Ok(Self::Number(s.parse().unwrap()))
        }
    }
}

pub fn part_1(snails: &Vec<SnailfishNumber>) -> u64 {
    let mut accum = snails[0].clone();
    for snail in snails.iter().skip(1) {
        accum.add(snail);
    }

    accum.magnitude()
}

pub fn part_2(snails: &Vec<SnailfishNumber>) -> u64 {
    snails
        .iter()
        .tuple_combinations()
        .collect::<Vec<(&SnailfishNumber, &SnailfishNumber)>>()
        .par_iter()
        .map(|(a, b)| (*a).clone().add(b).magnitude())
        .max()
        .unwrap()
}

#[test]
pub fn test() {
    let input = "[1,1]\n\
    [2,2]\n\
    [3,3]\n\
    [4,4]\n\
    [5,5]\n\
    [6,6]";
    let snails = generator(&input); // TODO: Debug
    let mut accum = snails[0].clone();
    for snail in snails.iter().skip(1) {
        accum.add(snail);
        println!("Accum: {}", accum);
    }
    assert_eq!(1, 1);
}

#[test]
fn test_explo() {
    let input = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]";
    let vals = generator(&input);
    let mut first = vals[0].clone();
    let second = &vals[1];

    first.add(second);
}
