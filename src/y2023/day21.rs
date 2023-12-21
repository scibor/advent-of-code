use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
}

fn parse_data(input: &str) -> (Position, HashSet<Position>) {
    let mut rocks = HashSet::new();
    let mut start = Position::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                let _ = rocks.insert(Position::new(x.try_into().unwrap(), y.try_into().unwrap()));
            } else if c == 'S' {
                start = Position::new(x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }
    (start, rocks)
}

fn walk_n_steps(start: &Position, rocks: &HashSet<Position>, n: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, start.clone()));
    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut solution = HashSet::new();
    solution.insert(start.clone());

    while !queue.is_empty() {
        let (step, node) = queue.pop_front().unwrap();

        let right = Position::new(node.x + 1, node.y);
        let left = Position::new(node.x - 1, node.y);
        let up = Position::new(node.x, node.y - 1);
        let down = Position::new(node.x, node.y + 1);

        if !rocks.contains(&right) && !visited.contains(&right) && step <= n {
            queue.push_back((step + 1, right.clone()));
            visited.insert(right.clone());
            if step % 2 == 1 {
                solution.insert(right.clone());
            }
        }

        if !rocks.contains(&left) && !visited.contains(&left) && step <= n {
            queue.push_back((step + 1, left.clone()));
            visited.insert(left.clone());
            if step % 2 == 1 {
                solution.insert(left.clone());
            }
        }

        if !rocks.contains(&up) && !visited.contains(&up) && step <= n {
            queue.push_back((step + 1, up.clone()));
            visited.insert(up.clone());
            if step % 2 == 1 {
                solution.insert(up.clone());
            }
        }

        if !rocks.contains(&down) && !visited.contains(&down) && step <= n {
            queue.push_back((step + 1, down.clone()));
            visited.insert(down.clone());
            if step % 2 == 1 {
                solution.insert(down.clone());
            }
        }
    }

    solution.len()
}

/// # Panics
#[must_use]
pub fn part1(input: &str) -> String {
    let (start, rocks) = parse_data(input);
    let result = walk_n_steps(&start, &rocks, 64);
    format!("{result}")
}

#[must_use]
pub fn part2(_input: &str) -> String {
    let result = 0;
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";

    #[test]
    fn test_case_part1() {
        let (start, rocks) = parse_data(TEST_DATA);
        assert_eq!(16, walk_n_steps(&start, &rocks, 6));
    }

    #[test]
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("1", part1(TEST_DATA));
    }
}
