#![allow(dead_code)]

mod y2018;

fn main() {
    // let input = std::fs::read_to_string("inputs/test.txt").unwrap();
    let input = std::fs::read_to_string("inputs/y2018/day7.txt").unwrap();
    println!("{}", y2018::day7::part2(&input));
}
