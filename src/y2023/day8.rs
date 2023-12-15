use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Choice {
    left: String,
    right: String,
}

impl Choice {
    fn make_choice(&self, c: char) -> String {
        match c {
            'L' => {
                return String::from(self.left.as_str());
            }
            'R' => {
                return String::from(self.right.as_str());
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Network {
    sides: String,
    elements: HashMap<String, Choice>,
}

fn parse_data(input: &str) -> Network {
    let mut split = input.lines();
    let sides = split.next().unwrap().to_string();
    split.next().unwrap();
    let mut elements = HashMap::new();
    for line in split {
        let mut definition_split = line.split('=');
        let key = definition_split.next().unwrap().trim().to_string();
        let mut choice_split = definition_split.next().unwrap().split(',');
        let left = choice_split
            .next()
            .unwrap()
            .replace('(', "")
            .trim()
            .to_string();
        let right = choice_split
            .next()
            .unwrap()
            .replace(')', "")
            .trim()
            .to_string();
        let value = Choice { left, right };
        elements.insert(key, value);
    }
    Network { sides, elements }
}

#[must_use] pub fn part1(input: &str) -> String {
    let network = parse_data(input);
    let mut result = 0;
    let mut side_iterator = 0;
    let sides_length = network.sides.len();
    let mut current_node = String::from("AAA");

    while current_node != *"ZZZ" {
        let direction = network.sides.chars().nth(side_iterator).unwrap();
        current_node = network
            .elements
            .get(&current_node)
            .unwrap()
            .make_choice(direction);
        side_iterator = (side_iterator + 1) % sides_length;
        result += 1;
    }

    format!("{result}")
}

fn vector_lcm(arguments: Vec<usize>) -> usize {
    arguments
        .into_iter()
        .fold(1, |acc, x| num::integer::lcm(x, acc))
}

#[must_use] pub fn part2(input: &str) -> String {
    let network = parse_data(input);
    let ending_with_a: Vec<String> = network
        .elements
        .iter()
        .filter_map(|(k, _v)| {
            if k.ends_with('A') {
                return Some(String::from(k));
            };
            None
        })
        .collect();

    let mut cycles = HashMap::new();

    for start_node in ending_with_a {
        let mut result = 0;
        let mut side_iterator = 0;
        let sides_length = network.sides.len();
        let mut current_node = String::from(start_node.as_str());

        while !current_node.ends_with('Z') {
            let direction = network.sides.chars().nth(side_iterator).unwrap();
            current_node = network
                .elements
                .get(&current_node)
                .unwrap()
                .make_choice(direction);
            side_iterator = (side_iterator + 1) % sides_length;
            result += 1;
        }
        cycles.insert(String::from(start_node.as_str()), result);
    }

    // XXX: calculating lcm over shortest cycles here doesn't have to be a general solution but works for my
    // input. I think there could be possibility that there could be smaller lcm for some value
    // other then shortest cycle. For example if next node after smallest cycle would also end with
    // 'Z' but have prime factorization yielding smaller lcm with other cycles lengths. So in
    // general after finding shortest cycle we should probably search if there are some other
    // cycles smaller than 2 * {smallest cycle}, calculate lcm from all such possible cycle lengths
    // and then return the smallest.
    let cycle_lengths: Vec<usize> = cycles.into_values().collect();
    let result = vector_lcm(cycle_lengths);
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA1: &str = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA2: &str = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA_PART2: &str = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";

    #[test]
    fn parse_data_test() {
        let mut elements = HashMap::new();
        elements.insert(
            "AAA".to_string(),
            Choice {
                left: String::from("BBB"),
                right: String::from("BBB"),
            },
        );
        elements.insert(
            "BBB".to_string(),
            Choice {
                left: String::from("AAA"),
                right: String::from("ZZZ"),
            },
        );
        elements.insert(
            "ZZZ".to_string(),
            Choice {
                left: String::from("ZZZ"),
                right: String::from("ZZZ"),
            },
        );
        let expected = Network {
            sides: String::from("LLR"),
            elements,
        };
        assert_eq!(expected, parse_data(TEST_DATA2));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("2", part1(TEST_DATA1));
        assert_eq!("6", part1(TEST_DATA2));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("6", part2(TEST_DATA_PART2));
    }
}
