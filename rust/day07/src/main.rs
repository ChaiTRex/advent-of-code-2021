use itertools::FoldWhile::{Continue,Done};
use itertools::Itertools;

fn main() {
    let mut input = std::fs::read_to_string("../day07.txt")
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    input.sort_unstable();

    let len = input.len();
    let median_index = len / 2;
    let median = input[median_index];

    let part_1 = {
        let lower_half = median_index * median - input[..median_index].iter().sum::<usize>();
        let upper_half =
            input[median_index + 1..].iter().sum::<usize>() - (len - median_index - 1) * median;
        lower_half + upper_half
    };
    println!("Part 1: {}", part_1);
    
    let part_2 = {
        let original_cost = cost_of(&input, median);
        
        let decreasing_target_cost = (input[0]..median).rev()
            .map(|target| cost_of(&input, target))
            .fold_while(original_cost, |old, new| {
                if old < new {
                    Done(old)
                } else {
                    Continue(new)
                }
            })
            .into_inner();
        
        let increasing_target_cost = (median + 1..=*input.last().unwrap())
            .map(|target| cost_of(&input, target))
            .fold_while(original_cost, |old, new| {
                if old < new {
                    Done(old)
                } else {
                    Continue(new)
                }
            })
            .into_inner();
            
        std::cmp::min(decreasing_target_cost, increasing_target_cost)
    };
    println!("Part 2: {}", part_2);
}

fn cost_of(input: &[usize], target: usize) -> usize {
    input.iter()
        .map(|&n| {
            let distance = (n as isize - target as isize).abs() as usize;
            distance * (distance + 1) / 2
        })
        .sum::<usize>()
}