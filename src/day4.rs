
use core::panic;

#[derive(Debug)]
pub struct BingoSystem {
    chosen: Vec<u32>,
    boards: Vec<Board>,
}

type Board = [[u32; 5]; 5];

trait Wincondition {
    fn wins(&self, values: &[u32]) -> Option<u32>;
    fn check_horizontal(&self, values: &[u32]) -> bool;
    fn check_vertical(&self, values: &[u32]) -> bool;
    // fn check_diagonal(&self, values: &[u32]) -> bool;
}

impl Wincondition for Board {
    fn wins(&self, values: &[u32]) -> Option<u32> {
        if self.check_horizontal(values) || self.check_vertical(values)
        // || self.check_diagonal(values)
        {
            Some(
                self.iter()
                    .flat_map(|l| l.iter())
                    .filter(|&v| !values.contains(v))
                    .sum(),
            )
        } else {
            None
        }
    }

    fn check_horizontal(&self, values: &[u32]) -> bool {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if !values.contains(&self[i][j]) {
                    continue 'outer;
                }
            }
            // println!("found in horizontal row {}", i);
            return true;
        }
        false
    }

    // fn check_diagonal(&self, values: &[u32]) -> bool {
    //     // (values.contains(&self[0][0])
    //     //     && values.contains(&self[1][1])
    //     //     && values.contains(&self[2][2])
    //     //     && values.contains(&self[3][3])
    //     //     && values.contains(&self[4][4]))
    //     //     || (values.contains(&self[4][0])
    //     //         && values.contains(&self[3][1])
    //     //         && values.contains(&self[2][2])
    //     //         && values.contains(&self[1][3])
    //     //         && values.contains(&self[0][4]))
    //     panic!("Does not exist really");
    // }

    fn check_vertical(&self, values: &[u32]) -> bool {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if !values.contains(&self[j][i]) {
                    continue 'outer;
                }
            }
            // println!("found in vertical column {}", i);
            return true;
        }
        false
    }
}

pub fn generator(input: &str) -> BingoSystem {
    let mut lines = input.lines();

    let chosen = lines
        .next()
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        if let Some(board) = get_board(&mut lines) {
            boards.push(board);
        }
    }

    BingoSystem { chosen, boards }
}

fn get_board(lines: &mut std::str::Lines) -> Option<Board> {
    let mut board = [[0; 5]; 5];
    for i in 0..5 {
        let line = lines.next()?;
        if !(line.len() > 1) {
            return None;
        }
        line.split_whitespace().enumerate().for_each(|(idx, val)| {
            board[i][idx] = val
                .parse::<u32>()
                .expect(&(val.to_owned() + " could not be parsed in line " + line))
        });
    }
    Some(board)
}

pub fn part_1(system: &BingoSystem) -> u32 {
    for i in 4..system.chosen.len() {
        // println!(
        //     "Trying with {:?} and last value {}",
        //     &system.chosen[..=i],
        //     &system.chosen[i]
        // );
        for board in &system.boards {
            if let Some(res) = board.wins(&system.chosen[..=i]) {
                // println!("board {:?}", board);
                // println!("wins with value {}", res);
                return res * system.chosen[i];
            }
        }
    }
    panic!("No result found!")
}

pub fn part_2(system: &BingoSystem) -> u32 {
    let mut boards = system.boards.clone();
    for i in 4..system.chosen.len() {
        if boards.len() > 1 {
            boards.retain(|board| board.wins(&system.chosen[..=i]).is_none());
        } else {
            if let Some(res) = boards[0].wins(&system.chosen[..=i]) {
                return res * system.chosen[i];
            }
        }
    }
    panic!("No result found!")
}

#[test]
pub fn test() {
    let test_input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    assert_eq!(4512, part_1(&generator(&test_input)));
}
