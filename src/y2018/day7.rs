use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type Vertex = char;

#[derive(Debug, PartialEq)]
struct Edge {
    from: Vertex,
    to: Vertex,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Step (?<from>.) must be finished before step (?<to>.) can begin.").unwrap();
}

fn parse_row(row: &str) -> Edge {
    let captures = RE.captures(row).unwrap();
    Edge {
        from: captures["from"].chars().next().unwrap(),
        to: captures["to"].chars().next().unwrap(),
    }
}

fn find_next(edges: &Vec<Edge>) -> Vec<Vertex> {
    let mut froms: HashSet<Vertex> = HashSet::new();
    let mut tos: HashSet<Vertex> = HashSet::new();
    for Edge { from, to } in edges {
        froms.insert(*from);
        tos.insert(*to);
    }
    froms.retain(|x| !tos.contains(x));
    let mut possible: Vec<Vertex> = froms.into_iter().collect::<Vec<Vertex>>();
    possible.sort();
    possible
}

pub fn part1(input: &str) -> String {
    let mut edges: Vec<Edge> = input.lines().map(|x| parse_row(x)).collect();
    let mut result = String::new();
    let mut rest: Vec<Edge> = Vec::new();
    while edges.len() > 0 {
        let next = find_next(&edges)[0];
        result.push(next);
        (edges, rest) = edges.into_iter().partition(|x| x.from != next);
    }
    let mut rest: Vec<Vertex> = rest.into_iter().map(|x| x.to).collect();
    rest.sort();
    for v in rest {
        result.push(v);
    }
    result
}

fn time_for_task(v: Vertex, time_constant: u8) -> u8 {
    (v as u8) + time_constant - 64
}

fn time_for_the_rest(rest: &Vec<Edge>, time_constant: u8) -> u64 {
    println!("time_for_the_rest: {:?}", rest);
    let mut x = 0;
    let mut y = 0;
    let tasks: Vec<u8> = rest
        .iter()
        .map(|e| time_for_task(e.to, time_constant))
        .collect();
    for t in tasks {
        if x <= y {
            x += t
        } else {
            y += t
        }
    }
    std::cmp::max(x, y) as u64
}

#[derive(Debug, Clone)]
struct Worker {
    current_task: char,
    time_to_finish: u8,
}

fn organize_work(mut edges: Vec<Edge>, time_constant: u8, number_of_workers: usize) -> u64 {
    let mut time = 0;
    let mut rest1: Vec<Edge> = Vec::new();
    let mut rest2: Vec<Edge> = Vec::new();
    let workers: Vec<Worker> = vec![
        Worker {
            current_task: '.',
            time_to_finish: 0
        };
        number_of_workers
    ];
    let mut worker1_to_finish = 0;
    let mut worker2_to_finish = 0;
    let mut worker1_task = ' ';
    let mut worker2_task = ' ';
    while edges.len() > 0 {
        if worker1_to_finish == 0 {
            (edges, rest1) = edges.into_iter().partition(|x| x.from != worker1_task);
            if let Some(next_task) = find_next(&edges)
                .iter()
                .filter(|&&x| x != worker2_task)
                .next()
            {
                worker1_task = *next_task;
                worker1_to_finish = time_for_task(*next_task, time_constant);
            }
        }
        if worker2_to_finish == 0 {
            (edges, rest2) = edges.into_iter().partition(|x| x.from != worker2_task);
            if let Some(next_task) = find_next(&edges)
                .iter()
                .filter(|&&x| x != worker1_task)
                .next()
            {
                worker2_task = *next_task;
                worker2_to_finish = time_for_task(*next_task, time_constant);
            }
        }
        worker1_to_finish = if worker1_to_finish > 0 {
            worker1_to_finish - 1
        } else {
            worker1_task = '.';
            0
        };
        worker2_to_finish = if worker2_to_finish > 0 {
            worker2_to_finish - 1
        } else {
            worker2_task = '.';
            0
        };
        time += 1;
    }
    rest1.append(&mut rest2);
    let rest_time = time_for_the_rest(&rest1, time_constant);
    time + rest_time - 1
}

pub fn part2(input: &str) -> u64 {
    let edges: Vec<Edge> = input.lines().map(|x| parse_row(x)).collect();
    organize_work(edges, 60, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_CASE_INPUT: &str = "Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.";

    #[test]
    fn parse_row1() {
        let input = "Step A must be finished before step B can begin.";
        assert_eq!(Edge { from: 'A', to: 'B' }, parse_row(input));
    }

    #[test]
    fn find_next1() {
        assert_eq!(
            'A',
            find_next(&vec![
                Edge { from: 'A', to: 'B' },
                Edge { from: 'A', to: 'C' }
            ])[0]
        );
    }

    #[test]
    fn find_next2() {
        assert_eq!(
            'C',
            find_next(&vec![
                Edge { from: 'C', to: 'A' },
                Edge { from: 'C', to: 'F' },
                Edge { from: 'A', to: 'B' },
                Edge { from: 'A', to: 'D' },
                Edge { from: 'B', to: 'E' },
                Edge { from: 'D', to: 'E' },
                Edge { from: 'F', to: 'E' },
            ])[0]
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!("CABDFE", part1(TEST_CASE_INPUT));
    }

    #[test]
    fn time_for_task1() {
        assert_eq!(6, time_for_task('F', 0));
    }

    #[test]
    fn time_for_task2() {
        assert_eq!(61, time_for_task('A', 60));
    }

    #[test]
    fn time_for_the_rest1() {
        assert_eq!(
            10,
            time_for_the_rest(
                &vec![
                    Edge { from: 'C', to: 'A' },
                    Edge { from: 'C', to: 'F' },
                    Edge { from: 'C', to: 'E' },
                    Edge { from: 'C', to: 'D' },
                ],
                0
            )
        )
    }

    #[test]
    fn test_case2() {
        let edges: Vec<Edge> = TEST_CASE_INPUT.lines().map(|x| parse_row(x)).collect();
        assert_eq!(15, organize_work(edges, 0, 2));
    }
}
