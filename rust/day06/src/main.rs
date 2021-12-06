fn main() {
    let mut fish = [0u64; 9];
    std::fs::read_to_string("../day06.txt").unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| fish[n] += 1);
    
    (0..80).for_each(|_| after_one_day(&mut fish));
    let part_1 = fish.iter().sum::<u64>();
    println!("Part 1: {}", part_1);
    
    (80..256).for_each(|_| after_one_day(&mut fish));
    let part_2 = fish.iter().sum::<u64>();
    println!("Part 2: {}", part_2);
}

fn after_one_day(fish: &mut [u64]) {
    let reproducing_fish = fish[0];
    for i in 0..8 {
        fish[i] = fish[i + 1];
    }
    fish[6] += reproducing_fish;
    fish[8] = reproducing_fish;
}
