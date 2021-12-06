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
