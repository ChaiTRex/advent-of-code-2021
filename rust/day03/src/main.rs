fn main() {
    let input = std::fs::read_to_string("../day03.txt").unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    println!("Problem input: {:?}", input);
    
    let mut line_count = 0;
    let mut one_bit_counts = vec![0; 12];
    
    for line in input.iter() {
        line_count += 1;
        for (i, bit) in (0usize .. 12).zip(line.chars()) {
            one_bit_counts[i] += if bit == '1' { 1 } else { 0 };
        }
    }
    
    println!("{:?}", one_bit_counts);
    
    let mut result = 0u32;
    
    for one_bit_count in one_bit_counts {
        result <<= 1;
        result |= if one_bit_count > line_count - one_bit_count { 1 } else { 0 };
    }
    println!("{}", result);
    println!("Part 1: {}", result * (!result & 0b111111111111));
    
    let mut input_oxy = input.clone();
    for bit_position in 0 .. 12 {
        let mut line_count = 0;
        let mut one_bit_count = 0;
        
        for line in input_oxy.iter() {
            line_count += 1;
            one_bit_count += if line.as_bytes()[bit_position] == '1' as u8 { 1 } else { 0 };
        }
        
        let to_keep = if one_bit_count >= line_count - one_bit_count { '1' } else { '0' };
        input_oxy.retain(|line| line.as_bytes()[bit_position] == to_keep as u8);
        if input_oxy.len() == 1 { break; }
    }
    let mut oxy = 0;
    let input_oxy = input_oxy.pop().unwrap();
    for ch in input_oxy.chars() {
        oxy <<= 1;
        oxy |= if ch == '1' { 1 } else { 0 };
    }
    println!("{}", oxy);
    
    let mut input_co2 = input.clone();
    for bit_position in 0 .. 12 {
        let mut line_count = 0;
        let mut one_bit_count = 0;
        
        for line in input_co2.iter() {
            line_count += 1;
            one_bit_count += if line.as_bytes()[bit_position] == '1' as u8 { 1 } else { 0 };
        }
        
        let to_keep = if one_bit_count >= line_count - one_bit_count { '0' } else { '1' };
        input_co2.retain(|line| line.as_bytes()[bit_position] == to_keep as u8);
        if input_co2.len() == 1 { break; }
    }
    let mut co2 = 0;
    let input_co2 = input_co2.pop().unwrap();
    for ch in input_co2.chars() {
        co2 <<= 1;
        co2 |= if ch == '1' { 1 } else { 0 };
    }
    println!("{}", co2);
    
    println!("Part 2: {}", oxy * co2);
}
