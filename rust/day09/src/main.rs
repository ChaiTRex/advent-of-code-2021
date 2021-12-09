use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("../day09.txt").unwrap()
        .lines()
        .map(|line| line.bytes().map(|ch| ch - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut low_points = Vec::new();
    
    let part_1 = {
        let mut low_point_height_sum = 0;
        
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                let height = input[y][x];
                let lowest_neighbor = [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter()
                    .flat_map(|(dy, dx)| read_input(&input, y as isize + dy, x as isize + dx))
                    .min()
                    .unwrap_or(0);
                if height < lowest_neighbor {
                    low_points.push((y, x));
                    low_point_height_sum += height as usize;
                }
            }
        }
        
        low_point_height_sum + low_points.len()
    };
    println!("Part 1: {}", part_1);
    
    let part_2 = {
        let mut basin_sizes = low_points.into_iter()
            .map(|position| basin_size(&input, position))
            .collect::<Vec<_>>();
        basin_sizes.sort_unstable();
        basin_sizes[basin_sizes.len() - 3..].iter().product::<usize>()
    };
    println!("Part 2: {}", part_2);
}

fn read_input(input: &Vec<Vec<u8>>, y: isize, x: isize) -> Option<u8> {
    if y < 0 || x < 0 {
        return None;
    }
    
    input.get(y as usize).and_then(|row| row.get(x as usize)).copied()
}

fn basin_size(input: &Vec<Vec<u8>>, position: (usize, usize)) -> usize {
    let mut result = 1;
    
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    visited.insert(position);
    let mut position = Some(position);
    while let Some((y, x)) = position {
        let height = input[y][x];
        
        if y != 0 && !visited.contains(&(y - 1, x)) && (height + 1..9).contains(&input[y - 1][x]) {
            // Go up
            stack.push((y, x));
            visited.insert((y - 1, x));
            position = Some((y - 1, x));
            
            result += 1;
        } else if x != 0 && !visited.contains(&(y, x - 1)) && (height + 1..9).contains(&input[y][x - 1]) {
            // Go left
            stack.push((y, x));
            visited.insert((y, x - 1));
            position = Some((y, x - 1));
            
            result += 1;
        } else if x != input[y].len() - 1 && !visited.contains(&(y, x + 1)) && (height + 1..9).contains(&input[y][x + 1]) {
            // Go right
            stack.push((y, x));
            visited.insert((y, x + 1));
            position = Some((y, x + 1));
            
            result += 1;
        } else if y != input.len() - 1 && !visited.contains(&(y + 1, x)) && (height + 1..9).contains(&input[y + 1][x]) {
            // Go down
            stack.push((y, x));
            visited.insert((y + 1, x));
            position = Some((y + 1, x));
            
            result += 1;
        } else {
            position = stack.pop();
        }
    }
    
    result
}