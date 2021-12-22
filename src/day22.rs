use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
#[display("{switch} x={x_min}..{x_max},y={y_min}..{y_max},z={z_min}..{z_max}")]
pub struct Instruction {
    switch: Switch,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Instruction {
    fn abuse_as_min_max(mut self, other: &Instruction) -> Instruction {
        self.x_min = self.x_min.min(other.x_min);
        self.x_max = self.x_max.max(other.x_max);
        self.y_min = self.y_min.min(other.y_min);
        self.y_max = self.y_max.max(other.y_max);
        self.z_min = self.z_min.min(other.z_min);
        self.z_max = self.z_max.max(other.z_max);
        self
    }
}

#[derive(Debug, Display, FromStr, PartialEq)]
#[display(style = "lowercase")]
pub enum Switch {
    On,
    Off,
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(|line| line.parse()).collect()
}

pub fn part_1(instructions: &[Instruction]) -> u32 {
    let mut on_cubes = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            'next: for z in -50..=50 {
                for instr in instructions.iter().rev() {
                    if x >= instr.x_min
                        && x <= instr.x_max
                        && y >= instr.y_min
                        && y <= instr.y_max
                        && z >= instr.z_min
                        && z <= instr.z_max
                    {
                        if instr.switch == Switch::On {
                            on_cubes += 1;
                        }
                        continue 'next;
                    }
                }
            }
        }
    }
    on_cubes
}

pub fn part_2(instructions: &[Instruction]) -> u64 {
    // let curr_instructions = None;
    // for instruction in instructions {
    //     // get the cube of the current element.
    //     // if the cube overlaps, split it and add all its children to the instructions
    //     // recursively do that for all
    // }
    // todo!();
    // let min_max = Instruction {
    //     switch: Switch::Off,
    //     x_min: i32::MAX,
    //     x_max: i32::MIN,
    //     y_min: i32::MAX,
    //     y_max: i32::MIN,
    //     z_min: i32::MAX,
    //     z_max: i32::MIN,
    // };

    // let min_max = instructions
    //     .iter()
    //     .fold(min_max, |acc, other| acc.abuse_as_min_max(other));

    // println!("min_max {}", &min_max);
    // // Is 20k^3 much?
    // let mut on_cubes = 0;
    // for x in min_max.x_min..=min_max.x_max {
    //     for y in min_max.y_min..=min_max.y_max {
    //         'next: for z in min_max.z_min..=min_max.z_max {
    //             for instr in instructions {
    //                 if x >= instr.x_min
    //                     && x <= instr.x_max
    //                     && y >= instr.y_min
    //                     && y <= instr.y_max
    //                     && z >= instr.z_min
    //                     && z <= instr.z_max
    //                 {
    //                     if instr.switch == Switch::On {
    //                         on_cubes += 1;
    //                     }
    //                     continue 'next;
    //                 }
    //             }
    //         }
    //     }
    // }
    42
}

// fn intersect_cube(&cube_1: Instruction, cube_2: Instruction) -> Vec<Instruction> {

// }
