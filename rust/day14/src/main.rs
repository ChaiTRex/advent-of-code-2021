use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../day14.txt").unwrap();
    let mut input = input.lines();
    
    let mut polymer = HashMap::<(u8, u8), usize>::new();
    let polymer_template = input.next().unwrap();
    let last_element = polymer_template.as_bytes()[polymer_template.len() - 1];
    polymer_template
        .bytes()
        .tuple_windows()
        .for_each(|pair| {
            *polymer.entry(pair).or_insert(0) += 1;
        });
        
    let _ = input.next();
    let insertion_rules = input.map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();
            let mut start = start.bytes();
            let start = (start.next().unwrap(), start.next().unwrap());
            let end = end.bytes().next().unwrap();
            let end = [(start.0, end), (end, start.1)];
            
            (start, end)
        })
        .collect::<HashMap<_, _>>();
        
    let polymer = (0..10).fold(polymer, |polymer, _| apply_rules(polymer, &insertion_rules));
    let mut counts = polymer.iter()
        .fold(HashMap::new(), |mut counts, (&(element, _), count)| {
            *counts.entry(element).or_insert(0) += count;
            counts
        });
    *counts.entry(last_element).or_insert(0) += 1;
    let mut counts = counts.into_values().collect::<Vec<_>>();
    counts.sort_unstable();
    println!("Part 1: {}", counts.last().unwrap() - counts.first().unwrap());
    
    let polymer = (0..30).fold(polymer, |polymer, _| apply_rules(polymer, &insertion_rules));
    let mut counts = polymer.iter()
        .fold(HashMap::new(), |mut counts, (&(element, _), count)| {
            *counts.entry(element).or_insert(0) += count;
            counts
        });
    *counts.entry(last_element).or_insert(0) += 1;
    let mut counts = counts.into_values().collect::<Vec<_>>();
    counts.sort_unstable();
    println!("Part 2: {}", counts.last().unwrap() - counts.first().unwrap());
}

fn apply_rules(polymer: HashMap<(u8, u8), usize>, insertion_rules: &HashMap<(u8, u8), [(u8, u8); 2]>) -> HashMap<(u8, u8), usize> {
    polymer.into_iter()
        .flat_map(|(xs, count)| {
            let &[ys, zs] = insertion_rules.get(&xs).unwrap();
            [(ys, count), (zs, count)].into_iter()
        })
        .fold(HashMap::new(), |mut polymer, (xs, count)| {
            *polymer.entry(xs).or_insert(0) += count;
            polymer
        })
}