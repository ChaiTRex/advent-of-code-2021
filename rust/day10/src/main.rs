fn main() {
    let input = std::fs::read_to_string("../day10.txt").unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    
    let part_1 = input.iter()
        .map(|line| {
            match find_bad_closing(line) {
                Some(')') => 3,
                Some(']') => 57,
                Some('}') => 1197,
                Some('>') => 25137,
                _ => 0,
            }
        })
        .sum::<u32>();
    println!("Part 1: {}", part_1);
    
    let part_2 = {
        let mut scores = input.iter()
            .flat_map(|line| complete_line(line))
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        }
                    })
                    .fold(0u64, |score, value| score * 5 + value)
            })
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() >> 1]
    };
    println!("Part 2: {}", part_2);
}

fn find_bad_closing(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    
    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => if stack.pop() != Some(ch) { return Some(ch) },
            _ => panic!("Invalid input"),
        }
    }
    
    None
}

fn complete_line(line: &str) -> Option<String> {
    let mut stack = Vec::new();
    
    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => if stack.pop() != Some(ch) { return None },
            _ => panic!("Invalid input"),
        }
    }
    
    Some(stack.iter().rev().collect::<String>())
}