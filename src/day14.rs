use fnv::FnvHashMap;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
#[display("{in_a}{in_b} -> {out}")]
pub struct Rule {
    #[from_str(regex = "[A-Z]")]
    in_a: char,
    #[from_str(regex = "[A-Z]")]
    in_b: char,
    out: char,
}

impl Rule {
    pub fn as_tuple(&self) -> ((char, char), char) {
        ((self.in_a, self.in_b), self.out)
    }
}

pub fn generator(input: &str) -> (&str, FnvHashMap<(char, char), char>) {
    let mut lines = input.lines();
    let initial = lines.next().unwrap();
    lines.next();
    (
        initial,
        lines
            .map(|line| line.parse::<Rule>().unwrap().as_tuple())
            .collect(),
    )
}

pub fn part_1(input: &(&str, FnvHashMap<(char, char), char>)) -> u32 {
    let map = &input.1;

    let mut curr_str = input.0.to_owned();
    for _ in 1..=10 {
        let mut next = Vec::new();
        next.push(curr_str.chars().next().unwrap());
        curr_str.chars().tuple_windows().for_each(|(a, b)| {
            if let Some(val) = map.get(&(a, b)) {
                next.push(*val);
            }
            next.push(b);
        });

        curr_str = next.iter().collect();
    }

    let mut char_counts = FnvHashMap::default();
    curr_str.chars().for_each(|c| *char_counts.entry(c).or_insert(0) += 1);

    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    max - min
}

pub fn part_2(input: &(&str, FnvHashMap<(char, char), char>)) -> u64 {
    let map = &input.1;
    let curr_str = input.0.to_owned();

    let mut tuples = FnvHashMap::default();
    curr_str.chars().tuple_windows().for_each(|tuple: (char, char)| {
        *tuples.entry(tuple).or_insert(0) += 1;
    });

    for _ in 1..=40 {
        let mut tuples_collected = FnvHashMap::default();
        tuples.iter().flat_map(|(tuple, count)| {
            if let Some(val) = map.get(tuple) {
                vec![((tuple.0, *val), count),((*val, tuple.1), count)]
            } else {
                vec![((tuple.0, tuple.1), count)]
            }
        }).for_each(|(tuple, count)| {
            *tuples_collected.entry(tuple).or_insert(0) += count;
        });

        tuples = tuples_collected;
    }

    let mut counter = FnvHashMap::default();
    counter.insert(curr_str.chars().next().unwrap(), 1);
    tuples.iter().for_each(|(tuple, count)| {
        *counter.entry(tuple.1).or_insert(0) += count;
    });
    let max = counter.values().max().unwrap();
    let min = counter.values().min().unwrap();

    max - min
}

#[test]
pub fn test() {
    let input = "NNCB\n\
\n\
    CH -> B\n\
    HH -> N\n\
    CB -> H\n\
    NH -> C\n\
    HB -> C\n\
    HC -> B\n\
    HN -> C\n\
    NN -> C\n\
    BH -> H\n\
    NC -> B\n\
    NB -> B\n\
    BN -> B\n\
    BB -> N\n\
    BC -> B\n\
    CC -> N\n\
    CN -> C";

    assert_eq!(1588, part_1(&generator(&input)));
    assert_eq!(2188189693529, part_2(&generator(&input)));
}
