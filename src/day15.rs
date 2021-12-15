use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 48).collect())
        .collect()
}

pub fn part_1(cave: &Vec<Vec<u8>>) -> u32 {
    let end_x = cave.len();
    let end_y = cave[0].len();

    let mut edges: FnvHashSet<(usize, usize)> = FnvHashSet::default();
    for x in 0..cave.len() {
        for y in 0..cave[0].len() {
            edges.insert((x, y));
        }
    }
    let mut distance: FnvHashMap<(usize, usize), u32> = FnvHashMap::default();
    distance.insert((0, 0), 0);
    let mut predecessor = FnvHashMap::default();

    dijkstra(
        Some((end_x - 1, end_y - 1)),
        &|(x, y): (usize, usize)| cave[x][y] as u32,
        &mut edges,
        &mut distance,
        &mut predecessor,
    )
    .unwrap()
}

pub fn part_2(cave: &Vec<Vec<u8>>) -> u32 {
    let end_x = cave.len();
    let end_y = cave[0].len();

    let mut edges: FnvHashSet<(usize, usize)> = FnvHashSet::default();
    for x in 0..end_x * 5 {
        for y in 0..end_y * 5 {
            edges.insert((x, y));
        }
    }
    let mut distance: FnvHashMap<(usize, usize), u32> = FnvHashMap::default();
    distance.insert((0, 0), 0);
    let mut predecessor = FnvHashMap::default();

    dijkstra(
        Some((end_x*5 - 1, end_y*5 - 1)),
        &|point: (usize, usize)| calc_danger(point, cave, end_x, end_y),
        &mut edges,
        &mut distance,
        &mut predecessor,
    )
    .unwrap()
}

fn calc_danger((x, y): (usize, usize), cave: &Vec<Vec<u8>>, clx: usize, cly: usize) -> u32 {
    let danger = cave[x%clx][y%cly];
    let x_add = x / clx;
    let y_add = y / clx;
    let res = danger as u32 + x_add as u32 + y_add as u32;
    if res > 9 {
        res - 9
    } else {
        res
    }
}


fn dijkstra(
    end: Option<(usize, usize)>,
    cave_access: &dyn Fn((usize, usize)) -> u32,
    edges: &mut FnvHashSet<(usize, usize)>,
    distance: &mut FnvHashMap<(usize, usize), u32>,
    predecessor: &mut FnvHashMap<(usize, usize), (usize, usize)>,
) -> Option<u32> {
    while !edges.is_empty() {
        let (&curr_node, curr_dist) = distance
            .iter()
            .filter(|(key, _value)| edges.contains(key))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap();

        if end.map_or(false, |end_val| end_val == curr_node) {
            return Some(*curr_dist);
        }
        edges.remove(&curr_node);
        for neighbor in get_adjacent(curr_node) {
            if edges.contains(&neighbor) {
                update_distance(
                    curr_node,
                    neighbor,
                    cave_access(neighbor),
                    distance,
                    predecessor,
                );
            }
        }
    }

    None
}

fn update_distance(
    u: (usize, usize),
    v: (usize, usize),
    dist_u_v: u32,
    distance: &mut FnvHashMap<(usize, usize), u32>,
    predecessor: &mut FnvHashMap<(usize, usize), (usize, usize)>,
) {
    let alt_distance = distance.get(&u).unwrap() + dist_u_v;
    if distance
        .get(&v)
        .map_or(true, |curr_dist| alt_distance < *curr_dist)
    {
        distance.insert(v, alt_distance);
        predecessor.insert(v, u);
    }
}

fn get_adjacent((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    adjacent.push((x, y + 1));
    adjacent.push((x + 1, y));
    if x > 0 {
        adjacent.push((x - 1, y));
    }
    if y > 0 {
        adjacent.push((x, y - 1));
    }
    adjacent
}
