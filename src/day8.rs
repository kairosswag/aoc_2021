use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct Entry<'a> {
    pub input: Vec<&'a str>,
    pub output: Vec<&'a str>,
}

impl Entry<'_> {
    pub fn from_strs<'a>(a: &'a str, b: &'a str) -> Entry<'a> {
        Entry {
            input: a.split_ascii_whitespace().collect(),
            output: b.split_ascii_whitespace().collect(),
        }
    }
}
pub fn generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|f| {
            let mut split = f.split("|");
            let input = split.next().unwrap();
            let output = split.next().unwrap();
            Entry::from_strs(input, output)
        })
        .collect()
}

pub fn part_1(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .flat_map(|val| corr_digit(val.len()))
                .count()
        })
        .sum()
}

pub fn part_2(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|entry| {
            let mut groupings: HashMap<usize, HashSet<char>> = HashMap::new();
            for el in entry.input.iter() {
                let entry = groupings
                    .entry(el.len())
                    .or_insert_with(|| HashSet::from_iter(el.chars()));
                entry.retain(|val| el.contains(|v| v == *val));
            }

            let spaces: HashMap<char, char> = groupings
                .iter()
                .map(|(bucket, set)| (bucket, set.iter().collect::<String>()))
                .map(|(bucket, str)| get_possibilities(&str, *bucket))
                .reduce(|mut a, b| {
                    a.retain(|val| b.contains(val));
                    a
                })
                .unwrap()
                .into_iter()
                .collect();

            // println!("entry: {:?}", &entry);
            // for char in all_chars {
            //     println!(
            //         "spaace {}: {:?}",
            //         char,
            //         spaces
            //             .iter()
            //             .filter(|(a, _b)| *a == char)
            //             .collect::<Vec<&(char, char)>>()
            //     );
            // }
            (&entry.output, spaces)
        })
        .flat_map(|(output, decoder)| {
            output
                .iter()
                .enumerate()
                .map(move |(idx, word)| (idx, decode(word, &decoder)))
                .map(|(pos, decoded)| to_number(&decoded, pos))
        })
        .sum()
}

fn decode(word: &str, decoder: &HashMap<char, char>) -> String {
    let res = word
        .chars()
        .map(|c| *decoder.get(&c).unwrap())
        .sorted()
        .collect::<String>();
    res
}

fn to_number(instr: &str, pos: usize) -> u32 {
    let res = match instr {
        val_0 => 0,
        val_1 => 1,
        val_2 => 2,
        val_3 => 3,
        val_4 => 4,
        val_5 => 5,
        val_6 => 6,
        val_7 => 7,
        val_8 => 8,
        val_9 => 9,
        _ => panic!("nah, {} not recognized", instr),
    };
    res * u32::pow(10, 3-pos as u32)
}

const val_0: &'static str = "abcefg";
const val_1: &'static str = "cf";
const val_2: &'static str = "acdeg";
const val_3: &'static str = "acdfg";
const val_4: &'static str = "bcdf";
const val_5: &'static str = "abdfg";
const val_6: &'static str = "abdefg";
const val_7: &'static str = "acf";
const val_8: &'static str = "abcdefg";
const val_9: &'static str = "abcdfg";

static all_chars: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

fn get_possibilities(val: &str, bucket: usize) -> HashSet<(char, char)> {
    let mut possibilities = HashSet::new();
    let val_c: Vec<char> = val.chars().collect();
    let target_c = match bucket {
        2 => vec!['c', 'f'],                          // 1
        3 => vec!['a', 'c', 'f'],                     // 7
        4 => vec!['b', 'c', 'd', 'f'],                // 4
        5 => vec!['a', 'd', 'g'],                     // 2, 3, 5
        6 => vec!['a', 'b', 'g', 'f'],                // 6, 9, 0
        7 => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], // 8
        1 | _ => panic!("eh"),
    };
    for cipher in all_chars {
        for plain in all_chars {
            // fixed: add all tuples (val*, target*)
            // fixed: add all tuples (!val*, !target*)

            // moving: add all tuples (val*, target*)
            // moving: add all tuples (e*, e*)

            // moving: val_a retain val_b retain val_c => 6
            if (val_c.contains(&cipher) && target_c.contains(&plain))
                || (!val_c.contains(&cipher) && !target_c.contains(&plain))
            {
                possibilities.insert((cipher, plain));
            }
        }
    }

    possibilities
}

#[test]
fn test_eight() {
    //["dbcfeag", "cgaed", "fe", "bfgad", "aefcdb", "efa", "efgda", "gcef", "dcaebg", "dfeagc"]
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    assert_eq!(5353, part_2(&generator(&input)));
}

pub fn corr_digit(length: usize) -> Option<u8> {
    Some(match length {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => return None,
    })
}

#[test]
pub fn test() {
    let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |\
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |\
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |\
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |\
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |\
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |\
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |\
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |\
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |\
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |\
fgae cfgab fg bagce";
    let small_input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

    assert_eq!(2, part_1(&generator(&small_input)));
}

// fn resolve_candidates(candidates: &mut HashMap<char, HashSet<char>>, entry: &[&str]) -> u32 {
//     let mut results = HashMap::new();
//     let mut notyetfound: HashSet<char> = HashSet::from_iter("abcdefg".chars());
//     loop {
//         // find single value
//         if let Some((&key, &value)) = get_candidate_with_single_remaining(&candidates) {
//             results.insert(key, value);
//             notyetfound.remove(&key);
//             candidates.remove(&key);
//             candidates.values_mut().for_each(|map| {
//                 map.remove(&key);
//             });
//         } else if let Some((key, value)) = get_candidate_for_remaining_slot(
//             &candidates,
//             &notyetfound,
//             &results.keys().collect::<Vec<&char>>(),
//         ) {
//             results.insert(key, value);
//             notyetfound.remove(&key);
//             candidates.remove(&key);
//             candidates.values_mut().for_each(|map| {
//                 map.remove(&key);
//             });
//         } else {
//             println!("results {:?}", results);
//             println!("notyetfound {:?}", notyetfound);
//             println!("candidates {:?}", candidates);
//             panic!("dunno");
//         }

//         if results.len() == 7 {
//             return 5;
//         }
//     }
// }

// fn get_candidate_with_single_remaining(
//     candidates: &HashMap<char, HashSet<char>>,
// ) -> Option<(&char, &char)> {
//     candidates
//         .iter()
//         .filter(|(_key, set)| set.len() == 1)
//         .map(|(key, set)| (key, set.iter().next().unwrap()))
//         .next()
// }

// fn get_candidate_for_remaining_slot(
//     candidates: &HashMap<char, HashSet<char>>,
//     notyetfound: &HashSet<char>,
//     found: &[&char],
// ) -> Option<(char, char)> {
//     let remaining: Vec<&char> = candidates
//         .values()
//         .flat_map(|val| val.iter())
//         .filter(|val| notyetfound.contains(val))
//         .collect();
//     if remaining.len() != 1 {
//         return None;
//     }
//     let singular = remaining[0];

//     let mut hole = None;
//     for char in "abcdefg".chars().filter(|c| !found.contains(&c)) {
//         if candidates.get(&char).is_none() {
//             if hole.is_none() {
//                 hole = Some(char);
//             } else {
//                 // more than one hole
//                 return None;
//             }
//         }
//     }

//     Some((hole?, *singular))
// }
