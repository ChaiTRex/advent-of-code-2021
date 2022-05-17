use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = {
        let mut input = Vec::with_capacity(1000);

        let file = match File::open("../day03.txt") {
            Ok(file) => file,
            Err(e) => panic!("Error reading file: {:?}", e),
        };
        let mut file = BufReader::new(file);
        let mut line = String::with_capacity(14);

        loop {
            line.clear();
            match file.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => match u16::from_str_radix(line.trim_end(), 2) {
                    Ok(n) => input.push(n),
                    Err(e) => panic!("Error reading file: {:?}", e),
                },
                Err(e) => panic!("Error reading file: {:?}", e),
            }
        }

        input
    };

    let result = {
        let mut one_bit_counts = [0; 12];
        for n in input.iter() {
            for (i, count) in (0..12).rev().zip(one_bit_counts.iter_mut()) {
                *count += ((n >> i) & 1) as usize;
            }
        }

        one_bit_counts.into_iter().fold(0, |result, n| {
            (result << 1) | ((n << 1) > input.len()) as u32
        })
    };

    println!("Part 1: {}", result * (result ^ 0b1111_1111_1111));

    fn get_rating(input: &mut Vec<u16>, f: fn(&usize, &usize) -> bool) -> u32 {
        for i in (0..12).rev() {
            let one_bit_count = input.iter().map(|n| ((n >> i) & 1) as usize).sum::<usize>();
            let to_keep = f(&(one_bit_count << 1), &input.len()) as u16;

            input.retain(|n| (n >> i) & 1 == to_keep);
            if input.len() == 1 {
                break;
            }
        }

        input.pop().unwrap() as u32
    }

    let mut input = input;

    let oxy = get_rating(&mut input.clone(), usize::ge);
    let co2 = get_rating(&mut input, usize::lt);

    println!("Part 2: {}", oxy * co2);
}
