pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn part_1(crabs: &[u32]) -> u32 {
    let lower = *crabs.iter().min().unwrap();
    let upper = *crabs.iter().max().unwrap();

    (lower..upper).map(|val| crabs.iter().map(|c| c.max(&val) - c.min(&val)).sum()).min().unwrap()
}

pub fn part_2(crabs: &[u32]) -> u64 {
    let lower = *crabs.iter().min().unwrap();
    let upper = *crabs.iter().max().unwrap();

    (lower..upper).map(|val| crabs.iter().map(|c| closed_sum(c.max(&val) - c.min(&val))).sum()).min().unwrap()
}

fn closed_sum(to: u32) -> u64 {
    (to as u64 * (to as u64 + 1)) / 2
}
