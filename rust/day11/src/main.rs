fn main() {
    let mut octopuses = [[0; 10]; 10];
    std::fs::read_to_string("../day11.txt").unwrap()
        .lines()
        .zip(0..)
        .for_each(|(line, y)| {
            line.bytes()
                .zip(0..)
                .for_each(|(ch, x)| {
                    octopuses[y][x] = ch - b'0';
                });
        });
    
    let mut flash_count = 0;
    let mut time_step = 0;
    
    let mut part_2_complete = false;
    
    while time_step < 100 || !part_2_complete {
        time_step += 1;
        let mut already_flashed = [[false; 10]; 10];
        
        // Initial increase by 1
        for y in 0..10 {
            for x in 0..10 {
                octopuses[y][x] += 1;
            }
        }
        
        let mut new_flashes_happened = true;
        while new_flashes_happened {
            new_flashes_happened = false;
            
            for y in 0..10 {
                for x in 0..10 {
                    if octopuses[y][x] > 9 && !already_flashed[y][x] {
                        new_flashes_happened = true;
                        flash_count += 1;
                        already_flashed[y][x] = true;
                        
                        if y != 0 {
                            if x != 0 {
                                octopuses[y - 1][x - 1] += 1;
                            }
                            octopuses[y - 1][x] += 1;
                            if x != 9 {
                                octopuses[y - 1][x + 1] += 1;
                            }
                        }
                        if x != 0 {
                            octopuses[y][x - 1] += 1;
                        }
                        if x != 9 {
                            octopuses[y][x + 1] += 1;
                        }
                        if y != 9 {
                            if x != 0 {
                                octopuses[y + 1][x - 1] += 1;
                            }
                            octopuses[y + 1][x] += 1;
                            if x != 9 {
                                octopuses[y + 1][x + 1] += 1;
                            }
                        }
                    }
                }
            }
        }
            // Set flashed octopuses to energy level zero
        for y in 0..10 {
            for x in 0..10 {
                if already_flashed[y][x] {
                    octopuses[y][x] = 0;
                }
            }
        }
        if time_step == 100 {
            println!("Part 1: {}", flash_count);
        }
        
        if already_flashed.iter().all(|row| row.iter().all(|&value| value)) {
            println!("Part 2: {}", time_step);
            part_2_complete = true;
        }
    }
}