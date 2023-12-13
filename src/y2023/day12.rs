use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Row {
    row: String,
    damaged: Vec<usize>,
    map: HashMap<(usize, usize, usize), usize>,
}

impl Row {
    fn parse(input: &str) -> Row {
        let mut split = input.split(' ');
        let row = split.next().unwrap();
        let damaged: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Row {
            row: String::from(row),
            damaged,
            map: HashMap::new(),
        }
    }

    fn parse_part2(input: &str) -> Row {
        let mut split = input.split(' ');
        let row = split.next().unwrap();
        let numbers: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut damaged = Vec::new();
        for _i in 0..5 {
            damaged.extend(numbers.clone());
        }

        Row {
            row: format!("{row}?{row}?{row}?{row}?{row}"),
            damaged,
            map: HashMap::new(),
        }
    }

    fn count_possibilities(&mut self) -> usize {
        let length = self.row.len();
        self.backtrack("", 0, length, 0, 0)
    }

    fn backtrack(
        &mut self,
        current_string: &str,
        length: usize,
        real_length: usize,
        number_of_hashes: usize,
        blocks_placed: usize,
    ) -> usize {
        if self
            .map
            .contains_key(&(length, number_of_hashes, blocks_placed))
        {
            let value = *self
                .map
                .get(&(length, number_of_hashes, blocks_placed))
                .unwrap();
            return value;
        }

        if length == real_length {
            if self.is_correct(current_string).0 {
                return 1;
            }
            return 0;
        }

        let current_char = self.row.chars().nth(length).unwrap();
        match current_char {
            '#' => {
                let mut new_string = current_string.to_string();
                new_string.push('#');
                self.backtrack(
                    &new_string,
                    length + 1,
                    real_length,
                    number_of_hashes + 1,
                    blocks_placed,
                )
            }
            '.' => {
                let mut new_string = current_string.to_string();
                new_string.push('.');
                let result = self.backtrack(
                    &new_string,
                    length + 1,
                    real_length,
                    number_of_hashes,
                    blocks_placed,
                );
                self.map
                    .insert((length, number_of_hashes, blocks_placed), result);
                result
            }

            '?' => {
                let mut new_string1 = current_string.to_string();
                let mut new_string2 = current_string.to_string();
                new_string1.push('#');
                new_string2.push('.');
                let (possible1, blocks_placed1) = self.is_possible_solution(&new_string1);
                let (possible2, blocks_placed2) = self.is_possible_solution(&new_string2);
                if possible1 && possible2 {
                    let result = self.backtrack(
                        &new_string1,
                        length + 1,
                        real_length,
                        number_of_hashes + 1,
                        blocks_placed1,
                    ) + self.backtrack(
                        &new_string2,
                        length + 1,
                        real_length,
                        number_of_hashes,
                        blocks_placed2,
                    );
                    self.map
                        .insert((length, number_of_hashes, blocks_placed), result);
                    result
                } else if possible1 {
                    self.backtrack(
                        &new_string1,
                        length + 1,
                        real_length,
                        number_of_hashes + 1,
                        blocks_placed1,
                    )
                } else if possible2 {
                    let result = self.backtrack(
                        &new_string2,
                        length + 1,
                        real_length,
                        number_of_hashes,
                        blocks_placed2,
                    );
                    self.map
                        .insert((length, number_of_hashes, blocks_placed), result);
                    result
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn is_correct(&self, current_string: &str) -> (bool, usize) {
        let mut blocks_placed = 0;
        let mut damaged_number = 0;
        let mut damaged_iter = self.damaged.iter().peekable();
        for c in current_string.chars() {
            if c == '.' {
                if damaged_number > 0 && damaged_iter.peek().is_none() {
                    return (false, blocks_placed);
                }
                if damaged_number > 0 && *damaged_iter.next().unwrap() != damaged_number {
                    return (false, blocks_placed);
                }
                damaged_number = 0;
                blocks_placed += 1;
                continue;
            }
            damaged_number += 1;
        }

        if damaged_number > 0 && damaged_iter.peek().is_none() {
            return (false, blocks_placed);
        }
        if damaged_number > 0 && *damaged_iter.next().unwrap() != damaged_number {
            return (false, blocks_placed);
        }

        if damaged_iter.peek().is_some() {
            return (false, blocks_placed);
        }

        blocks_placed += 1;

        (true, blocks_placed)
    }

    fn is_possible_solution(&self, current_string: &str) -> (bool, usize) {
        let mut blocks_placed = 0;
        let mut damaged_number = 0;
        let mut damaged_iter = self.damaged.iter().peekable();
        for c in current_string.chars() {
            if c == '.' {
                if damaged_number > 0 && damaged_iter.peek().is_none() {
                    return (false, blocks_placed);
                }
                if damaged_number > 0 {
                    if *damaged_iter.next().unwrap() != damaged_number {
                        return (false, blocks_placed);
                    }
                    blocks_placed += 1;
                }
                damaged_number = 0;
                continue;
            }
            damaged_number += 1;
        }

        if damaged_number > 0 && damaged_iter.peek().is_none() {
            return (false, blocks_placed);
        }
        if damaged_number > 0 && *damaged_iter.next().unwrap() < damaged_number {
            return (false, blocks_placed);
        }

        blocks_placed += 1;

        (true, blocks_placed)
    }
}

pub fn part1(input: &str) -> String {
    let rows: Vec<Row> = input.lines().map(|line| Row::parse(line.trim())).collect();
    let mut result = 0;
    for mut row in rows {
        let possibilities = row.count_possibilities();
        result += possibilities;
    }
    format!("{result}")
}

pub fn part2(input: &str) -> String {
    let rows: Vec<Row> = input
        .lines()
        .map(|line| Row::parse_part2(line.trim()))
        .collect();
    let mut result = 0;
    for mut row in rows {
        let possibilities = row.count_possibilities();
        result += possibilities;
    }
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

    #[test]
    fn parse_test() {
        let input = "????.######..#####. 1,6,5";
        let row = String::from("????.######..#####.");
        let damaged = Vec::from([1, 6, 5]);
        assert_eq!(
            Row {
                row,
                damaged,
                map: HashMap::new()
            },
            Row::parse(input)
        );
    }

    #[test]
    fn count_possibilities_test1() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(4, Row::parse(input).count_possibilities());
    }

    #[test]
    fn count_possibilities_test2() {
        let input = "???.### 1,1,3";
        assert_eq!(1, Row::parse(input).count_possibilities());
    }

    #[test]
    fn is_correct_test() {
        let input = ".??..??...?##. 1,1,3";
        let row = Row::parse(input);
        let test_input1 = "..#...#....###.";
        let test_input2 = ".#...#.....###.";
        assert!(row.is_correct(test_input1).0);
        assert!(row.is_correct(test_input2).0);
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("21", part1(TEST_DATA));
    }

    #[test]
    fn count_possibilities_part2() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(2500, Row::parse_part2(input).count_possibilities());
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("525152", part2(TEST_DATA));
    }
}
