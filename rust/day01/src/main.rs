use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(File::open("../day01.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .tuple_windows();

    let (a, b, c, d) = input.next().unwrap();
    let mut part_1 = if a < b { 1 } else { 0 };
    part_1 += if b < c { 1 } else { 0 };
    part_1 += if c < d { 1 } else { 0 };

    let mut part_2 = if a < d { 1 } else { 0 };

    for (a, _, c, d) in input {
        part_1 += if c < d { 1 } else { 0 };
        part_2 += if a < d { 1 } else { 0 };
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
