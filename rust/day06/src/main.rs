fn main() {
    let mut fish = [0u64; 9];
    std::fs::read_to_string("../day06.txt")
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| fish[n] += 1);

    let mut day_cycle = (7..=8).chain(0..7).zip(0..=8).cycle();

    let part_1 = {
        for (i, j) in day_cycle.by_ref().take(80) {
            fish[i] += fish[j];
        }
        fish.iter().sum::<u64>()
    };
    println!("Part 1: {}", part_1);

    let part_2 = {
        for (i, j) in day_cycle.take(256 - 80) {
            fish[i] += fish[j];
        }
        fish.iter().sum::<u64>()
    };
    println!("Part 2: {}", part_2);
}
