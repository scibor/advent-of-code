#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use std::collections::{BinaryHeap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let size = input.lines().count();
    let mut result = Vec::with_capacity(size);
    for _i in 0..size {
        result.push(Vec::with_capacity(size));
    }
    for (i, line) in input.lines().enumerate() {
        for c in line.trim().chars() {
            result
                .get_mut(i)
                .unwrap()
                .push(format!("{c}").parse::<usize>().unwrap());
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct VertexInDijkstra {
    heat_loss: usize,
    row: isize,
    column: isize,
    row_delta: isize,
    column_delta: isize,
    steps: isize,
}

impl VertexInDijkstra {
    fn new(
        heat_loss: usize,
        row: isize,
        column: isize,
        row_delta: isize,
        column_delta: isize,
        steps: isize,
    ) -> Self {
        VertexInDijkstra {
            heat_loss,
            row,
            column,
            row_delta,
            column_delta,
            steps,
        }
    }

    fn to_visited_vertex(&self) -> VisitedVertex {
        VisitedVertex {
            row: self.row,
            column: self.column,
            row_delta: self.row_delta,
            column_delta: self.column_delta,
            steps: self.steps,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct VisitedVertex {
    row: isize,
    column: isize,
    row_delta: isize,
    column_delta: isize,
    steps: isize,
}

impl PartialOrd for VertexInDijkstra {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VertexInDijkstra {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

/// # Panics
#[must_use]
pub fn part1(input: &str) -> String {
    let board = parse_input(input);
    let size = board.len() as isize;

    let start = VertexInDijkstra::new(0, 0, 0, 0, 0, 0);
    let mut q: BinaryHeap<VertexInDijkstra> = BinaryHeap::new();
    q.push(start);

    let mut visited: HashSet<VisitedVertex> = HashSet::new();

    let mut result = 0;

    while !&q.is_empty() {
        let v = q.pop().unwrap();
        if v.row == size - 1 && v.column == size - 1 {
            result = v.heat_loss;
            break;
        }

        if visited.contains(&v.to_visited_vertex()) {
            continue;
        }

        visited.insert(v.to_visited_vertex());

        if v.steps < 3 && (v.row_delta, v.column_delta) != (0, 0) {
            let next_row = v.row + v.row_delta;
            let next_column = v.column + v.column_delta;
            if next_row >= 0 && next_column >= 0 && next_row < size && next_column < size {
                q.push(VertexInDijkstra::new(
                    v.heat_loss
                        + board
                            .get(next_row as usize)
                            .unwrap()
                            .get(next_column as usize)
                            .unwrap(),
                    next_row,
                    next_column,
                    v.row_delta,
                    v.column_delta,
                    v.steps + 1,
                ));
            }
        }

        for (row_delta, column_delta) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (row_delta, column_delta) != (v.row_delta, v.column_delta)
                && (row_delta, column_delta) != (-v.row_delta, -v.column_delta)
            {
                let next_row = v.row + row_delta;
                let next_column = v.column + column_delta;
                if next_row >= 0 && next_column >= 0 && next_row < size && next_column < size {
                    q.push(VertexInDijkstra::new(
                        v.heat_loss
                            + board
                                .get(next_row as usize)
                                .unwrap()
                                .get(next_column as usize)
                                .unwrap(),
                        next_row,
                        next_column,
                        row_delta,
                        column_delta,
                        1,
                    ));
                }
            }
        }
    }

    format!("{result}")
}
/// # Panics
#[must_use]
pub fn part2(input: &str) -> String {
    let board = parse_input(input);
    let size = board.len() as isize;

    let start = VertexInDijkstra::new(0, 0, 0, 0, 0, 0);
    let mut q: BinaryHeap<VertexInDijkstra> = BinaryHeap::new();
    q.push(start);

    let mut visited: HashSet<VisitedVertex> = HashSet::new();

    let mut result = 0;

    while !&q.is_empty() {
        let v = q.pop().unwrap();
        if v.row == size - 1 && v.column == size - 1 && v.steps >= 4 {
            result = v.heat_loss;
            break;
        }

        if visited.contains(&v.to_visited_vertex()) {
            continue;
        }

        visited.insert(v.to_visited_vertex());

        if v.steps < 10 && (v.row_delta, v.column_delta) != (0, 0) {
            let next_row = v.row + v.row_delta;
            let next_column = v.column + v.column_delta;
            if next_row >= 0 && next_column >= 0 && next_row < size && next_column < size {
                q.push(VertexInDijkstra::new(
                    v.heat_loss
                        + board
                            .get(next_row as usize)
                            .unwrap()
                            .get(next_column as usize)
                            .unwrap(),
                    next_row,
                    next_column,
                    v.row_delta,
                    v.column_delta,
                    v.steps + 1,
                ));
            }
        }

        if v.steps >= 4 || (v.row_delta, v.column_delta) == (0, 0) {
            for (row_delta, column_delta) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (row_delta, column_delta) != (v.row_delta, v.column_delta)
                    && (row_delta, column_delta) != (-v.row_delta, -v.column_delta)
                {
                    let next_row = v.row + row_delta;
                    let next_column = v.column + column_delta;
                    if next_row >= 0 && next_column >= 0 && next_row < size && next_column < size {
                        q.push(VertexInDijkstra::new(
                            v.heat_loss
                                + board
                                    .get(next_row as usize)
                                    .unwrap()
                                    .get(next_column as usize)
                                    .unwrap(),
                            next_row,
                            next_column,
                            row_delta,
                            column_delta,
                            1,
                        ));
                    }
                }
            }
        }
    }

    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

    #[test]
    fn test_case_part1() {
        assert_eq!("102", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("94", part2(TEST_DATA));
    }
}
