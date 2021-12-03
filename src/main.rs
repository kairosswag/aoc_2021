aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
}

mod day3 {

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DiagnosticNumber {
        digits: [u8; 12],
    }

    impl DiagnosticNumber {

        pub fn to_dec(&self) -> u32 {
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

        oxygen_rating.to_dec() * scrubber_rating.to_dec()
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
            return *numbers.get(0).unwrap();
        } else if numbers.len() == 0 {
            panic!("This should not happen, nothing found");
        } else {
            return sieve(&numbers, index + 1, use_more_common);
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
