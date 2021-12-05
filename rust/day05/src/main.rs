use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../day05.txt").unwrap()
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect::<Vec<_>>();
        
        let part_1 = {
        let mut locations: HashMap::<(u16, u16), usize> = HashMap::new();
        
        for line in input.iter() {
            if line.x_1 == line.x_2 {
                let x = line.x_1;
                let mut ys = [line.y_1, line.y_2];
                ys.as_mut_slice().sort_unstable();
                for y in ys[0]..=ys[1] {
                    locations.entry((x, y)).and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else if line.y_1 == line.y_2 {
                let y = line.y_1;
                let mut xs = [line.x_1, line.x_2];
                xs.as_mut_slice().sort_unstable();
                for x in xs[0]..=xs[1] {
                    locations.entry((x, y)).and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
        
        locations.into_values()
            .filter(|&count| count >= 2)
            .count()
    };
    println!("Part 1: {}", part_1);
    
    let part_2 = {
        let mut locations: HashMap::<(u16, u16), usize> = HashMap::new();
        
        for line in input.iter() {
            let increment_count = |location| { locations.entry(location).and_modify(|count| *count += 1).or_insert(1); };
            
            if line.x_1 == line.x_2 {
                let x = line.x_1;
                let mut ys = [line.y_1, line.y_2];
                ys.as_mut_slice().sort_unstable();
                for y in ys[0]..=ys[1] {
                    locations.entry((x, y)).and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else if line.y_1 == line.y_2 {
                let y = line.y_1;
                let mut xs = [line.x_1, line.x_2];
                xs.as_mut_slice().sort_unstable();
                for x in xs[0]..=xs[1] {
                    locations.entry((x, y)).and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else if line.x_1 < line.x_2 {
                if line.y_1 < line.y_2 {
                    (line.x_1 ..= line.x_2).zip(line.y_1 ..= line.y_2).for_each(increment_count);
                } else {
                    (line.x_1 ..= line.x_2).zip((line.y_2 ..= line.y_1).rev()).for_each(increment_count);
                }
            } else {
                if line.y_1 < line.y_2 {
                    ((line.x_2 ..= line.x_1).rev()).zip(line.y_1 ..= line.y_2).for_each(increment_count);
                } else {
                    ((line.x_2 ..= line.x_1).rev()).zip((line.y_2 ..= line.y_1).rev()).for_each(increment_count);
                }
            }
        }
        
        locations.into_values()
            .filter(|&count| count >= 2)
            .count()
    };
    println!("Part 2: {}", part_2);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Line {
    pub x_1: u16,
    pub y_1: u16,
    pub x_2: u16,
    pub y_2: u16,
}

impl core::str::FromStr for Line {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (point_1, point_2) = s.split_once(" -> ").ok_or("Missing arrow in middle of points")?;
        let (x_1, y_1) = point_1.split_once(',').ok_or("Missing comma in first point")?;
        let (x_2, y_2) = point_2.split_once(',').ok_or("Missing comma in second point")?;
        let x_1 = x_1.parse::<u16>().map_err(|_| "x₁ is not a valid number")?;
        let y_1 = y_1.parse::<u16>().map_err(|_| "y₁ is not a valid number")?;
        let x_2 = x_2.parse::<u16>().map_err(|_| "x₂ is not a valid number")?;
        let y_2 = y_2.parse::<u16>().map_err(|_| "y₂ is not a valid number")?;
        
        Ok(Line { x_1, y_1, x_2, y_2 })
    }
}