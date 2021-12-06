
use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{x1},{y1} -> {x2},{y2}")]
pub struct LineSegment {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

pub fn generator(input: &str) -> Vec<LineSegment> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(line_segments: &[LineSegment]) -> usize {
    let mut point_map = HashMap::new();
    for line_segment in line_segments
        .iter()
        .filter(|&seg| seg.x1 == seg.x2 || seg.y1 == seg.y2)
    {
        let mut cursor_x = line_segment.x1;
        let mut cursor_y = line_segment.y1;

        'add_to_map: loop {
            *point_map.entry((cursor_x, cursor_y)).or_insert(0) += 1;

            if cursor_x > line_segment.x2 {
                cursor_x -= 1;
            } else if cursor_x < line_segment.x2 {
                cursor_x += 1;
            } else if cursor_y > line_segment.y2 {
                cursor_y -= 1;
            } else if cursor_y < line_segment.y2 {
                cursor_y += 1;
            } else {
                break 'add_to_map;
            }
        }
    }
    point_map.values().filter(|&&val| val >= 2).count()
}

pub fn part_2(line_segments: &[LineSegment]) -> usize {
    let mut point_map = HashMap::new();
    for line_segment in line_segments.iter() {
        let mut cursor_x = line_segment.x1;
        let mut cursor_y = line_segment.y1;

        'add_to_map: loop {
            *point_map.entry((cursor_x, cursor_y)).or_insert(0) += 1;

            if cursor_x == line_segment.x2 && cursor_y == line_segment.y2 {
                break 'add_to_map;
            }

            if cursor_x > line_segment.x2 {
                cursor_x -= 1;
            } else if cursor_x < line_segment.x2 {
                cursor_x += 1;
            }

            if cursor_y > line_segment.y2 {
                cursor_y -= 1;
            } else if cursor_y < line_segment.y2 {
                cursor_y += 1;
            }
        }
    }
    point_map.values().filter(|&&val| val >= 2).count()
}

fn _print_debug(point_map: &HashMap<(u32, u32), i32>) {
    let max_x = *point_map.keys().map(|(x, _y)| x).max().unwrap();
    let max_y = *point_map.keys().map(|(_x, y)| y).max().unwrap();

    for i in 0..=max_x {
        for j in 0..=max_y {
            if let Some(val) = point_map.get(&(i, j)) {
                print!("{}", val);
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
}

#[test]
fn test() {
    let input = "0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2";

    assert_eq!(5, part_1(&generator(&input)));
    assert_eq!(12, part_2(&generator(&input)));
}
