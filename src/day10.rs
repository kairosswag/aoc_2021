pub fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(lines: &[&str]) -> u32 {
    let mut tally = 0;
    'lines: for &line in lines {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                opening @ '{' | opening @ '[' | opening @ '(' | opening @ '<' => stack.push(opening),

                closing @ '}' | closing @ ']' | closing @ ')' | closing @ '>' => {
                    if let Some(val) = stack.pop() {
                        if closing_for(&val) != closing {
                            // corrupted found
                            tally += points_awarded_for(&closing);
                            // println!("{}: Expected {} but found {} instead", line, val, closing);
                            continue 'lines;
                        } 
                    } else {
                        panic!("no item found anymore");
                    }
                }

                other => panic!("unexpected char {}!", other),
            }
        }
    }
    tally
}


pub fn part_2(lines: &[&str]) -> u64 {
    // let mut tally = 0;
    let mut scores = Vec::new();
    'lines: for &line in lines {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                opening @ '{' | opening @ '[' | opening @ '(' | opening @ '<' => stack.push(opening),

                closing @ '}' | closing @ ']' | closing @ ')' | closing @ '>' => {
                    if let Some(val) = stack.pop() {
                        if closing_for(&val) != closing {
                            // corrupted found
                            // tally += points_awarded_for(&closing);
                            // println!("{}: Expected {} but found {} instead", line, val, closing);
                            continue 'lines;
                        } 
                    } else {
                        panic!("no item found anymore");
                    }
                }

                other => panic!("unexpected char {}!", other),
            }
        }
        let mut score: u64 = 0;
        while let Some(opening) = stack.pop() {
            score *= 5;
            score += points_awarded_for_complete(&opening) as u64;
        }
        scores.push(score);
    }
    scores.sort();
    scores[(scores.len()) / 2]
}

fn points_awarded_for_complete(opening: &char) -> u32 {
    match opening {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        err => panic!("unknown bracket {}", err),
    }
}

fn points_awarded_for(closing: &char) -> u32 {
    match closing {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("wtf?"),
    }
}

fn closing_for(opening: &char) -> char {
    match *opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        err => panic!("unknown bracket {}", err),
    }
}

#[test]
pub fn test() {
    let input = "[({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(26397, part_1(&generator(&input)));
    assert_eq!(288957, part_2(&generator(&input)));
}
