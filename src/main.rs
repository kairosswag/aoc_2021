aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
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
                },
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
