#![allow(dead_code)]

mod y2017;
//mod y2018;
//mod y2019;

fn main() {
    let input = std::fs::read_to_string("inputs/y2017/day13.txt").unwrap();
    println!("part1: {}", y2017::day13::part1(&input));
    println!("part2: {}", y2017::day13::part2(&input));
}
