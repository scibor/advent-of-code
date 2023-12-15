#[derive(Debug, PartialEq)]
struct Galaxy {
    stars: Vec<(usize, usize)>,
    empty_horizontal: Vec<usize>,
    empty_vertical: Vec<usize>,
}

impl Galaxy {
    fn parse(input: &str) -> Self {
        let mut stars = Vec::new();
        let width = input.lines().next().unwrap().len();
        let mut empty_horizontal = Vec::new();
        let mut is_line_empty = true;
        let mut empty_columns = vec![true; width];

        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    stars.push((i, j));
                    is_line_empty = false;
                    if let Some(e) = empty_columns.get_mut(i) {
                        *e = false;
                    }
                }
            }
            if is_line_empty {
                empty_horizontal.push(j);
            }
            is_line_empty = true;
        }

        let mut empty_vertical = Vec::new();
        for (i, b) in empty_columns.iter().enumerate() {
            if *b {
                empty_vertical.push(i);
            }
        }
        Galaxy {
            stars,
            empty_horizontal,
            empty_vertical,
        }
    }

    fn find_distances(&self, expansion_rate: usize) -> usize {
        let number_of_stars = self.stars.len();
        let mut result = 0;

        for i in 0..number_of_stars {
            for j in i + 1..number_of_stars {
                let star1 = self.stars.get(i).unwrap();
                let star2 = self.stars.get(j).unwrap();
                result += self.distance(*star1, *star2, expansion_rate);
            }
        }

        result
    }

    fn distance(
        &self,
        star1: (usize, usize),
        star2: (usize, usize),
        expansion_rate: usize,
    ) -> usize {
        let (min_x, max_x) = if star1.0 < star2.0 {
            (star1.0, star2.0)
        } else {
            (star2.0, star1.0)
        };
        let (min_y, max_y) = if star1.1 < star2.1 {
            (star1.1, star2.1)
        } else {
            (star2.1, star1.1)
        };

        star1.0.abs_diff(star2.0)
            + star1.1.abs_diff(star2.1)
            + self
                .empty_horizontal
                .iter()
                .filter(|&&x| x < max_y && x > min_y)
                .count()
                * expansion_rate
            + self
                .empty_vertical
                .iter()
                .filter(|&&x| x < max_x && x > min_x)
                .count()
                * expansion_rate
    }
}

#[must_use] pub fn part1(input: &str) -> String {
    let galaxy = Galaxy::parse(input);
    let result = galaxy.find_distances(1);
    format!("{result}")
}

#[must_use] pub fn part2(input: &str) -> String {
    let galaxy = Galaxy::parse(input);
    let result = galaxy.find_distances(999_999);
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

    #[test]
    fn parse_galaxy_test() {
        let stars = vec![
            (3, 0),
            (7, 1),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9),
        ];
        let empty_horizontal = vec![3, 7];
        let empty_vertical = vec![2, 5, 8];
        let expected = Galaxy {
            stars,
            empty_horizontal,
            empty_vertical,
        };
        assert_eq!(expected, Galaxy::parse(TEST_DATA));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("374", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2_1() {
        let galaxy = Galaxy::parse(TEST_DATA);
        let result = galaxy.find_distances(9);
        assert_eq!(1030, result);
    }

    #[test]
    fn test_case_part2_2() {
        let galaxy = Galaxy::parse(TEST_DATA);
        let result = galaxy.find_distances(99);
        assert_eq!(8410, result);
    }
}
