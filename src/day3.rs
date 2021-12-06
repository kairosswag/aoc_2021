
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

pub fn sieve(input: &[DiagnosticNumber], index: usize, use_more_common: bool) -> DiagnosticNumber {
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
