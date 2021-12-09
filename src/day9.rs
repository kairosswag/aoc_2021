pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

pub fn part_1(input: &[Vec<u8>]) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..input.len() {
        for j in 0..input[1].len() {
            let spot = safe_get(input, i, j).unwrap();
            let north = safe_get(input, i, j.wrapping_sub(1));
            let south = safe_get(input, i, j.wrapping_add(1));
            let west = safe_get(input, i.wrapping_sub(1), j);
            let east = safe_get(input, i.wrapping_add(1), j);
            if is_min(spot, north)
                && is_min(spot, south)
                && is_min(spot, east)
                && is_min(spot, west)
            {
                sum += 1u32 + *spot as u32;
            }
        }
    }
    sum
}

pub fn part_2(input: &[Vec<u8>]) -> u32 {
    let mut world: Vec<Vec<u8>> = input.iter().map(|row| row.clone()).collect();
    let mut sums = Vec::new();
    for low_point in find_low_points(input) {
        let mut sum = 0;

        let mut adjacent = Vec::new();
        adjacent.push(low_point);

        'inner: while !adjacent.is_empty() {
            let (n_x, n_y) = adjacent.pop().unwrap();
            if let Some(&val) = safe_get(&world, n_x, n_y) {
                if val == 9 {
                    continue 'inner;
                }

                world[n_x][n_y] = 9;
                sum += 1;

                adjacent.push((n_x.wrapping_add(1), n_y));
                adjacent.push((n_x.wrapping_sub(1), n_y));
                adjacent.push((n_x, n_y.wrapping_add(1)));
                adjacent.push((n_x, n_y.wrapping_sub(1)));
            }
        }

        sums.push(sum);
    }
    sums.sort();
    sums.pop().unwrap() * sums.pop().unwrap() * sums.pop().unwrap()
}

fn find_low_points(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[1].len() {
            let spot = safe_get(input, i, j).unwrap();
            let north = safe_get(input, i, j.wrapping_sub(1));
            let south = safe_get(input, i, j.wrapping_add(1));
            let west = safe_get(input, i.wrapping_sub(1), j);
            let east = safe_get(input, i.wrapping_add(1), j);
            if is_min(spot, north)
                && is_min(spot, south)
                && is_min(spot, east)
                && is_min(spot, west)
            {
                low_points.push((i, j));
            }
        }
    }
    low_points
}

pub fn safe_get(input: &[Vec<u8>], xpos: usize, ypos: usize) -> Option<&u8> {
    input.get(xpos)?.get(ypos)
}

pub fn is_min(spot: &u8, val: Option<&u8>) -> bool {
    val.map(|v| spot < v).unwrap_or(true)
}

#[test]
fn test() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(15, part_1(&generator(&input)));
}
