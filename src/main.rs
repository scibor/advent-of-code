use aoclibrary::*;

fn main() {
    let input = std::fs::read_to_string("inputs/y2023/day19.txt").unwrap();
    println!("part1: {}", y2023::day19::part1(&input));
    println!("part2: {}", y2023::day19::part2(&input));
}
