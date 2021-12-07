use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY_80_AND_256_WEIGHTS: [(u64, u64); 5] = [
    (1401, 6206821033),
    (1191, 5617089148),
    (1154, 5217223242),
    (1034, 4726100874),
    (950, 4368232009),
];

fn main() {
    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut input = BufReader::new(File::open("../day06.txt").unwrap());
    let mut read_buffer = Vec::with_capacity(2);
    loop {
        match input.read_until(b',', &mut read_buffer) {
            Ok(0) => break,
            Ok(1 | 2) => {
                let (part_1_weight, part_2_weight) =
                    DAY_80_AND_256_WEIGHTS[(read_buffer[0] - b'1') as usize];
                part_1 += part_1_weight;
                part_2 += part_2_weight;
                read_buffer.clear();
            }
            _ => panic!("Invalid input"),
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
