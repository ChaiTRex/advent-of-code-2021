use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Move::*;

fn main() {
    let (x, y1, y2) = BufReader::new(File::open("../day02.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<Move>().unwrap())
        .fold((0, 0, 0), |(x, y1, y2), n| match n {
            Forward(n) => (x + n as i64, y1, y2 + y1 * n as i64),
            Down(n) => (x, y1 + n as i64, y2),
            Up(n) => (x, y1 - n as i64, y2),
        });

    println!("Part 1: {}", x * y1);
    println!("Part 2: {}", x * y2);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl core::str::FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, amount) = s
            .split_once(' ')
            .ok_or("Missing a space between command and amount")?;
        let amount = amount
            .parse::<u32>()
            .map_err(|_| "Amount is an invalid number")?;
        Ok(match command {
            "forward" => Forward(amount),
            "down" => Down(amount),
            "up" => Up(amount),
            _ => return Err("Invalid command"),
        })
    }
}
