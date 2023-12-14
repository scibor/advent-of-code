use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    N,
    W,
    S,
    E,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let height = input.lines().count();
        let mut rows = vec![Vec::new(); height];
        let width = input.lines().next().unwrap().trim().len();
        let mut columns = vec![Vec::new(); width];
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                rows.get_mut(i).unwrap().push(c);
                columns.get_mut(j).unwrap().push(c);
            }
        }
        Grid { rows, columns }
    }

    fn move_rocks(&self, direction: &Direction) -> Grid {
        if *direction == Direction::N || *direction == Direction::S {
            let mut new_columns: Vec<Vec<char>> = Vec::new();
            for column in &self.columns {
                new_columns.push(Self::create_new_vector(column, direction));
            }

            let rows = Self::create_rows_from_columns(&new_columns);
            Grid {
                rows,
                columns: new_columns,
            }
        } else {
            let mut new_rows: Vec<Vec<char>> = Vec::new();
            for row in &self.rows {
                new_rows.push(Self::create_new_vector(row, direction));
            }

            let columns = Self::create_rows_from_columns(&new_rows);
            Grid {
                rows: new_rows,
                columns,
            }
        }
    }

    fn perform_one_cycle(&self) -> Grid {
        let mut grid = self.move_rocks(&Direction::N);
        grid = grid.move_rocks(&Direction::W);
        grid = grid.move_rocks(&Direction::S);
        grid.move_rocks(&Direction::E)
    }

    fn create_new_vector(column: &[char], direction: &Direction) -> Vec<char> {
        let mut input_vector = column.to_owned();
        if *direction == Direction::S || *direction == Direction::E {
            input_vector.reverse();
        }
        let mut last_block: Option<usize> = None;
        let mut last_rock_position: Option<usize> = None;
        let mut rocks_positions = Vec::new();
        let mut square_rock_positions = Vec::new();
        for (i, c) in input_vector.iter().enumerate() {
            match c {
                '#' => {
                    last_block = Some(i);
                    square_rock_positions.push(i);
                }
                'O' => match last_block {
                    None => match last_rock_position {
                        None => {
                            rocks_positions.push(0);
                            last_rock_position = Some(0);
                        }
                        Some(i) => {
                            rocks_positions.push(i + 1);
                            last_rock_position = Some(i + 1);
                        }
                    },
                    Some(i) => match last_rock_position {
                        None => {
                            rocks_positions.push(i + 1);
                            last_rock_position = Some(i + 1);
                        }
                        Some(j) => {
                            if j > i {
                                rocks_positions.push(j + 1);
                                last_rock_position = Some(j + 1);
                            } else {
                                rocks_positions.push(i + 1);
                                last_rock_position = Some(i + 1);
                            }
                        }
                    },
                },
                '.' => {}
                _ => unreachable!(),
            }
        }

        let mut result = Vec::new();
        for i in 0..column.len() {
            if rocks_positions.contains(&i) {
                result.push('O');
            } else if square_rock_positions.contains(&i) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        if *direction == Direction::S || *direction == Direction::E {
            result.reverse();
        }
        result
    }

    fn create_rows_from_columns(columns: &[Vec<char>]) -> Vec<Vec<char>> {
        let height = columns.len();
        let mut rows = vec![Vec::new(); height];

        for column in columns {
            for (i, c) in column.iter().enumerate() {
                rows.get_mut(i).unwrap().push(*c);
            }
        }
        rows
    }

    fn calculate_total_load(&self) -> usize {
        let mut rows_number = self.rows.len();
        let mut result = 0;
        for row in &self.rows {
            result += row.iter().filter(|&&c| c == 'O').count() * rows_number;
            rows_number -= 1;
        }
        result
    }
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    let new_grid = grid.move_rocks(&Direction::N);
    let result = new_grid.calculate_total_load();
    format!("{result}")
}

pub fn part2(input: &str) -> String {
    let mut grid = Grid::parse(input);
    let mut states = HashMap::new();
    let mut cycles = 0;

    while !states.contains_key(&grid) {
        states.insert(grid.clone(), cycles);
        grid = grid.perform_one_cycle();
        cycles += 1;
    }

    let start_of_cycle = states.get(&grid).unwrap();
    let cycle_length = cycles - start_of_cycle;

    let how_many_cycles = start_of_cycle + ((1_000_000_000 - start_of_cycle) % cycle_length);
    let mut grid = Grid::parse(input);

    for _i in 0..how_many_cycles {
        grid = grid.perform_one_cycle();
    }

    let result = grid.calculate_total_load();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";

    #[test]
    fn calculate_load_test() {
        let input = "OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....";
        let grid = Grid::parse(input);
        assert_eq!(136, grid.calculate_total_load());
    }

    #[test]
    fn create_new_column_test_n() {
        let column = ['.', 'O', 'O', '#', 'O', '.', '.', 'O', '#', 'O'];
        let expected = vec!['O', 'O', '.', '#', 'O', 'O', '.', '.', '#', 'O'];
        assert_eq!(expected, Grid::create_new_vector(&column, &Direction::N));
    }

    #[test]
    fn move_rocks_up_test() {
        let expected = "OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....";
        let grid = Grid::parse(TEST_DATA);
        let expected_grid = Grid::parse(expected);
        assert_eq!(expected_grid, grid.move_rocks(&Direction::N));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("136", part1(TEST_DATA));
    }

    #[test]
    fn create_new_column_test_s() {
        let column = ['.', 'O', 'O', '#', 'O', '.', '.', 'O', '#', 'O'];
        let expected = vec!['.', 'O', 'O', '#', '.', '.', 'O', 'O', '#', 'O'];
        assert_eq!(expected, Grid::create_new_vector(&column, &Direction::S));
    }

    #[test]
    fn perform_one_cycle_test() {
        let grid = Grid::parse(TEST_DATA);
        let expected = ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....";
        let expected_grid = Grid::parse(expected);
        assert_eq!(expected_grid, grid.perform_one_cycle());
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("64", part2(TEST_DATA));
    }
}
