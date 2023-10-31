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

fn find_next(edges: &Vec<Edge>) -> Vertex {
    let mut froms: HashSet<Vertex> = HashSet::new();
    let mut tos: HashSet<Vertex> = HashSet::new();
    for Edge { from, to } in edges {
        froms.insert(*from);
        tos.insert(*to);
    }
    froms.retain(|x| !tos.contains(x));
    let mut possible: Vec<Vertex> = froms.into_iter().collect::<Vec<Vertex>>();
    possible.sort();
    possible[0]
}

pub fn part1(input: &str) -> String {
    let mut edges: Vec<Edge> = input.lines().map(|x| parse_row(x)).collect();
    let mut result = String::new();
    let mut rest: Vec<Edge> = Vec::new();
    while edges.len() > 0 {
        let next = find_next(&edges);
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
            ])
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
            ])
        );
    }

    #[test]
    fn test_case1() {
        let input = "Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.";
        assert_eq!("CABDFE", part1(input));
    }
}
