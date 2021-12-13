use std::collections::HashSet;

use crate::PaperFoldInstruction::*;

fn main() {
    let input = std::fs::read_to_string("../day13.txt").unwrap();
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots = dots.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
        })
        .collect::<HashSet<_>>();
    
    let folds = folds.lines()
        .map(|line| line.parse::<PaperFoldInstruction>().unwrap())
        .collect::<Vec<_>>();
    
    let dots = folds[0].apply(dots);
    println!("Part 1: {}", dots.len());
    
    let dots = folds[1..].into_iter()
        .fold(dots, |dots, paper_fold| paper_fold.apply(dots));
    println!("Part 2:\n{}", display_dots(&dots));
}

pub fn display_dots(dots: &HashSet<(u16, u16)>) -> String {
    let mut char_matrix = {
        let mut max_x = 0;
        let mut max_y = 0;
        
        for &(x, y) in dots.iter() {
            max_x = core::cmp::max(max_x, x);
            max_y = core::cmp::max(max_y, y);
        }
        
        vec![{
            let mut row = vec![' '; max_x as usize + 2];
            row[max_x as usize + 1] = '\n';
            row
        }; max_y as usize + 1]
    };
    
    for &(x, y) in dots.iter() {
        char_matrix[y as usize][x as usize] = '█';
    }
    
    char_matrix.into_iter()
        .flat_map(|row| row.into_iter())
        .collect::<String>()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaperFoldInstruction {
    FoldAlongX(u16),
    FoldAlongY(u16),
}

impl PaperFoldInstruction {
    pub fn apply(&self, dots: HashSet<(u16, u16)>) -> HashSet<(u16, u16)> {
        match self {
            FoldAlongX(fold_line) => {
               dots.into_iter()
                  .filter(|(x, _)| x != fold_line)
                  .map(|(x, y)| {
                      if x < *fold_line {
                          (x, y)
                      } else {
                          (fold_line + fold_line - x, y)
                      }
                  })
                  .collect::<HashSet<_>>()
            },
            FoldAlongY(fold_line) => {
               dots.into_iter()
                  .filter(|(_, y)| y != fold_line)
                  .map(|(x, y)| {
                      if y < *fold_line {
                          (x, y)
                      } else {
                          (x, fold_line + fold_line - y)
                      }
                  })
                  .collect::<HashSet<_>>()
            },
        }
    }
}

impl core::str::FromStr for PaperFoldInstruction {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("fold along ") {
            return Err("Paper fold instruction does not start with \"fold along \"");
        }
        let s = &s[11..];
        
        if s.starts_with("x=") {
            s[2..].parse::<u16>()
                .map(FoldAlongX)
                .map_err(|_| "Paper fold instruction does not contain a valid number")
        } else if s.starts_with("y=") {
            s[2..].parse::<u16>()
                .map(FoldAlongY)
                .map_err(|_| "Paper fold instruction does not contain a valid number")
        } else {
            Err("Paper fold instruction does not specify \"x=\" or \"y=\"")
        }
    }
}