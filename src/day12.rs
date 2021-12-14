use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, Clone)]
#[display("{vert_a}-{vert_b}")]
pub struct Edge {
    pub vert_a: String,
    pub vert_b: String,
}

pub fn generator(
    input: &str,
) -> (
    FnvHashMap<u32, Vec<u32>>,
    FnvHashMap<u32, (bool, String)>,
    u32,
    u32,
) {
    let edges: Vec<Edge> = input.lines().map(|l| l.parse::<Edge>().unwrap()).collect();

    let nodes: FnvHashMap<u32, (bool, String)> = (0..)
        .zip(
            edges
                .iter()
                .flat_map(|e| [e.vert_a.clone(), e.vert_b.clone()].into_iter())
                .unique()
                .map(|v| (v.to_ascii_lowercase().eq(&v), v)),
        )
        .collect();

    let mut map: FnvHashMap<u32, Vec<u32>> = FnvHashMap::default();
    edges
        .iter()
        .map(|edge| {
            (
                get_reverse(&nodes, &edge.vert_a),
                get_reverse(&nodes, &edge.vert_b),
            )
        })
        .flat_map(|tuple| [(tuple.0, tuple.1), (tuple.1, tuple.0)].into_iter())
        .for_each(|entry| {
            map.entry(entry.0)
                .or_insert_with(|| Vec::new())
                .push(entry.1);
        });
    let start = get_reverse(&nodes, "start");
    let end = get_reverse(&nodes, "end");
    (map, nodes, start, end)
}

fn get_reverse(node_map: &FnvHashMap<u32, (bool, String)>, val: &str) -> u32 {
    *node_map.iter().find(|entry| entry.1 .1.eq(&val)).unwrap().0
}

pub fn part_1(
    (map, nodes, start, end): &(
        FnvHashMap<u32, Vec<u32>>,
        FnvHashMap<u32, (bool, String)>,
        u32,
        u32,
    ),
) -> u32 {
    let mut visited = FnvHashSet::default();
    // println!("map: {:?}", map);
    // println!("nodes: {:?}", nodes);
    visit_rec(*start, &map, &mut visited, &nodes, *end).unwrap()
}

pub fn part_2(
    (map, nodes, start, end): &(
        FnvHashMap<u32, Vec<u32>>,
        FnvHashMap<u32, (bool, String)>,
        u32,
        u32,
    ),
) -> u32 {
    let mut visited = FnvHashMap::default();
    visit_rec_2(
        *start,
        &map,
        &mut visited,
        &nodes,
        *start,
        *end,
        true,
    )
    .unwrap()
}

fn visit_rec(
    curr_node: u32,
    map: &FnvHashMap<u32, Vec<u32>>,
    visited: &mut FnvHashSet<u32>,
    lookup: &FnvHashMap<u32, (bool, String)>,
    end: u32,
) -> Option<u32> {
    if curr_node == end {
        return Some(1);
    }
    let add = |a, b| a + b;

    visited.insert(curr_node);

    let mut sub_paths = None;

    for node in map.get(&curr_node).unwrap() {
        if !lookup.get(node).unwrap().0 || !visited.contains(node) {
            let res = visit_rec(*node, map, visited, lookup, end);
            sub_paths = match (sub_paths, res) {
                (Some(a), Some(b)) => Some(add(a, b)),
                (sub_paths, res) => sub_paths.or(res),
            };
        }
    }

    visited.remove(&curr_node);

    sub_paths
}

fn visit_rec_2(
    curr_node: u32,
    map: &FnvHashMap<u32, Vec<u32>>,
    visited: &mut FnvHashMap<u32, u32>,
    lookup: &FnvHashMap<u32, (bool, String)>,
    start: u32,
    end: u32,
    allow_twice: bool,
) -> Option<u32> {
    if curr_node == end {
        return Some(1);
    }
    let add = |a, b| a + b;

    *visited.entry(curr_node).or_insert(0) += 1;

    let mut sub_paths = None;

    for node in map.get(&curr_node).unwrap() {
        let is_start = *node == start; // may never revisit

        let big_cave = !lookup.get(node).unwrap().0;
        let has_visited = visited.get(node).unwrap_or(&0) > &0;
        let is_end = *node == end; // may only revisit once

        if is_start {
            continue;
        } else if big_cave || is_end || !has_visited {
            let res = visit_rec_2(*node, map, visited, lookup, start, end, allow_twice);
            sub_paths = match (sub_paths, res) {
                (Some(a), Some(b)) => Some(add(a, b)),
                (sub_paths, res) => sub_paths.or(res),
            };
        } else if allow_twice {
            let res = visit_rec_2(*node, map, visited, lookup, start, end, false);
            sub_paths = match (sub_paths, res) {
                (Some(a), Some(b)) => Some(add(a, b)),
                (sub_paths, res) => sub_paths.or(res),
            };
        }
    }

    *visited.entry(curr_node).or_insert(1) -= 1;
    sub_paths
}

#[test]
pub fn test() {
    let input = "start-A\n\
    start-b\n\
    A-c\n\
    A-b\n\
    b-d\n\
    A-end\n\
    b-end";

    assert_eq!(10, part_1(&generator(&input)));
    assert_eq!(36, part_2(&generator(&input)));
}
