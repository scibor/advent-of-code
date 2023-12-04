#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod y2023;

fn main() {
    let input = std::fs::read_to_string("inputs/y2023/day4.txt").unwrap();
    println!("part1: {}", y2023::day4::part1(&input));
    println!("part2: {}", y2023::day4::part2(&input));
}
