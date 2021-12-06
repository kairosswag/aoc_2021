aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day4 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
    day6 : generator => part_1, part_2;
}

mod day6 {

    use std::collections::HashMap;

    pub fn generator(input: &str) -> Vec<u32> {
        input.split(",").map(|res| res.parse::<u32>().expect("no number")).collect()
    }

    pub fn part_1(fishes: &[u32]) -> u64 {
        let mut memo = HashMap::new();
        for i in 0..=8 {
            memo.insert((i, 0), 1);
        }
        let mut total = 0;
        for fish in fishes {
            total += sim_fishes(*fish, 80, &mut memo)
        }
        total
    }

    pub fn part_2(fishes: &[u32]) -> u64 {
        let mut memo = HashMap::new();
        for i in 0..=8 {
            memo.insert((i, 0), 1);
        }
        let mut total = 0;
        for fish in fishes {
            total += sim_fishes(*fish, 256, &mut memo)
        }
        total
    }

    fn sim_fishes(fish: u32, days_remaining: u32, memo: &mut HashMap<(u32, u32), u64>) -> u64 {
        if let Some(calc) = memo.get(&(fish, days_remaining)) {
            *calc
        } else {
            let total = if  fish == 0 {
                sim_fishes(8, days_remaining - 1, memo) +
                sim_fishes(6, days_remaining - 1, memo)
            } else {
                sim_fishes(fish - 1, days_remaining - 1, memo)
            };
            memo.insert((fish, days_remaining), total);
            total
        }
    }

    #[test]
    fn test() {
        let input = "3,4,3,1,2";
        assert_eq!(5934, part_1(&generator(&input)));
        assert_eq!(26984457539, part_2(&generator(&input)));
    }
}

mod day5 {

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
        for line_segment in line_segments.iter().filter(|&seg| seg.x1 == seg.x2 || seg.y1 == seg.y2) {
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

        for i in 0..= max_x {
            for j in 0..= max_y {
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
}
mod day4 {
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
}

mod day3 {

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DiagnosticNumber {
        digits: [u8; 12],
    }

    impl DiagnosticNumber {
        pub fn as_dec(&self) -> u32 {
            self.digits
                .iter()
                .rev()
                .enumerate()
                .filter(|&(_idx, val)| *val == 49)
                .fold(0, |accum, (significance, _value)| {
                    accum + 2u32.pow(significance as u32)
                })
        }
    }

    fn transmute(input: [u8; 12]) -> DiagnosticNumber {
        unsafe { std::mem::transmute::<[u8; 12], DiagnosticNumber>(input) }
    }

    pub fn generator(input: &str) -> Vec<DiagnosticNumber> {
        input
            .lines()
            .map(|l| transmute(l.as_bytes()[..12].try_into().expect("Not long enough")))
            .collect()
    }

    pub fn part_1(input: &[DiagnosticNumber]) -> u32 {
        let mut init = [0; 12];
        for number in input {
            for (idx, digit) in number.digits.iter().enumerate() {
                if *digit == 49 {
                    init[idx] += 1;
                }
            }
        }
        let treshold = input.len() / 2;
        let gamma = init
            .iter()
            .rev()
            .enumerate()
            .filter(|&(_significance, value)| *value > treshold)
            .fold(0, |accum, (significance, _value)| {
                accum + 2u32.pow(significance as u32)
            });

        let epsilon = 2u32.pow(12) - 1 - gamma;

        epsilon * gamma
    }

    pub fn part_2(input: &[DiagnosticNumber]) -> u32 {
        let oxygen_rating = sieve(input, 0, true);
        let scrubber_rating = sieve(input, 0, false);

        oxygen_rating.as_dec() * scrubber_rating.as_dec()
    }

    pub fn sieve(
        input: &[DiagnosticNumber],
        index: usize,
        use_more_common: bool,
    ) -> DiagnosticNumber {
        let comp = match (calc_msb_for_index(input, index), use_more_common) {
            // if ths is some logical pattern i do not get it
            (true, true) | (false, false) => 49,
            (false, true) | (true, false) => 48,
        };
        let numbers: Vec<DiagnosticNumber> = input
            .iter()
            .filter(|i| i.digits[index] == comp)
            .map(|s| s.to_owned())
            .collect();
        if numbers.len() == 1 {
            *numbers.get(0).unwrap()
        } else if numbers.is_empty() {
            panic!("This should not happen, nothing found");
        } else {
            sieve(&numbers, index + 1, use_more_common)
        }
    }

    pub fn calc_msb_for_index(input: &[DiagnosticNumber], index: usize) -> bool {
        input.iter().filter(|i| i.digits[index] == 49).count() >= input.len() / 2
    }
}

mod day2 {
    use parse_display::{Display, FromStr};

    #[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
    pub enum Command {
        #[display("forward {0}")]
        Forward(u32),
        #[display("down {0}")]
        Down(u32),
        #[display("up {0}")]
        Up(u32),
    }

    #[derive(Debug)]
    struct Position {
        horizontal: u32,
        depth: u32,
    }

    pub fn generator(input: &str) -> Vec<Command> {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    pub fn part_1(commands: &[Command]) -> u32 {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
        };
        for command in commands {
            match command {
                Command::Forward(forward) => pos.horizontal += forward,
                Command::Down(down) => pos.depth += down,
                Command::Up(up) => {
                    if up > &pos.depth {
                        panic!("Can't fly! since depth was {:?}", pos);
                    }
                    pos.depth -= up;
                }
            }
        }
        pos.depth * pos.horizontal
    }

    #[derive(Debug)]
    struct AimedPosition {
        horizontal: u32,
        depth: u32,
        aim: i32,
    }

    pub fn part_2(commands: &[Command]) -> u32 {
        let mut pos = AimedPosition {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        for command in commands {
            match command {
                Command::Forward(forward) => {
                    pos.horizontal += forward;
                    pos.depth = (pos.depth as i32 + pos.aim * *forward as i32) as u32;
                }
                Command::Down(down) => pos.aim += *down as i32,
                Command::Up(up) => pos.aim -= *up as i32,
            }
        }
        pos.depth * pos.horizontal
    }

    #[test]
    pub fn test() {
        let input = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";

        assert_eq!(150, part_1(&generator(input)));
        assert_eq!(900, part_2(&generator(input)));
    }
}

mod day1 {

    pub fn generator(input: &str) -> Vec<u32> {
        input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
    }

    pub fn part_1(lines: &[u32]) -> usize {
        lines
            .windows(2)
            .filter(|&items| items[0] < items[1])
            .count()
    }

    pub fn part_2(lines: &[u32]) -> usize {
        lines
            .windows(3)
            .map(|items| items[0] + items[1] + items[2])
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|&items| items[0] < items[1])
            .count()
    }

    #[test]
    pub fn test() {
        let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, part_1(&test_data));
        assert_eq!(5, part_2(&test_data));
    }
}
