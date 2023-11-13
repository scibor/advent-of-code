#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2022;

fn main() {
    let input = std::fs::read_to_string("inputs/y2022/day21.txt").unwrap();
    println!("part1: {}", y2022::day21::part1(&input));
    println!("part2: {}", y2022::day21::part2(&input));
}
