use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, Clone, Copy)]
#[display("{x},{y}")]
pub struct Dot {
    x: u32,
    y: u32,
}

#[derive(Debug, Display, FromStr, Clone, Copy)]
#[display("fold along {variant}={val}")]
pub struct Instruction {
    variant: Variant,
    val: u32,
}

#[derive(Debug, Display, FromStr, Clone, Copy)]
pub enum Variant {
    #[display("x")]
    VerticalFold,
    #[display("y")]
    HorizontalFold,
}

pub struct Manual {
    dots: Vec<Dot>,
    instructions: Vec<Instruction>,
}

impl Variant {
    pub fn fold_dot(&self, dot: (u32, u32), value: u32) -> (u32, u32) {
        match self {
            &Self::VerticalFold => Variant::fold_vertical(dot, value),
            &Self::HorizontalFold => Variant::fold_horizontal(dot, value),
        }
    }

    pub fn fold_vertical((a, b): (u32, u32), value: u32) -> (u32, u32) {
        if a > value {
            (value - (a - value), b)
        } else {
            (a, b)
        }
    }

    pub fn fold_horizontal((a, b): (u32, u32), value: u32) -> (u32, u32) {
        if b > value {
            (a, value - (b - value))
        } else {
            (a, b)
        }
    }
}

pub fn generator(input: &str) -> Manual {
    let mut dots = Vec::new();
    let mut instructions = Vec::new();
    let mut switched = false;
    for line in input.lines() {
        if line.is_empty() {
            switched = true;
            continue;
        }
        if !switched {
            dots.push(line.parse().unwrap());
        } else {
            instructions.push(line.parse().unwrap());
        }
    }
    Manual { dots, instructions }
}

pub fn part_1(manual: &Manual) -> usize {
    let mut coordinates = HashSet::new();

    manual.dots.iter().for_each(|dot| {
        coordinates.insert((dot.x, dot.y));
    });

    let first = manual.instructions.first().unwrap();

    let coordinates: HashSet<(u32, u32)> = coordinates
        .iter()
        .map(|c| first.variant.fold_dot(*c, first.val))
        .collect();

    coordinates.len()
}

pub fn part_2(manual: &Manual) -> usize {
    let mut coordinates = HashSet::new();

    manual.dots.iter().for_each(|dot| {
        coordinates.insert((dot.x, dot.y));
    });

    for instuction in &manual.instructions {
        coordinates = coordinates
            .iter()
            .map(|c| instuction.variant.fold_dot(*c, instuction.val))
            .collect();
    }

    let x_max = coordinates.iter().map(|(x, _y)| *x).max().unwrap();
    let y_max = coordinates.iter().map(|(_x, y)| *y).max().unwrap();
    for y in 0..=y_max {
        for x in 0..=x_max {
            if coordinates.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    42
}
