use parse_display::{Display, FromStr};

#[derive(Debug, Clone, Copy, Display, FromStr)]
#[display("target area: x={x1}..{x2}, y={y1}..{y2}")]
pub struct TargetRectangle {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

pub fn generator(input: &str) -> TargetRectangle {
    input.parse().unwrap()
}

pub fn part_1(input: &TargetRectangle) -> i32 {
    let y_target = input.y1..input.y2;
    let x_target = input.x1..input.x2;

    // highest x is when it clears the whole range in two steps -> 2x-1 > max_x_target
    let max_x = (input.x2 + 1) / 2;

    for y_val in (0..=input.y1.abs()).rev() {
        for n_val in y_val..input.x2 {
            if y_target.contains(&y_at(y_val, n_val)) {
                for x_val in 1..max_x {
                    if x_target.contains(&x_at(x_val, n_val)) {
                        return gauss(y_val);
                    }
                }
            }
        }
    }
    unreachable!();
}


#[test]
pub fn test2() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(112, part_2(&generator(&input)));
}

pub fn part_2(input: &TargetRectangle) -> i32 {
    let y_target = input.y1..=input.y2;
    let x_target = input.x1..=input.x2;

    let max_x = input.x2;

    let mut counter = 0;

    for y_val in input.y1..=input.y1.abs() {
        'xloop: for x_val in 1..=max_x {
            for n_val in 0..input.x2 {
                if y_target.contains(&y_at(y_val, n_val)) {
                    if x_target.contains(&x_at(x_val, n_val)) {
                        counter += 1;
                        continue 'xloop;
                    }
                }
            }
        }
    }
    counter
}

fn x_at(x_vel: i32, n: i32) -> i32 {
    if x_vel > n {
        (n + 1) * x_vel - gauss(n)
    } else {
        gauss(x_vel)
    }
}

fn y_at(y_vel: i32, n: i32) -> i32 {
    (n + 1) * y_vel - gauss(n)
}

fn gauss(i: i32) -> i32 {
    (i * (i + 1)) / 2
}

#[test]
pub fn test() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(45, part_1(&generator(&input)));
    // assert_eq!(112, part_2(&generator(&input)));
}
