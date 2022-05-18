use std::fs::File;
use std::io::{BufRead, BufReader};

const LINE_COUNT: usize = 1000;
const BITS_PER_LINE: usize = 12;
const LINE_BITMASK: DoubleLineValue = (1 << BITS_PER_LINE) - 1;

// Enough bits to hold two LineValues multiplied together
type DoubleLineValue = u32;
// Enough bits to hold the highest possible power-of-two LineValue times LINE_COUNT
type LineSum = u32;
// Enough bits to hold a value on a line in the input file
type LineValue = u16;

fn main() {
    let input = {
        let mut input = Vec::with_capacity(LINE_COUNT);

        let file = match File::open("../day03.txt") {
            Ok(file) => file,
            Err(e) => panic!("Error reading file: {:?}", e),
        };
        let mut file = BufReader::new(file);
        // Plus 2 because Windows-created text files have "\r\n" at the end of lines.
        let mut line = String::with_capacity(BITS_PER_LINE + 2);

        loop {
            match file.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => match LineValue::from_str_radix(line.trim_end(), 2) {
                    Ok(n) => input.push(n),
                    Err(e) => panic!("Error reading file: {:?}", e),
                },
                Err(e) => panic!("Error reading file: {:?}", e),
            }
            line.clear()
        }

        input
    };

    let result = (0..BITS_PER_LINE)
        .rev()
        .map(|bit_index| {
            let mask_for_this_bit = 1 << bit_index;

            let one_bit_count = input
                .iter()
                .map(|&n| n as LineSum & mask_for_this_bit)
                .sum::<LineSum>()
                >> bit_index;
            let zero_bit_count = input.len() as LineSum - one_bit_count;

            (one_bit_count > zero_bit_count) as DoubleLineValue
        })
        .fold(0, |result, bit| (result << 1) | bit);

    println!("Part 1: {}", result * (result ^ LINE_BITMASK));

    fn get_rating(
        input: &mut Vec<LineValue>,
        f: fn(&LineSum, &LineSum) -> bool,
    ) -> DoubleLineValue {
        for bit_index in (0..BITS_PER_LINE).rev() {
            let mask_for_this_bit = 1 << bit_index;

            let one_bit_count = input
                .iter()
                .map(|&n| (n & mask_for_this_bit) as LineSum)
                .sum::<LineSum>()
                >> bit_index;
            let zero_bit_count = input.len() as LineSum - one_bit_count;

            let bit_to_keep = (f(&one_bit_count, &zero_bit_count) as LineValue) << bit_index;
            input.retain(|&n| n & mask_for_this_bit == bit_to_keep);

            if input.len() == 1 {
                return input.pop().unwrap() as DoubleLineValue;
            }
        }

        match input.len() {
            0 => panic!("Error in part 2: No values remaining after reducing values"),
            1 => unreachable!(),
            _ => panic!("Error in part 2: More than one value remaining after reducing values"),
        }
    }

    let mut input = input;

    let oxy = get_rating(&mut input.clone(), LineSum::ge);
    let co2 = get_rating(&mut input, LineSum::lt);

    println!("Part 2: {}", oxy * co2);
}
