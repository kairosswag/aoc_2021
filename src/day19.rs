use std::collections::VecDeque;

use fnv::{FnvHashMap, FnvHashSet};
use nalgebra::{Matrix3, Rotation3, Vector3};
use parse_display::{Display, FromStr};

#[derive(Debug, Clone)]
pub struct Scanner {
    _id: u32,
    beacons: FnvHashSet<Vector3<i32>>,
}

impl Scanner {
    pub fn new(id: u32) -> Scanner {
        let beacons = FnvHashSet::default();
        Scanner { _id: id, beacons }
    }
}

#[derive(Debug, Display, FromStr)]
#[display("--- scanner {id} ---")]
pub struct ScannerHeader {
    id: u32,
}

#[derive(Debug, Display, FromStr)]
#[display("{x},{y},{z}")]
pub struct Beacon {
    x: i32, 
    y: i32,
    z: i32,
}

pub fn generator(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut scanner: Option<Scanner> = None;
    for line in input.lines() {
        if let Some(ref mut value) = scanner {
            if line.is_empty() {
                scanners.push(value.clone());
                scanner = None;
            } else {
                let beacon: Beacon = line.parse().unwrap();
                value.beacons.insert(Vector3::new(beacon.x, beacon.y, beacon.z));
            }
        } else {
            let header: ScannerHeader = line.parse().unwrap();
            scanner.get_or_insert(Scanner::new(header.id));
        }
    }
    scanners
}

pub fn part_1(full_scan: &[Scanner]) -> usize {
    let mut found_beacons: FnvHashSet<Vector3<i32>> = FnvHashSet::default();
    full_scan[0].beacons.iter().for_each(|b| {
        found_beacons.insert(*b);
    });
  
    // TODO: match any scanner to each other since this method can swallow results
    let mut not_found: VecDeque<&Scanner> = full_scan[1..].iter().collect();
    'outer: while let Some(scan) = not_found.pop_back() {
        println!("Trying scanner {:?}", scan._id);
        for mode in 0..24 {
            let rotation = rotation(mode);
            let mut offsets: FnvHashMap<Vector3<i32>, u32> = FnvHashMap::default();
            for beacon in &scan.beacons {
                let rotated = rotation * beacon;
                for found_beacon in &found_beacons {
                    let offset: Vector3<i32> = rotated - *found_beacon;
                    *offsets.entry(offset).or_default() += 1;
                }
            }
            if let Some(offset) = offsets
                .iter()
                .filter(|(_m, &count)| count >= 12)
                .map(|(m, _c)| m)
                .next()
            {
                let offset: Vector3<i32> = *offset;
                for beacon in &scan.beacons {
                    let rotated = rotation * beacon;
                    let scan_zero_space = rotated - offset;
                    found_beacons.insert(scan_zero_space);
                }
                continue 'outer;
            }
        }
        not_found.push_front(scan);
    }

    found_beacons.len()
}

fn rotation(mode: u8) -> Rotation3<i32> {
    // turns = 0, 90, 180, 270
    let RotationValue {
        ca,
        cb,
        cy,
        sa,
        sb,
        sy,
    } = get_rotation_values(mode);

    let rota = Matrix3::new(
        ca * cb,
        ca * sb * sy - sa * cy,
        ca * sb * cy + sa * sy,
        sa * cb,
        sa * sb * sy + ca * cy,
        sa * sb * cy - ca * sy,
        -sb,
        cb * sy,
        cb * cy,
    );
    Rotation3::from_matrix_unchecked(rota)
}

fn get_rotation_values(mode: u8) -> RotationValue {
    match mode {
        0 => RotationValue::by_val(1, 1, 1, 0, 0, 0),
        1 => RotationValue::by_val(0, 1, 1, 1, 0, 0),
        2 => RotationValue::by_val(-1, 1, 1, 0, 0, 0),
        3 => RotationValue::by_val(0, 1, 1, -1, 0, 0),

        4 => RotationValue::by_val(1, 0, 1, 0, 1, 0),
        5 => RotationValue::by_val(0, 0, 1, 1, 1, 0),
        6 => RotationValue::by_val(-1, 0, 1, 0, 1, 0),
        7 => RotationValue::by_val(0, 0, 1, -1, 1, 0),

        8 => RotationValue::by_val(1, 0, 1, 0, -1, 0),
        9 => RotationValue::by_val(0, 0, 1, 1, -1, 0),
        10 => RotationValue::by_val(-1, 0, 1, 0, -1, 0),
        11 => RotationValue::by_val(0, 0, 1, -1, -1, 0),

        12 => RotationValue::by_val(1, 1, 0, 0, 0, 1),
        13 => RotationValue::by_val(0, 1, 0, 1, 0, 1),
        14 => RotationValue::by_val(-1, 1, 0, 0, 0, 1),
        15 => RotationValue::by_val(0, 1, 0, -1, 0, 1),

        16 => RotationValue::by_val(1, 1, -1, 0, 0, 0),
        17 => RotationValue::by_val(0, 1, -1, 1, 0, 0),
        18 => RotationValue::by_val(-1, 1, -1, 0, 0, 0),
        19 => RotationValue::by_val(0, 1, -1, -1, 0, 0),

        20 => RotationValue::by_val(1, 1, 0, 0, 0, -1),
        21 => RotationValue::by_val(0, 1, 0, 1, 0, -1),
        22 => RotationValue::by_val(-1, 1, 0, 0, 0, -1),
        23 => RotationValue::by_val(0, 1, 0, -1, 0, -1),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct RotationValue {
    ca: i32,
    cb: i32,
    cy: i32,
    sa: i32,
    sb: i32,
    sy: i32,
}

impl RotationValue {
    fn by_val(cx: i32, cy: i32, cz: i32, sx: i32, sy: i32, sz: i32) -> RotationValue {
        RotationValue {
            ca: cx,
            cb: cy,
            cy: cz,
            sa: sx,
            sb: sy,
            sy: sz,
        }
    }
}
