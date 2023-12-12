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
        let damaged: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Row {
            row: format!("{row}?{row}?{row}?{row}?{row}"),
            damaged,
        }
    }

    fn count_possibilities(&self) -> usize {
        let length = self.row.len();
        self.backtrack(String::new(), 0, length)
    }

    fn backtrack(&self, current_string: String, length: usize, real_length: usize) -> usize {
        if length == real_length {
            if self.is_correct(&current_string) {
                return 1;
            }
            return 0;
        }
        let current_char = self.row.chars().nth(length).unwrap();
        match current_char {
            '#' => {
                let mut new_string = current_string.clone();
                new_string.push('#');
                self.backtrack(new_string, length + 1, real_length)
            }
            '.' => {
                let mut new_string = current_string.clone();
                new_string.push('.');
                self.backtrack(new_string, length + 1, real_length)
            }

            '?' => {
                let mut new_string1 = current_string.clone();
                let mut new_string2 = current_string.clone();
                new_string1.push('#');
                new_string2.push('.');
                self.backtrack(new_string1, length + 1, real_length)
                    + self.backtrack(new_string2, length + 1, real_length)
            }
            _ => unreachable!(),
        }
    }

    fn is_correct(&self, current_string: &str) -> bool {
        let mut damaged_number = 0;
        let mut numbers = Vec::new();
        for c in current_string.chars() {
            if c == '#' {
                damaged_number += 1;
            } else if c == '.' && damaged_number > 0 {
                numbers.push(damaged_number);
                damaged_number = 0;
            }
        }

        if damaged_number > 0 {
            numbers.push(damaged_number);
        }

        numbers == self.damaged
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

pub fn part2(_input: &str) -> String {
    let result = 0;
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
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("525152", part2(TEST_DATA));
    }
}
