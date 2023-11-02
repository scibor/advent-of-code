#![allow(dead_code)]

mod y2018;
mod y2019;

fn main() {
    let input = std::fs::read_to_string("inputs/y2019/day12.txt").unwrap();
    println!("part1: {}", y2019::day12::part1(&input));
    println!("part2: {}", y2019::day12::part2(&input));
}
