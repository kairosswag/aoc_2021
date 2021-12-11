pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u32 - 48).collect())
        .collect()
}

pub fn part_1(val: &[Vec<u32>]) -> u32 {
    let mut val: Vec<Vec<u32>> = val.iter().map(|line| line.clone()).collect();
    let mut spark_count = 0;

    for _ in 1..=100 {
        // increase all by 1
        val.iter_mut()
            .flat_map(|l: &mut Vec<u32>| l.iter_mut())
            .for_each(|val: &mut u32| *val += 1);

        let mut adjacent = Vec::new();

        // get initial sparks
        for row in 0..val.len() {
            for column in 0..val[0].len() {
                if let Some(10) = safe_get(&mut val, row as i32, column as i32) {
                    spark_count += 1;
                    handle_spark(&mut adjacent, row as i32, column as i32);
                }
            }
        }

        // proceed until all sparks are done
        while let Some((row, column)) = adjacent.pop() {
            if let Some(val) = safe_get(&mut val, row, column) {
                *val += 1;
                if *val == 10 {
                    spark_count += 1;
                    handle_spark(&mut adjacent, row, column);
                }
            }
        }

        val.iter_mut()
            .flat_map(|l| l.iter_mut())
            .for_each(|val| *val = wrap_at(*val, 9));
    }

    spark_count
}

pub fn part_2(val: &[Vec<u32>]) -> u32 {
    let mut val: Vec<Vec<u32>> = val.iter().map(|line| line.clone()).collect();
    
    for step in 1.. {
        let mut spark_count = 0;
        // increase all by 1
        val.iter_mut()
            .flat_map(|l: &mut Vec<u32>| l.iter_mut())
            .for_each(|val: &mut u32| *val += 1);

        let mut adjacent = Vec::new();

        // get initial sparks
        for row in 0..val.len() {
            for column in 0..val[0].len() {
                if let Some(10) = safe_get(&mut val, row as i32, column as i32) {
                    spark_count += 1;
                    handle_spark(&mut adjacent, row as i32, column as i32);
                }
            }
        }

        // proceed until all sparks are done
        while let Some((row, column)) = adjacent.pop() {
            if let Some(val) = safe_get(&mut val, row, column) {
                *val += 1;
                if *val == 10 {
                    spark_count += 1;
                    handle_spark(&mut adjacent, row, column);
                }
            }
        }

        if spark_count == 100 {
            return step;
        }

        val.iter_mut()
            .flat_map(|l| l.iter_mut())
            .for_each(|val| *val = wrap_at(*val, 9));

        if step > 1_000_000 {
            panic!("not yet found!");
        }

    }
    unreachable!()
}

fn handle_spark(adjacent: &mut Vec<(i32, i32)>, i: i32, j: i32) {
    adjacent.push((i, j - 1)); // N
    adjacent.push((i + 1, j - 1)); // NE
    adjacent.push((i + 1, j)); // E
    adjacent.push((i + 1, j + 1)); // SE
    adjacent.push((i, j + 1)); // S
    adjacent.push((i - 1, j + 1)); //SW
    adjacent.push((i - 1, j)); // W
    adjacent.push((i - 1, j - 1)); // NW
}

pub fn safe_get(input: &mut [Vec<u32>], xpos: i32, ypos: i32) -> Option<&mut u32> {
    let xpos = if xpos < 0 {
        return None;
    } else {
        xpos as usize
    };

    let ypos = if ypos < 0 {
        return None;
    } else {
        ypos as usize
    };
    input.get_mut(xpos)?.get_mut(ypos)
}

pub fn wrap_at(val: u32, max_val: u32) -> u32 {
    if val > max_val {
        0
    } else {
        val
    }
}

#[test]
pub fn test() {
    let input = "5483143223\n\
    2745854711\n\
    5264556173\n\
    6141336146\n\
    6357385478\n\
    4167524645\n\
    2176841721\n\
    6882881134\n\
    4846848554\n\
    5283751526";

    assert_eq!(1656, part_1(&generator(&input)));
    assert_eq!(195, part_2(&generator(&input)));
}
