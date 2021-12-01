fn main() {
    let input = std::fs::read_to_string("../day01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let part_1 = input.windows(2).filter(|xs| xs[0] < xs[1]).count();
    println!("Part 1: {}", part_1);

    let part_2 = input.windows(4).filter(|xs| xs[0] < xs[3]).count();
    println!("Part 2: {}", part_2);
}
