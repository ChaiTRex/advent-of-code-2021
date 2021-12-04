use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("../day04.txt").unwrap();
    let mut input = input.lines()
        .map(|line| line.to_owned());
    
    let called_numbers = input.next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
        
    let boards = &input.chunks(6)
        .into_iter()
        .map(|chunk| Board::new(chunk))
        .collect::<Vec<_>>();
    
    let number_finder = {
        let mut number_finder: HashMap::<u8, Vec<(usize, usize, usize)>> = HashMap::new();
        for (i, board) in (0..).zip(boards.iter()) {
            for (j, row) in (0..).zip(board.numbers.iter()) {
                for (k, &number) in (0..).zip(row.iter()) {
                    number_finder.entry(number)
                        .and_modify(|vec| vec.push((i, j, k)))
                        .or_insert(vec![(i, j, k)]);
                }
            }
        }
        number_finder
    };
    
    let part_1 = {
        let mut part_1 = None;
        
        let mut boards = boards.clone();
        'part_1_main_loop: for &called_number in called_numbers.iter() {
            match number_finder.get(&called_number) {
                Some(locations) => {
                    for &(i, j, k) in locations {
                        let board = boards.get_mut(i).unwrap();
                        board.marked[j][k] = true;
                        if board.is_winner() {
                            part_1 = Some(called_number as u32 * board.numbers.iter()
                                .flat_map(|num| num.iter())
                                .zip(board.marked.iter().flat_map(|mar| mar.iter()))
                                .filter(|(_, mar)| !*mar)
                                .map(|(num, _)| *num as u32)
                                .sum::<u32>());
                            break 'part_1_main_loop;
                        }
                    }
                }
                _ => (),
            }
        }
        
        part_1
    };
    println!("Part 1: {:?}", part_1);
    
    let part_2 = {
        let mut part_2 = None;
        
        let mut boards = boards.clone();
        let mut remaining_boards = boards.iter()
            .map(|board| board.numbers.clone())
            .collect::<HashSet<Vec<Vec<u8>>>>();
        'part_2_main_loop: for &called_number in called_numbers.iter() {
            match number_finder.get(&called_number) {
                Some(locations) => {
                    for &(i, j, k) in locations {
                        let board = boards.get_mut(i).unwrap();
                        board.marked[j][k] = true;
                        if board.is_winner() {
                            if remaining_boards.remove(&board.numbers) && remaining_boards.len() == 0 {
                                part_2 = Some(called_number as u32 * board.numbers.iter()
                                    .flat_map(|num| num.iter())
                                    .zip(board.marked.iter().flat_map(|mar| mar.iter()))
                                    .filter(|(_, mar)| !*mar)
                                    .map(|(num, _)| *num as u32)
                                    .sum::<u32>());
                                break 'part_2_main_loop;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        
        part_2
    };
    println!("Part 2: {:?}", part_2);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
    pub numbers: Vec<Vec<u8>>,
    pub marked: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(mut chunk: impl Iterator<Item = String>) -> Board {
        let _ = chunk.next();
        let numbers = chunk.map(|line| line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        Board {
            numbers,
            marked: vec![vec![false; 5]; 5],
        }
    }
    
    pub fn is_winner(&self) -> bool {
        self.marked.iter().any(|row| row.iter().all(|&x| x)) ||
        (0..5).any(|column| self.marked.iter().all(|row| row[column]))
    }
}