use crate::SeaCucumber::*;

fn main() {
    let mut seafloor = std::fs::read_to_string("../day25.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    match ch {
                        '.' => None,
                        '>' => Some(EastFacing),
                        'v' => Some(SouthFacing),
                        _ => panic!("Invalid input character (not '.', '>', or 'v')"),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    let height = seafloor.len();
    let width = seafloor[0].len();
    
    for step in 1.. {
        let mut moved = false;
        
        let mut move_eastward = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                if seafloor[y][x] == Some(EastFacing) && seafloor[y][(x + 1) % width] == None {
                    move_eastward[y][x] = true;
                    moved = true;
                }
            }
        }
        for y in 0..height {
            for x in 0..width {
                if move_eastward[y][x] {
                    seafloor[y][x] = None;
                    seafloor[y][(x + 1) % width] = Some(EastFacing);
                }
            }
        }
        drop(move_eastward);
        
        let mut move_southward = vec![vec![false; width]; height];
        for y in 0..height {
            let southward_y = (y + 1) % height;
            for x in 0..width {
                if seafloor[y][x] == Some(SouthFacing) && seafloor[southward_y][x] == None {
                    move_southward[y][x] = true;
                    moved = true;
                }
            }
        }
        for y in 0..height {
            let southward_y = (y + 1) % height;
            for x in 0..width {
                if move_southward[y][x] {
                    seafloor[y][x] = None;
                    seafloor[southward_y][x] = Some(SouthFacing);
                }
            }
        }
        
        if !moved {
            println!("Part 1: {}", step);
            break;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum SeaCucumber {
    EastFacing,
    SouthFacing,
}