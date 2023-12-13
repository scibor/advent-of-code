#[derive(Debug, PartialEq, Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
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

    fn find_value(&self) -> usize {
        let h_middle = self.horizontal_symmetry();
        if h_middle > 0 {
            h_middle
        } else {
            self.vertical_symmetry()
        }
    }

    fn vertical_symmetry(&self) -> usize {
        let mut i = 0;
        let number_of_columns = self.columns.len();
        'main_loop: for window in self.columns.windows(2) {
            if window[0] == window[1] {
                let mut lower = i as isize;
                let mut upper = i + 1;
                while lower >= 0 && upper < number_of_columns {
                    if self.columns[lower as usize] != self.columns[upper] {
                        i += 1;
                        continue 'main_loop;
                    }
                    lower -= 1;
                    upper += 1;
                }
                return i + 1;
            }
            i += 1;
        }
        0
    }

    fn horizontal_symmetry(&self) -> usize {
        let mut i = 0;
        let number_of_columns = self.rows.len();
        'main_loop: for window in self.rows.windows(2) {
            if window[0] == window[1] {
                let mut lower = i as isize;
                let mut upper = i + 1;
                while lower >= 0 && upper < number_of_columns {
                    if self.rows[lower as usize] != self.rows[upper] {
                        i += 1;
                        continue 'main_loop;
                    }
                    lower -= 1;
                    upper += 1;
                }
                return (i + 1) * 100;
            }
            i += 1;
        }
        0
    }

    fn find_alternative_value(&self) -> usize {
        let old_value = self.find_value();
        let width = self.rows.first().unwrap().len();
        let height = self.rows.len();

        for i in 0..height {
            for j in 0..width {
                let new_grid = self.create_copy(i, j);
                let new_value = new_grid.find_value_part2(old_value);
                if new_value != 0 {
                    return new_value;
                }
            }
        }

        unreachable!();
    }

    fn create_copy(&self, i: usize, j: usize) -> Self {
        let mut rows = self.rows.clone();
        let mut columns = self.columns.clone();

        let new_char = if rows.get(i).unwrap().get(j).unwrap() == &'.' {
            '#'
        } else {
            '.'
        };
        *rows.get_mut(i).unwrap().get_mut(j).unwrap() = new_char;
        *columns.get_mut(j).unwrap().get_mut(i).unwrap() = new_char;
        Grid { rows, columns }
    }

    fn find_value_part2(&self, old_value: usize) -> usize {
        let h_middle = self.horizontal_symmetry_part2(old_value);
        if h_middle > 0 {
            h_middle
        } else {
            self.vertical_symmetry_part2(old_value)
        }
    }

    fn vertical_symmetry_part2(&self, old_value: usize) -> usize {
        let mut i = 0;
        let number_of_columns = self.columns.len();
        'main_loop: for window in self.columns.windows(2) {
            if window[0] == window[1] {
                let mut lower = i as isize;
                let mut upper = i + 1;
                while lower >= 0 && upper < number_of_columns {
                    if self.columns[lower as usize] != self.columns[upper] {
                        i += 1;
                        continue 'main_loop;
                    }
                    lower -= 1;
                    upper += 1;
                }
                if i + 1 != old_value {
                    return i + 1;
                }
                i += 1;
                continue 'main_loop;
            }
            i += 1;
        }
        0
    }

    fn horizontal_symmetry_part2(&self, old_value: usize) -> usize {
        let mut i = 0;
        let number_of_columns = self.rows.len();
        'main_loop: for window in self.rows.windows(2) {
            if window[0] == window[1] {
                let mut lower = i as isize;
                let mut upper = i + 1;
                while lower >= 0 && upper < number_of_columns {
                    if self.rows[lower as usize] != self.rows[upper] {
                        i += 1;
                        continue 'main_loop;
                    }
                    lower -= 1;
                    upper += 1;
                }
                if (i + 1) * 100 != old_value {
                    return (i + 1) * 100;
                }
                i += 1;
                continue 'main_loop;
            }
            i += 1;
        }
        0
    }
}

pub fn part1(input: &str) -> String {
    let grids: Vec<Grid> = input.split("\n\n").map(Grid::parse).collect();
    let result: usize = grids.iter().map(Grid::find_value).sum();
    format!("{result}")
}

pub fn part2(input: &str) -> String {
    let grids: Vec<Grid> = input.split("\n\n").map(Grid::parse).collect();
    let result: usize = grids.iter().map(Grid::find_alternative_value).sum();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

    #[test]
    fn calculate_value_for_grid1() {
        let input = "#.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.";
        let grid = Grid::parse(input);
        assert_eq!(5, grid.find_value());
    }

    #[test]
    fn calculate_value_for_grid2() {
        let input = "#...##..#
                #....#..#
                ..##..###
                #####.##.
                #####.##.
                ..##..###
                #....#..#";
        let grid = Grid::parse(input);
        assert_eq!(400, grid.find_value());
    }

    #[test]
    fn calculate_value_for_grid3() {
        let input = "...#....
            ########
            ########
            ##......
            ########
            ########
            ##......
            ########
            ########
            ...#....";
        let grid = Grid::parse(input);
        assert_eq!(500, grid.find_value());
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("405", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("400", part2(TEST_DATA));
    }
}
