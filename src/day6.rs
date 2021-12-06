use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|res| res.parse::<u32>().expect("no number"))
        .collect()
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
        let total = if fish == 0 {
            sim_fishes(8, days_remaining - 1, memo) + sim_fishes(6, days_remaining - 1, memo)
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
