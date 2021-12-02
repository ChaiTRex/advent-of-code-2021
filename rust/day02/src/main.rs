use crate::Move::*;

fn main() {
    let input = std::fs::read_to_string("../day02.txt").unwrap()
        .lines()
        .map(|x| x.parse::<Move>().unwrap())
        .collect::<Vec<_>>();
    
    let (x, y) = input.iter()
        .map(|&n| {
            match n {
                Forward(n) => (n as i64, 0),
                Down(n) => (0, n as i64),
                Up(n) => (0, -(n as i64)),
            }
        })
        .fold((0, 0), |(x1, y1), (x2, y2)| {
            (x1 + x2, y1 + y2)
        });
    
    println!("Part 1: {}", x * y);

    let (x, y, _) = input.iter()
        .fold((0, 0, 0), |(horiz, depth, aim), &n| {
            match n {
                Forward(n) => (horiz + n as i64, depth + aim * n as i64, aim),
                Down(n) => (horiz, depth, aim + n as i64),
                Up(n) => (horiz, depth, aim - n as i64),
            }
        });
    
    println!("Part 2: {}", x * y);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl core::str::FromStr for Move {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("forward ") {
            s[8 ..].parse::<u32>()
                .map(Forward)
                .map_err(|err| format!("{:?}", err))
        } else if s.starts_with("down ") {
            s[5 ..].parse::<u32>()
                .map(Down)
                .map_err(|err| format!("{:?}", err))
        } else if s.starts_with("up ") {
            s[3 ..].parse::<u32>()
                .map(Up)
                .map_err(|err| format!("{:?}", err))
        } else {
            Err("Doesn't start with \"forward \", \"down \", or \"up \"".to_owned())
        }
    }
}