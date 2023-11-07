use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type Vertex = char;

#[derive(Debug, PartialEq, Clone)]
struct Edge {
    from: Vertex,
    to: Vertex,
}

fn flip_edge(e: &Edge) -> Edge {
    Edge {
        from: e.to,
        to: e.from,
    }
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
    let mut possible_next_tasks = froms.into_iter().collect::<Vec<Vertex>>();
    possible_next_tasks.sort_unstable();
    possible_next_tasks
}

pub fn part1(input: &str) -> String {
    let mut edges: Vec<Edge> = input.lines().map(parse_row).collect();
    let mut result = String::new();
    let mut rest: Vec<Edge>;
    let mut tail_handled = false;
    while !edges.is_empty() {
        let next = find_next(&edges)[0];
        result.push(next);
        (edges, rest) = edges.into_iter().partition(|x| x.from != next);

        // this case handles the rest of the vertices when last requirement is met
        if edges.is_empty() && !rest.is_empty() && !tail_handled {
            edges = rest.iter().map(flip_edge).collect();
            tail_handled = true;
        }
    }
    result
}

fn time_for_task(v: Vertex, time_constant: u8) -> u8 {
    (v as u8) + time_constant - 64
}

#[derive(Debug, Clone, PartialEq)]
struct Worker {
    current_task: char,
    time_to_finish: u8,
}

impl Worker {
    fn finished(&self) -> bool {
        self.time_to_finish == 0
    }
}

fn tasks_performed_by_other(workers: &[Worker], index: usize) -> HashSet<char> {
    workers
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, worker)| worker.current_task)
        .collect()
}

fn process_next_second(
    edges: &mut Vec<Edge>,
    workers: &mut Vec<Worker>,
    time_constant: u8,
) -> Vec<Edge> {
    let mut rest: Vec<Edge> = Vec::new();
    for i in 0..workers.len() {
        if workers[i].finished() {
            let other_workers_tasks = tasks_performed_by_other(workers, i);
            let filtered_edges: Vec<Edge> = edges
                .iter()
                .filter(|x| x.from != workers[i].current_task)
                .cloned()
                .collect();
            let potential_rest: Vec<Edge> = edges
                .iter()
                .filter(|x| x.from == workers[i].current_task)
                .cloned()
                .collect();
            if !potential_rest.is_empty() {
                rest = potential_rest;
            };
            if let Some(next_task) = find_next(&filtered_edges)
                .iter()
                .find(|&&x| !other_workers_tasks.contains(&x))
            {
                workers[i].current_task = *next_task;
                workers[i].time_to_finish = time_for_task(*next_task, time_constant);
            }
            *edges = filtered_edges;
        }
        workers[i].time_to_finish = if workers[i].time_to_finish > 0 {
            workers[i].time_to_finish - 1
        } else {
            workers[i].current_task = '.';
            0
        };
    }
    rest
}

fn organize_work(mut edges: Vec<Edge>, time_constant: u8, number_of_workers: usize) -> u64 {
    let mut time = 0;
    let mut tail_handled = false;
    let mut workers: Vec<Worker> = vec![
        Worker {
            current_task: '.',
            time_to_finish: 0
        };
        number_of_workers
    ];

    let mut rest: Vec<Edge>;
    while !edges.is_empty() {
        rest = process_next_second(&mut edges, &mut workers, time_constant);
        time += 1;

        if edges.is_empty() && !rest.is_empty() && !tail_handled {
            edges = rest.iter().map(flip_edge).collect();
            tail_handled = true;
        }
    }

    if tail_handled {
        time -= 2;
    } else {
        time -= 1;
    }

    time
}

pub fn part2(input: &str) -> u64 {
    let edges: Vec<Edge> = input.lines().map(parse_row).collect();
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
    fn parse_row_example() {
        let input = "Step A must be finished before step B can begin.";
        assert_eq!(Edge { from: 'A', to: 'B' }, parse_row(input));
    }

    #[test]
    fn find_next_test1() {
        assert_eq!(
            'A',
            find_next(&vec![
                Edge { from: 'A', to: 'B' },
                Edge { from: 'A', to: 'C' }
            ])[0]
        );
    }

    #[test]
    fn find_next_test2() {
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
    fn test_case_test() {
        assert_eq!("CABDFE", part1(TEST_CASE_INPUT));
    }

    #[test]
    fn time_for_task_ascii_without_addition() {
        assert_eq!(6, time_for_task('F', 0));
    }

    #[test]
    fn time_for_task_with_addition() {
        assert_eq!(61, time_for_task('A', 60));
    }

    #[test]
    fn flip_edge_test() {
        assert_eq!(
            Edge { from: 'x', to: 'D' },
            flip_edge(&Edge { from: 'D', to: 'x' })
        );
    }

    #[test]
    fn tasks_performed_by_other_test() {
        let workers = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'B',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'C',
                time_to_finish: 10,
            },
        ];
        let mut result = HashSet::new();
        result.insert('A');
        result.insert('C');
        assert_eq!(result, tasks_performed_by_other(&workers, 1))
    }

    #[test]
    fn process_next_second_time_flows() {
        let mut edges = vec![Edge { from: 'A', to: 'B' }];
        let mut workers = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'B',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'C',
                time_to_finish: 10,
            },
        ];
        let result = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 9,
            },
            Worker {
                current_task: 'B',
                time_to_finish: 9,
            },
            Worker {
                current_task: 'C',
                time_to_finish: 9,
            },
        ];
        let _ = process_next_second(&mut edges, &mut workers, 0);
        assert_eq!(result, workers);
    }

    #[test]
    fn process_next_second_selecting_new_task() {
        let mut edges = vec![Edge { from: 'A', to: 'B' }];
        let mut workers = vec![
            Worker {
                current_task: 'Z',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'W',
                time_to_finish: 10,
            },
            Worker {
                current_task: '.',
                time_to_finish: 0,
            },
        ];
        let result = vec![
            Worker {
                current_task: 'Z',
                time_to_finish: 9,
            },
            Worker {
                current_task: 'W',
                time_to_finish: 9,
            },
            Worker {
                current_task: 'A',
                time_to_finish: 5,
            },
        ];
        let _ = process_next_second(&mut edges, &mut workers, 5);
        assert_eq!(result, workers);
    }

    #[test]
    fn process_next_second_cant_select_new_task() {
        let mut edges = vec![Edge { from: 'A', to: 'B' }];
        let mut workers = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 10,
            },
            Worker {
                current_task: 'W',
                time_to_finish: 10,
            },
            Worker {
                current_task: '.',
                time_to_finish: 0,
            },
        ];
        let result = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 9,
            },
            Worker {
                current_task: 'W',
                time_to_finish: 9,
            },
            Worker {
                current_task: '.',
                time_to_finish: 0,
            },
        ];
        let _ = process_next_second(&mut edges, &mut workers, 5);
        assert_eq!(result, workers);
    }

    #[test]
    fn process_next_second_result_test() {
        let mut edges = vec![Edge { from: 'A', to: 'B' }];
        let mut workers = vec![
            Worker {
                current_task: 'A',
                time_to_finish: 0,
            },
            Worker {
                current_task: '.',
                time_to_finish: 0,
            },
        ];
        let expected_result = edges.clone();
        let restult = process_next_second(&mut edges, &mut workers, 5);
        assert_eq!(expected_result, restult);
    }

    #[test]
    fn test_case2() {
        let edges: Vec<Edge> = TEST_CASE_INPUT.lines().map(|x| parse_row(x)).collect();
        assert_eq!(15, organize_work(edges, 0, 2));
    }
}
