fn main() {
    let mut part_1 = 0;
    let mut part_2_scores = std::fs::read_to_string("../day10.txt")
        .unwrap()
        .lines()
        .map(complete_line)
        .inspect(|result| {
            part_1 += match result {
                Err(')') => 3,
                Err(']') => 57,
                Err('}') => 1197,
                Err('>') => 25137,
                _ => 0,
            }
        })
        .flatten()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                })
                .fold(0u64, |score, value| score * 5 + value)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1);

    part_2_scores.sort_unstable();
    let part_2 = part_2_scores[part_2_scores.len() >> 1];

    println!("Part 2: {}", part_2);
}

fn complete_line(line: &str) -> Result<String, char> {
    let mut stack = Vec::new();

    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if stack.pop() != Some(ch) {
                    return Err(ch);
                }
            }
            _ => panic!("Invalid input character"),
        }
    }

    Ok(stack.iter().rev().collect::<String>())
}
