use std::collections::HashMap;

use crate::Cave::*;

fn main() {
    let cave_system = {
        let mut cave_system: HashMap<Cave, Vec<Cave>> = HashMap::new();
        std::fs::read_to_string("../day12.txt").unwrap()
            .lines()
            .for_each(|line| {
                let (cave_1, cave_2) = line.split_once("-").unwrap();
                let cave_1 = cave_1.parse::<Cave>().unwrap();
                let cave_2 = cave_2.parse::<Cave>().unwrap();
                
                (*cave_system.entry(cave_1.clone()).or_insert_with(|| Vec::new()))
                    .push(cave_2.clone());
                
                (*cave_system.entry(cave_2).or_insert_with(|| Vec::new()))
                    .push(cave_1);
            });
        cave_system
    };
    
    println!("Part 1: {}", get_single_paths(&cave_system).len());
    println!("Part 2: {}", get_double_paths(&cave_system).len());
}

fn get_single_paths(cave_system: &HashMap<Cave, Vec<Cave>>) -> Vec<Vec<Cave>> {
    fn dfs(cave_system: &HashMap<Cave, Vec<Cave>>, this_path: Vec<Cave>, paths: &mut Vec<Vec<Cave>>) {
        cave_system.get(this_path.last().unwrap())
            .unwrap()
            .iter()
            .filter(|connected_cave| connected_cave.is_large() || !this_path.contains(&connected_cave))
            .for_each(|connected_cave| {
                let mut this_path = this_path.clone();
                this_path.push(connected_cave.clone());
                if connected_cave.name() == "end" {
                    paths.push(this_path);
                } else {
                    dfs(&cave_system, this_path, paths);
                }
            });
    }
    
    let mut paths = Vec::new();
    dfs(&cave_system, vec!["start".parse::<Cave>().unwrap()], &mut paths);
    paths
}

fn get_double_paths(cave_system: &HashMap<Cave, Vec<Cave>>) -> Vec<Vec<Cave>> {
    fn dfs(cave_system: &HashMap<Cave, Vec<Cave>>, this_path: Vec<Cave>, mut visited_twice: Option<Cave>, paths: &mut Vec<Vec<Cave>>) {
        for connected_cave in cave_system.get(this_path.last().unwrap()).unwrap().iter() {
            if connected_cave.name() == "start" {
                continue;
            }
            let visited_twice = if connected_cave.is_small() && connected_cave.name() != "end" {
                if this_path.contains(connected_cave) {
                    if visited_twice.is_some() {
                        continue;
                    }
                    Some(connected_cave.clone())
                } else {
                    visited_twice.clone()
                }
            } else {
                visited_twice.clone()
            };
            let mut this_path = this_path.clone();
            this_path.push(connected_cave.clone());
            if connected_cave.name() == "end" {
                paths.push(this_path);
            } else {
                dfs(&cave_system, this_path, visited_twice, paths);
            }
        }
    }
    
    let mut paths = Vec::new();
    dfs(&cave_system, vec!["start".parse::<Cave>().unwrap()], None, &mut paths);
    paths
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Cave {
    SmallCave(String),
    LargeCave(String),
}

impl Cave {
    pub fn name(&self) -> &str {
        match self {
            SmallCave(name) => &name,
            LargeCave(name) => &name,
        }
    }
    
    pub fn is_small(&self) -> bool {
        matches!(self, SmallCave(_))
    }
    
    pub fn is_large(&self) -> bool {
        matches!(self, LargeCave(_))
    }
}

impl core::str::FromStr for Cave {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|ch| ch.is_ascii_lowercase()) {
            Ok(SmallCave(s.to_owned()))
        } else if s.chars().all(|ch| ch.is_ascii_uppercase()) {
            Ok(LargeCave(s.to_owned()))
        } else {
            println!("Error: {}", s);
            Err("Not a valid cave name (all capital or all lowercase letters)")
        }
    }
}