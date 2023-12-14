#[derive(Debug, PartialEq, Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
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

    fn move_rocks_up(&self) -> Grid {
        let mut new_columns: Vec<Vec<char>> = Vec::new();
        for column in &self.columns {
            new_columns.push(Self::create_new_column(column));
        }

        let rows = Self::create_rows_from_columns(&new_columns);
        Grid {
            rows,
            columns: new_columns,
        }
    }

    fn create_new_column(column: &[char]) -> Vec<char> {
        let mut last_block: Option<usize> = None;
        let mut last_rock_position: Option<usize> = None;
        let mut rocks_positions = Vec::new();
        let mut square_rock_positions = Vec::new();
        for (i, c) in column.iter().enumerate() {
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
    let new_grid = grid.move_rocks_up();
    let result = new_grid.calculate_total_load();
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
    fn create_new_column_test() {
        let column = ['.', 'O', 'O', '#', 'O', '.', '.', 'O', '#', 'O'];
        let expected = vec!['O', 'O', '.', '#', 'O', 'O', '.', '.', '#', 'O'];
        assert_eq!(expected, Grid::create_new_column(&column));
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
        assert_eq!(expected_grid, grid.move_rocks_up());
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("136", part1(TEST_DATA));
    }

    #[test]
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("1", part1(TEST_DATA));
    }
}
