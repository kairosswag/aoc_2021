use fnv::FnvHashMap;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, Clone, Copy)]
#[display("Player {_id} starting position: {pos}")]
#[from_str(default_fields("score"))]
pub struct Player {
    _id: u32,
    pos: u32,
    score: u32,
}

pub fn generator(input: &str) -> Vec<Player> {
    input.lines().map(|p| p.parse().unwrap()).collect()
}

pub fn part_1(val: &[Player]) -> u32 {
    let mut p1 = val[0];
    let mut p2 = val[1];

    (1..)
        .tuples()
        .zip(1..)
        .flat_map(|((a, b, c), t)| easy_player_turn(a + b + c, &mut p1, &mut p2, t))
        .next()
        .unwrap()
}

fn easy_player_turn(
    eyes: u32,
    player_1: &mut Player,
    player_2: &mut Player,
    turn: u32,
) -> Option<u32> {
    let player_turn = |p1: &mut Player, p2: &mut Player, eyes, turn| {
        let next_pos = p2.pos + eyes;
        let next_pos = next_pos - ((next_pos - 1) / 10) * 10;
        p2.pos = next_pos;
        p2.score += p2.pos;
        if p2.score >= 1000 {
            Some(turn * 3 * p1.score)
        } else {
            None
        }
    };

    if turn % 2 == 0 {
        player_turn(player_1, player_2, eyes, turn)
    } else {
        player_turn(player_2, player_1, eyes, turn)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Universe {
    total_value_a: u8,
    total_value_b: u8,
    curr_pos_a: u8,
    curr_pos_b: u8,
}

impl Universe {
    pub fn new(total_value_a: u8, total_value_b: u8, curr_pos_a: u8, curr_pos_b: u8) -> Universe {
        Universe {
            total_value_a,
            total_value_b,
            curr_pos_a,
            curr_pos_b,
        }
    }

    pub fn expand(&self, quantity: u64) -> Vec<(Universe, u64)> {
        let mut universes = Vec::with_capacity(49);
        let universes_by_eyes = |eyes| match eyes {
            3 | 9 => 1,
            4 | 8 => 3,
            5 | 7 => 6,
            6 => 7,
            _ => unreachable!(),
        };
        for a_eyes in 3..=9 {
            let next_pos_a = &self.curr_pos_a + a_eyes;
            let next_pos_a = next_pos_a - ((next_pos_a - 1) / 10) * 10;
            assert!(next_pos_a > 0 && next_pos_a < 11);
            let next_total_a = &self.total_value_a + next_pos_a;
            if next_total_a >= 21 {
                let created_universe = Universe::new(next_total_a, 0, next_pos_a, 0);
                universes.push((created_universe, universes_by_eyes(a_eyes) * quantity));
            } else {
                for b_eyes in 3..=9 {
                    let next_pos_b = &self.curr_pos_b + b_eyes;
                    let next_pos_b = next_pos_b - ((next_pos_b - 1) / 10) * 10;
                    assert!(next_pos_b > 0 && next_pos_b < 11);
                    let next_total_b = &self.total_value_b + next_pos_b;
                    let created_universe =
                        Universe::new(next_total_a, next_total_b, next_pos_a, next_pos_b);
                    universes.push((
                        created_universe,
                        universes_by_eyes(a_eyes) * quantity * universes_by_eyes(b_eyes),
                    ))
                }
            }
        }

        universes
    }

    fn has_winner(&self) -> WinState {
        if self.total_value_a >= 21 {
            WinState::WinnerA
        } else if self.total_value_b >= 21 {
            WinState::WinnerB
        } else {
            WinState::NotYetDecided
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum WinState {
    WinnerA,
    WinnerB,
    NotYetDecided,
}

pub fn part_2(val: &[Player]) -> u64 {
    let p1 = val[0];
    let p2 = val[1];

    let mut curr_gen = FnvHashMap::default();
    curr_gen.insert(
        Universe::new(0, 0, p1.pos.try_into().unwrap(), p2.pos.try_into().unwrap()),
        1,
    );

    let mut a_won_universes = 0u64;
    let mut b_won_universes = 0u64;

    loop {
        println!(
            "CurrGenEntries {}\tSum: {}",
            curr_gen.len(),
            curr_gen.iter().map(|(_u, c)| c).sum::<u64>()
        );
        let mut next_gen = None;
        for (key, group) in &curr_gen
            .iter()
            .flat_map(|(univ, q)| univ.expand(*q))
            .group_by(|(univ, _count)| univ.has_winner())
        {
            use WinState::*;
            match key {
                WinnerA => {
                    a_won_universes += group.map(|(_u, count)| count as u64).sum::<u64>();
                }
                WinnerB => {
                    b_won_universes += group.map(|(_u, count)| count).sum::<u64>();
                }
                NotYetDecided => {
                    let collected_universes = group.group_by(|(universe, _count)| *universe);
                    let collected_universes = collected_universes
                        .into_iter()
                        .map(|(k, c)| (k, c.into_iter().map(|(_u, c)| c).sum::<u64>()));
                    next_gen = Some(FnvHashMap::<Universe, u64>::from_iter(collected_universes));
                }
            }
        }
        // let next_gen = FnvHashMap::from_iter(curr_gen.iter().flat_map(|(univ, q)| univ.expand(*q)));

        if next_gen.is_none() {
            break;
        }
        curr_gen = next_gen.unwrap();
    }

    a_won_universes.max(b_won_universes)
}

#[test]
fn test() {
    let input = "Player 1 starting position: 4\n\
    Player 2 starting position: 8";

    assert_eq!(739785, part_1(&generator(&input)));
    assert_eq!(444356092776315, part_2(&generator(&input)));
}
