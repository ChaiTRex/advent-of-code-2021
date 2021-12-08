fn main() {
    let input = std::fs::read_to_string("../day08.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Display>().unwrap())
        .collect::<Vec<_>>();
    
    let part_1 = input.iter()
        .map(|display| {
            display.0
                .iter()
                .filter(|&&x| x == 1 || x == 4 || x == 7 || x == 8)
                .count()
        })
        .sum::<usize>();
    println!("Part 1: {}", part_1);
    
    let part_2 = input.iter()
        .map(|display| {
            let digits = &display.0;
            digits[0] as u32*1000 + digits[1] as u32*100 + digits[2] as u32*10 + digits[3] as u32
        })
        .sum::<u32>();
    println!("Part 2: {}", part_2);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Display([u8; 4]);

impl core::str::FromStr for Display {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn rewiring_to_bits(rewiring_code: &str) -> Result<u8, &'static str> {
            let mut result = 0;

            for ch in rewiring_code.bytes() {
                if (b'a'..=b'g').contains(&ch) {
                    result |= 1 << (ch - b'a');
                } else {
                    return Err(
                        "Wiring code has something other than a lowercase letter from a through g",
                    );
                }
            }

            Ok(result)
        }

        let (rewiring_codes, displayed_codes) = s
            .split_once(" | ")
            .ok_or("Missing a \" | \" between rewirings and displayed digits")?;
        
        let bits_to_digits: [u8; 128] = {
            let mut rewirings_bits: [u8; 10] = [0; 10];
            let mut rewired_segment_counts: [u8; 7] = [0; 7];
            
            let mut i = 0;
            for rewiring_code in rewiring_codes.split(' ') {
                if i == 10 {
                    return Err("Found more than all ten digits' rewiring codes");
                }
                
                let mut rewiring_bits = rewiring_to_bits(rewiring_code)?;
                rewirings_bits[i] = rewiring_bits;
                
                for i in 0..7 {
                    rewired_segment_counts[i] += rewiring_bits & 1;
                    rewiring_bits >>= 1;
                }
                
                i += 1;
            }
            if i != 10 {
                return Err("Found less than all ten digits' rewiring codes");
            }
            
            rewirings_bits.sort_unstable_by_key(|rewiring_bits| rewiring_bits.count_ones());
            
            let mut bits_to_digits = [u8::MAX; 128];
            let ones_bits = rewirings_bits[0];
            bits_to_digits[ones_bits as usize] = 1;
            bits_to_digits[rewirings_bits[1] as usize] = 7;
            bits_to_digits[rewirings_bits[2] as usize] = 4;
            bits_to_digits[rewirings_bits[9] as usize] = 8;
            
            let mut nines_bits = None;
            for i in 6..9 {
                let rewiring_bits = rewirings_bits[i];
                bits_to_digits[rewiring_bits as usize] = match rewired_segment_counts[(rewiring_bits ^ 0b1111111).trailing_zeros() as usize] {
                    4 => {
                        nines_bits = Some(rewiring_bits);
                        9
                    },
                    7 => 0,
                    8 => 6,
                    _ => return Err("Invalid rewiring code"),
                }
            }
            let nines_bits = nines_bits.ok_or("Missing rewiring code for digit 9")?;
            
            for i in 3..6 {
                let rewiring_bits = rewirings_bits[i];
                bits_to_digits[rewiring_bits as usize] = if rewiring_bits & ones_bits == ones_bits {
                    3
                } else if rewiring_bits & nines_bits == rewiring_bits {
                    5
                } else {
                    2
                }
            }
            
            bits_to_digits
        };
        
        Ok(Display({
            let mut i = 0;
            
            let mut digits: [u8; 4] = [u8::MAX - 1; 4];
            
            for displayed_code in displayed_codes.split(' ') {
                if i == 4 {
                    return Err("Found more than four digits on the display");
                }
                
                digits[i] = bits_to_digits[rewiring_to_bits(displayed_code)? as usize];
                
                i += 1;
            }
            if i != 4 {
                return Err("Found less than four digits on the display")
            }
            
            digits
        }))
    }
}