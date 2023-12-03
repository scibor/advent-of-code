use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position(isize, isize);

#[derive(Debug, PartialEq)]
struct Number {
    pos: Position,
    value: String,
}

#[derive(Debug, PartialEq)]
struct Board {
    height: usize,
    width: usize,
    numbers: Vec<Number>,
    symbols: HashMap<Position, char>,
}

impl Board {
    fn adjacent_to_symbol(&self, number: &Number) -> bool {
        let number_positions = Self::get_number_positions(number);
        for position in number_positions {
            let neighbors = Self::get_neighbors(&position);
            for neighbor in neighbors {
                if self.symbols.contains_key(&neighbor) {
                    return true;
                }
            }
        }
        false
    }

    fn get_number_positions(number: &Number) -> Vec<Position> {
        let mut number_positions = Vec::new();
        let Position(y, x) = number.pos;
        for i in 0..number.value.len() {
            number_positions.push(Position(y, x + (i as isize)));
        }
        number_positions
    }

    fn get_neighbors(position: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let Position(y, x) = position;
        neighbors.push(Position(*y, x - 1));
        neighbors.push(Position(*y, x + 1));
        neighbors.push(Position(y + 1, x + 1));
        neighbors.push(Position(y + 1, *x));
        neighbors.push(Position(y + 1, x - 1));
        neighbors.push(Position(y - 1, x + 1));
        neighbors.push(Position(y - 1, *x));
        neighbors.push(Position(y - 1, x - 1));
        neighbors
    }
}

fn parse_board(input: &str) -> Board {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    let mut y = 0;
    let mut parsing_number = false;
    let mut number_starting_position = Position(0, 0);
    let mut number = String::new();

    for line in input.lines() {
        for (i, c) in line.trim().chars().enumerate() {
            match c {
                '.' => {
                    if parsing_number {
                        parsing_number = false;
                        numbers.push(Number {
                            pos: number_starting_position.clone(),
                            value: number.clone(),
                        });
                        number = String::new();
                    }
                }
                d @ '0'..='9' => {
                    if !parsing_number {
                        number_starting_position = Position(y, i.try_into().unwrap());
                    }
                    parsing_number = true;
                    number.push(d);
                }
                s => {
                    if parsing_number {
                        parsing_number = false;
                        numbers.push(Number {
                            pos: number_starting_position.clone(),
                            value: number,
                        });
                        number = String::new();
                    }
                    symbols.insert(Position(y, i.try_into().unwrap()), s);
                }
            }
        }

        if parsing_number {
            numbers.push(Number {
                pos: number_starting_position.clone(),
                value: number,
            });
        }

        parsing_number = false;
        number = String::new();
        y += 1;
    }

    Board {
        height: y.try_into().unwrap(),
        width: input.lines().next().unwrap().len(),
        numbers,
        symbols,
    }
}

pub fn part1(input: &str) -> String {
    let b = parse_board(input);
    let mut set = BTreeSet::new();
    for number in &b.numbers {
        if b.adjacent_to_symbol(number) {
            if !set.contains(&number.value) {
                dbg!(&number.value);
            }
            set.insert(&number.value);
        }
    }
    let result: usize = set.into_iter().map(|x| x.parse::<usize>().unwrap()).sum();
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

    const TEST_DATA: &str = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

    #[test]
    fn parse_board_test() {
        let b = parse_board(TEST_DATA);
        let top_left = Number {
            pos: Position(0, 0),
            value: String::from("467"),
        };
        let star_position = Position(1, 3);
        assert!(b.numbers.contains(&top_left));
        assert!(b.symbols.contains_key(&star_position));
        assert_eq!(&'*', b.symbols.get(&star_position).unwrap());
    }

    #[test]
    fn get_number_positions_test() {
        let b = parse_board(TEST_DATA);
        let number = Number {
            pos: Position(9, 5),
            value: "598".to_owned(),
        };
        let expected = vec![Position(9, 5), Position(9, 6), Position(9, 7)];
        assert_eq!(expected, Board::get_number_positions(&number));
    }

    #[test]
    fn get_neighbors_test() {
        let position = Position(4, 1);
        let expected = vec![
            Position(4, 0),
            Position(4, 2),
            Position(5, 2),
            Position(5, 1),
            Position(5, 0),
            Position(3, 2),
            Position(3, 1),
            Position(3, 0),
        ];
        assert_eq!(expected, Board::get_neighbors(&position));
    }

    #[test]
    fn adjacent_to_symbol_test_adjacent() {
        let b = parse_board(TEST_DATA);
        let number = Number {
            pos: Position(0, 0),
            value: "467".to_owned(),
        };
        assert!(b.adjacent_to_symbol(&number))
    }

    #[test]
    fn adjacent_to_symbol_test_not_adjacent() {
        let b = parse_board(TEST_DATA);
        let number = Number {
            pos: Position(0, 5),
            value: "114".to_owned(),
        };
        assert!(!b.adjacent_to_symbol(&number))
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("4361", part1(TEST_DATA));
    }

    #[test]
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("1", part1(TEST_DATA));
    }
}
