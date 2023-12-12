#[derive(Debug, PartialEq)]
struct Row {
    row: String,
    damaged: Vec<usize>,
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
        }
    }

    fn count_possibilities(&self) -> usize {
        let length = self.row.len();
        let damaged_number = self.damaged.iter().sum();
        self.backtrack(String::new(), 0, length, damaged_number, 0)
    }

    fn backtrack(
        &self,
        current_string: String,
        length: usize,
        real_length: usize,
        damaged_number: usize,
        number_of_hashes: usize,
    ) -> usize {
        if length == real_length {
            if self.is_correct(&current_string) {
                return 1;
            }
            return 0;
        }

        if number_of_hashes > damaged_number {
            return 0;
        }

        if damaged_number - number_of_hashes > real_length - length {
            return 0;
        }

        let current_char = self.row.chars().nth(length).unwrap();
        match current_char {
            '#' => {
                let mut new_string = current_string.clone();
                new_string.push('#');
                self.backtrack(
                    new_string,
                    length + 1,
                    real_length,
                    damaged_number,
                    number_of_hashes + 1,
                )
            }
            '.' => {
                let mut new_string = current_string.clone();
                new_string.push('.');
                self.backtrack(
                    new_string,
                    length + 1,
                    real_length,
                    damaged_number,
                    number_of_hashes,
                )
            }

            '?' => {
                let mut new_string1 = current_string.clone();
                let mut new_string2 = current_string.clone();
                new_string1.push('#');
                new_string2.push('.');
                let possible1 = self.is_possible_solution(&new_string1);
                let possible2 = self.is_possible_solution(&new_string2);
                if possible1 && possible2 {
                    self.backtrack(
                        new_string1,
                        length + 1,
                        real_length,
                        damaged_number,
                        number_of_hashes + 1,
                    ) + self.backtrack(
                        new_string2,
                        length + 1,
                        real_length,
                        damaged_number,
                        number_of_hashes,
                    )
                } else if possible1 {
                    self.backtrack(
                        new_string1,
                        length + 1,
                        real_length,
                        damaged_number,
                        number_of_hashes + 1,
                    )
                } else if possible2 {
                    self.backtrack(
                        new_string2,
                        length + 1,
                        real_length,
                        damaged_number,
                        number_of_hashes,
                    )
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn is_correct(&self, current_string: &str) -> bool {
        //println!("{current_string} {:?}", self);
        let mut damaged_number = 0;
        let mut string_iter = current_string.chars();
        let mut damaged_iter = self.damaged.iter().peekable();
        while let Some(c) = string_iter.next() {
            if c == '.' {
                if damaged_number > 0 && damaged_iter.peek().is_none() {
                    //println!("FALSE1");
                    return false;
                }
                if damaged_number > 0 && *damaged_iter.next().unwrap() != damaged_number {
                    //println!("FALSE2");
                    return false;
                }
                damaged_number = 0;
                continue;
            }
            damaged_number += 1;
        }

        if damaged_number > 0 && damaged_iter.peek().is_none() {
            //println!("FALSE3");
            return false;
        }
        if damaged_number > 0 && *damaged_iter.next().unwrap() != damaged_number {
            //println!("FALSE4");
            return false;
        }

        if damaged_iter.peek().is_some() {
            return false;
        }

        true
    }

    fn is_possible_solution(&self, current_string: &str) -> bool {
        let mut damaged_number = 0;
        let mut string_iter = current_string.chars();
        let mut damaged_iter = self.damaged.iter().peekable();
        while let Some(c) = string_iter.next() {
            if c == '.' {
                if damaged_number > 0 && damaged_iter.peek().is_none() {
                    return false;
                }
                if damaged_number > 0 && *damaged_iter.next().unwrap() != damaged_number {
                    return false;
                }
                damaged_number = 0;
                continue;
            }
            damaged_number += 1;
        }

        if damaged_number > 0 && damaged_iter.peek().is_none() {
            return false;
        }
        if damaged_number > 0 && *damaged_iter.next().unwrap() < damaged_number {
            return false;
        }

        true
    }
}

pub fn part1(input: &str) -> String {
    let rows: Vec<Row> = input.lines().map(|line| Row::parse(line.trim())).collect();
    let mut result = 0;
    for row in rows {
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
    let mut counter = 1;
    for row in rows {
        println!("{counter}");
        let possibilities = row.count_possibilities();
        result += possibilities;
        counter += 1;
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
        assert_eq!(Row { row, damaged }, Row::parse(input));
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
        assert!(row.is_correct(test_input1));
        assert!(row.is_correct(test_input2));
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
