use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Relation {
    Greater,
    Less,
}

#[derive(Debug, PartialEq)]
struct Rule {
    dimension: Option<char>,
    relation: Option<Relation>,
    value: Option<usize>,
    goto: String,
}

impl Rule {
    fn parse(input: &str) -> Self {
        if input.contains(':') {
            let mut split = input.split(':');
            let first_part = split.next().unwrap();
            let dimension;
            let relation;
            let value;
            if first_part.contains('<') {
                let mut inner_split = first_part.split('<');
                dimension = inner_split.next().unwrap().chars().next().unwrap();
                relation = Relation::Less;
                value = inner_split.next().unwrap().parse().unwrap();
            } else {
                let mut inner_split = first_part.split('>');
                relation = Relation::Greater;
                dimension = inner_split.next().unwrap().chars().next().unwrap();
                value = inner_split.next().unwrap().parse().unwrap();
            }

            let goto = String::from(split.next().unwrap());
            Rule {
                dimension: Some(dimension),
                relation: Some(relation),
                value: Some(value),
                goto,
            }
        } else {
            Rule {
                dimension: None,
                relation: None,
                value: None,
                goto: String::from(input),
            }
        }
    }

    fn applies(&self, part: &Part) -> Option<String> {
        if self.dimension.is_none() {
            return Some(self.goto.to_owned());
        }
        match (&self.dimension.unwrap(), &self.relation.unwrap()) {
            ('x', Relation::Greater) => {
                if part.x > self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('x', Relation::Less) => {
                if part.x < self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('m', Relation::Greater) => {
                if part.m > self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('m', Relation::Less) => {
                if part.m < self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('a', Relation::Greater) => {
                if part.a > self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('a', Relation::Less) => {
                if part.a < self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('s', Relation::Greater) => {
                if part.s > self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            ('s', Relation::Less) => {
                if part.s < self.value.unwrap() {
                    return Some(self.goto.to_owned());
                }
                return None;
            }
            _ => unreachable!(),
        }
    }

    fn split_range(&self, ranges: &Ranges) -> (String, Option<Ranges>, Option<Ranges>) {
        let goto = self.goto.to_owned();
        if self.dimension.is_none() {
            return (goto, Some(ranges.clone()), None);
        }
        let value = self.value.unwrap();
        match (&self.dimension.unwrap(), &self.relation.unwrap()) {
            ('x', Relation::Greater) => {
                if value > ranges.x_max {
                    return (goto, None, Some(ranges.clone()));
                }
                if value < ranges.x_min {
                    return (goto, Some(ranges.clone()), None);
                }
                let (r1, r2) = ranges.split('x', value);
                return (goto, r2, Some(r1));
            }
            ('x', Relation::Less) => {
                if value > ranges.x_max {
                    return (goto, Some(ranges.clone()), None);
                }
                if value < ranges.x_min {
                    return (goto, None, Some(ranges.clone()));
                }
                let (r1, r2) = ranges.split('x', value - 1);
                return (goto, Some(r1), r2);
            }
            ('m', Relation::Greater) => {
                if value > ranges.m_max {
                    return (goto, None, Some(ranges.clone()));
                }
                if value < ranges.m_min {
                    return (goto, Some(ranges.clone()), None);
                }
                let (r1, r2) = ranges.split('m', value);
                return (goto, r2, Some(r1));
            }
            ('m', Relation::Less) => {
                if value > ranges.m_max {
                    return (goto, Some(ranges.clone()), None);
                }
                if value < ranges.m_min {
                    return (goto, None, Some(ranges.clone()));
                }
                let (r1, r2) = ranges.split('m', value - 1);
                return (goto, Some(r1), r2);
            }
            ('a', Relation::Greater) => {
                if value > ranges.a_max {
                    return (goto, None, Some(ranges.clone()));
                }
                if value < ranges.a_min {
                    return (goto, Some(ranges.clone()), None);
                }
                let (r1, r2) = ranges.split('a', value);
                return (goto, r2, Some(r1));
            }
            ('a', Relation::Less) => {
                if value > ranges.a_max {
                    return (goto, Some(ranges.clone()), None);
                }
                if value < ranges.a_min {
                    return (goto, None, Some(ranges.clone()));
                }
                let (r1, r2) = ranges.split('a', value - 1);
                return (goto, Some(r1), r2);
            }
            ('s', Relation::Greater) => {
                if value > ranges.s_max {
                    return (goto, None, Some(ranges.clone()));
                }
                if value < ranges.s_min {
                    return (goto, Some(ranges.clone()), None);
                }
                let (r1, r2) = ranges.split('s', value);
                return (goto, r2, Some(r1));
            }
            ('s', Relation::Less) => {
                if value > ranges.s_max {
                    return (goto, Some(ranges.clone()), None);
                }
                if value < ranges.s_min {
                    return (goto, None, Some(ranges.clone()));
                }
                let (r1, r2) = ranges.split('s', value - 1);
                return (goto, Some(r1), r2);
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> Self {
        let mut split = input.split('{');
        let name = split.next().unwrap().to_owned();
        let rules = split
            .next()
            .unwrap()
            .replace("}", "")
            .split(',')
            .into_iter()
            .map(|rule| Rule::parse(rule))
            .collect();
        Workflow { name, rules }
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Part { x, m, a, s }
    }

    fn parse(input: &str) -> Self {
        let mut split = input.trim().split('=');
        let _first = split.next().unwrap();
        let x = split
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let m = split
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let a = split
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let s = split
            .next()
            .unwrap()
            .split('}')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        Part { x, m, a, s }
    }

    fn xmas_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_data(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut split = input.split("\n\n");
    let workflows = split
        .next()
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| Workflow::parse(line))
        .collect();
    let parts = split
        .next()
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| Part::parse(line))
        .collect();
    (workflows, parts)
}

fn workflows_to_map(workflows: Vec<Workflow>) -> HashMap<String, Vec<Rule>> {
    let mut result = HashMap::new();

    for workflow in workflows {
        result.insert(workflow.name, workflow.rules);
    }

    result
}

fn eval(part: Part, workflow_map: &HashMap<String, Vec<Rule>>) -> usize {
    let mut current_state = String::from("in");

    'outer: while &current_state != "A" && &current_state != "R" {
        for rule in workflow_map.get(&current_state).unwrap() {
            if let Some(goto) = rule.applies(&part) {
                current_state = goto;
                continue 'outer;
            }
        }
    }

    if current_state == "R" {
        0
    } else {
        part.xmas_rating()
    }
}

pub fn part1(input: &str) -> String {
    let (workflows, parts) = parse_data(input);
    let workflow_map = workflows_to_map(workflows);
    let mut result = 0;

    for part in parts {
        result += eval(part, &workflow_map)
    }

    format!("{result}")
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Ranges {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

impl Ranges {
    fn new(
        x_min: usize,
        x_max: usize,
        m_min: usize,
        m_max: usize,
        a_min: usize,
        a_max: usize,
        s_min: usize,
        s_max: usize,
    ) -> Self {
        Ranges {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        }
    }

    fn number_of_elements(&self) -> usize {
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }

    fn split(&self, dimension: char, value: usize) -> (Ranges, Option<Ranges>) {
        match dimension {
            'x' => {
                if value < self.x_min || value > self.x_max {
                    return (self.clone(), None);
                }
                return (
                    Ranges::new(
                        self.x_min, value, self.m_min, self.m_max, self.a_min, self.a_max,
                        self.s_min, self.s_max,
                    ),
                    Some(Ranges::new(
                        value + 1,
                        self.x_max,
                        self.m_min,
                        self.m_max,
                        self.a_min,
                        self.a_max,
                        self.s_min,
                        self.s_max,
                    )),
                );
            }
            'm' => {
                if value < self.m_min || value > self.m_max {
                    return (self.clone(), None);
                }
                return (
                    Ranges::new(
                        self.x_min, self.x_max, self.m_min, value, self.a_min, self.a_max,
                        self.s_min, self.s_max,
                    ),
                    Some(Ranges::new(
                        self.x_min,
                        self.x_max,
                        value + 1,
                        self.m_max,
                        self.a_min,
                        self.a_max,
                        self.s_min,
                        self.s_max,
                    )),
                );
            }
            'a' => {
                if value < self.a_min || value > self.a_max {
                    return (self.clone(), None);
                }
                return (
                    Ranges::new(
                        self.x_min, self.x_max, self.m_min, self.m_max, self.a_min, value,
                        self.s_min, self.s_max,
                    ),
                    Some(Ranges::new(
                        self.x_min,
                        self.x_max,
                        self.m_min,
                        self.m_max,
                        value + 1,
                        self.a_max,
                        self.s_min,
                        self.s_max,
                    )),
                );
            }
            's' => {
                if value < self.s_min || value > self.s_max {
                    return (self.clone(), None);
                }
                return (
                    Ranges::new(
                        self.x_min, self.x_max, self.m_min, self.m_max, self.a_min, self.a_max,
                        self.s_min, value,
                    ),
                    Some(Ranges::new(
                        self.x_min,
                        self.x_max,
                        self.m_min,
                        self.m_max,
                        self.a_min,
                        self.a_max,
                        value + 1,
                        self.s_max,
                    )),
                );
            }
            _ => unreachable!(),
        }
    }
}

fn eval_part2(
    current_state: String,
    workflow_map: &HashMap<String, Vec<Rule>>,
    ranges: Ranges,
) -> usize {
    if &current_state == "A" {
        return ranges.number_of_elements();
    } else if &current_state == "R" {
        return 0;
    }

    let mut result = 0;
    let rules = workflow_map.get(&current_state).unwrap();
    let mut current_range = Some(ranges);
    for rule in rules {
        let (goto, r1, r2) = rule.split_range(&current_range.unwrap());
        if let Some(r) = r1 {
            result += eval_part2(goto, &workflow_map, r)
        }
        current_range = r2;
    }

    result
}

pub fn part2(input: &str) -> String {
    let (workflows, _) = parse_data(input);
    let workflow_map = workflows_to_map(workflows);

    let result = eval_part2(
        String::from("in"),
        &workflow_map,
        Ranges {
            x_min: 1,
            x_max: 4000,
            m_min: 1,
            m_max: 4000,
            a_min: 1,
            a_max: 4000,
            s_min: 1,
            s_max: 4000,
        },
    );

    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn parse_part() {
        let input = "{x=2036,m=264,a=79,s=2244}";
        let expected = Part {
            x: 2036,
            m: 264,
            a: 79,
            s: 2244,
        };
        assert_eq!(expected, Part::parse(input));
    }

    #[test]
    fn parse_rule_with_optional_values() {
        let input = "a<2006:qkq";
        let expected = Rule {
            dimension: Some('a'),
            relation: Some(Relation::Less),
            value: Some(2006),
            goto: String::from("qkq"),
        };
        assert_eq!(expected, Rule::parse(input));
    }

    #[test]
    fn parse_rule_without_optional_values() {
        let input = "rfg";
        let expected = Rule {
            dimension: None,
            relation: None,
            value: None,
            goto: String::from("rfg"),
        };
        assert_eq!(expected, Rule::parse(input));
    }

    #[test]
    fn parse_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let expected = Workflow {
            name: String::from("px"),
            rules: vec![
                Rule {
                    dimension: Some('a'),
                    relation: Some(Relation::Less),
                    value: Some(2006),
                    goto: String::from("qkq"),
                },
                Rule {
                    dimension: Some('m'),
                    relation: Some(Relation::Greater),
                    value: Some(2090),
                    goto: String::from("A"),
                },
                Rule {
                    dimension: None,
                    relation: None,
                    value: None,
                    goto: String::from("rfg"),
                },
            ],
        };
        assert_eq!(expected, Workflow::parse(input));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("19114", part1(TEST_DATA));
    }

    #[test]
    fn split_ranges() {
        let range = Ranges::new(0, 1000, 0, 1000, 0, 1000, 0, 1000);
        let expected1 = Ranges::new(0, 1000, 0, 500, 0, 1000, 0, 1000);
        let expected2 = Ranges::new(0, 1000, 501, 1000, 0, 1000, 0, 1000);
        assert_eq!((expected1, Some(expected2)), range.split('m', 500));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("167409079868000", part2(TEST_DATA));
    }
}
