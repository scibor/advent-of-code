#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod y2017;
mod y2018;
mod y2019;
mod y2020;

fn main() {
    let input = std::fs::read_to_string("inputs/y2020/day21.txt").unwrap();
    let answer = y2020::day21::part1_and_2(&input);
    println!("part1: {}", answer.0);
    println!("part2: {}", answer.1);
    //    println!("part2: {}", y2020::day21::part2(&input));
}
