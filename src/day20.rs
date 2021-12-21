use fnv::FnvHashSet;

pub struct Input {
    light_lookup: Vec<bool>,
    initial_image: FnvHashSet<(i32, i32)>,
}

pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let lookup = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    lines.next();
    let is_point = |x, y, c| {
        if c == '#' {
            Some((x, y))
        } else {
            None
        }
    };
    let initial_image = lines
        .zip(0..)
        .flat_map(|(line, x_idx)| {
            line.chars()
                .zip(1..)
                .flat_map(move |(char, y_idx)| is_point(x_idx, y_idx, char))
        })
        .collect();

    Input {
        light_lookup: lookup,
        initial_image,
    }
}

pub fn part_1(input: &Input) -> usize {
    let mut curr = input.initial_image.clone();

    let mut dark_mode = false;

    for _ in 0..2 {
        let x_max = curr.iter().map(|(x, _y)| *x).max().unwrap();
        let x_min = curr.iter().map(|(x, _y)| *x).min().unwrap();
        let y_max = curr.iter().map(|(_x, y)| *y).max().unwrap();
        let y_min = curr.iter().map(|(_x, y)| *y).min().unwrap();
        let mut next = FnvHashSet::default();

        for x in x_min - 20..x_max + 20 {
            for y in y_min - 20..y_max + 20 {
                let x8 = get_value(x - 1, y - 1, &curr, 8, dark_mode);
                let x7 = get_value( x - 1, y, &curr, 7, dark_mode);
                let x6 = get_value( x - 1, y + 1, &curr, 6, dark_mode);
                let x5 = get_value( x, y - 1, &curr, 5, dark_mode);
                let x4 = get_value( x, y, &curr, 4, dark_mode);
                let x3 = get_value( x, y + 1, &curr, 3, dark_mode);
                let x2 = get_value( x + 1, y - 1, &curr, 2, dark_mode);
                let x1 = get_value( x + 1, y, &curr, 1, dark_mode);
                let x0 = get_value( x + 1, y + 1, &curr, 0, dark_mode);

                if input.light_lookup[x8 + x7 + x6 + x5 + x4 + x3 + x2 + x1 + x0] == dark_mode {
                    next.insert((x, y));
                }
            }
        }
        
        //####################..................##.#######.########..#...#..#..###...#...###.#.##.###..###.#.##....###..#.############...#.###...##.###.................#.##################
        // _print_map(&curr);
        dark_mode = !dark_mode;
        curr = next;
    }
    _print_map(&curr);

    let x_max = curr.iter().map(|(x, _y)| *x).max().unwrap();
    let x_min = curr.iter().map(|(x, _y)| *x).min().unwrap();
    let y_max = curr.iter().map(|(_x, y)| *y).max().unwrap();
    let y_min = curr.iter().map(|(_x, y)| *y).min().unwrap();

    // curr.iter().filter((x, y) x ).len()
    curr.len()
}


pub fn part_2(input: &Input) -> usize {
    let mut curr = input.initial_image.clone();

    let mut dark_mode = false;

    for _ in 0..50 {
        let x_max = curr.iter().map(|(x, _y)| *x).max().unwrap();
        let x_min = curr.iter().map(|(x, _y)| *x).min().unwrap();
        let y_max = curr.iter().map(|(_x, y)| *y).max().unwrap();
        let y_min = curr.iter().map(|(_x, y)| *y).min().unwrap();
        let mut next = FnvHashSet::default();

        for x in x_min - 20..x_max + 20 {
            for y in y_min - 20..y_max + 20 {
                let x8 = get_value(x - 1, y - 1, &curr, 8, dark_mode);
                let x7 = get_value( x - 1, y, &curr, 7, dark_mode);
                let x6 = get_value( x - 1, y + 1, &curr, 6, dark_mode);
                let x5 = get_value( x, y - 1, &curr, 5, dark_mode);
                let x4 = get_value( x, y, &curr, 4, dark_mode);
                let x3 = get_value( x, y + 1, &curr, 3, dark_mode);
                let x2 = get_value( x + 1, y - 1, &curr, 2, dark_mode);
                let x1 = get_value( x + 1, y, &curr, 1, dark_mode);
                let x0 = get_value( x + 1, y + 1, &curr, 0, dark_mode);

                if input.light_lookup[x8 + x7 + x6 + x5 + x4 + x3 + x2 + x1 + x0] == dark_mode {
                    next.insert((x, y));
                }
            }
        }
        
        dark_mode = !dark_mode;
        curr = next;
    }


    curr.len()
}

fn get_value(x: i32, y: i32, set: &FnvHashSet<(i32, i32)>, bit: u8, dark_mode: bool) -> usize {
    if set.contains(&(x, y)) != dark_mode {
        1 << bit
    } else {
        0
    }
}

fn _print_map(map: &FnvHashSet<(i32, i32)>) {
    let x_max = map.iter().map(|(x, _y)| *x).max().unwrap();
    let x_min = map.iter().map(|(x, _y)| *x).min().unwrap();
    let y_max = map.iter().map(|(_x, y)| *y).max().unwrap();
    let y_min = map.iter().map(|(_x, y)| *y).min().unwrap();

    println!("Printing map with dimensions [{x_min},{y_min}]..[{x_max},{y_max}]");

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

#[test]
fn test() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
    #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
    .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
    .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
    .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
    ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
    ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
    \n\
    #..#.\n\
    #....\n\
    ##..#\n\
    ..#..\n\
    ..###";

    assert_eq!(35, part_1(&generator(&input)));
}
