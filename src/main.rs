#![allow(dead_code)]

//mod y2017;
//mod y2018;
//mod y2019;
mod y2020;

fn main() {
    let input = std::fs::read_to_string("inputs/y2020/day22.txt").unwrap();
    println!("part1: {}", y2020::day22::part1(&input));
    println!("part2: {}", y2020::day22::part2(&input));
}
