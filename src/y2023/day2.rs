use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_RED: Regex = Regex::new(r" (?<number>\d+) red").unwrap();
    static ref RE_BLUE: Regex = Regex::new(r" (?<number>\d+) blue").unwrap();
    static ref RE_GREEN: Regex = Regex::new(r" (?<number>\d+) green").unwrap();
}

#[derive(Debug, PartialEq)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug, PartialEq)]
struct ColoredCubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl ColoredCubes {
    fn power(&self) -> usize {
        self.green * self.red * self.blue
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    bag: Vec<ColoredCubes>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.bag
            .iter()
            .all(|bag| bag.red <= 12 && bag.green <= 13 && bag.blue <= 14)
    }

    fn find_minimal(&self) -> ColoredCubes {
        let green = self.bag.iter().map(|round| round.green).max().unwrap();
        let red = self.bag.iter().map(|round| round.red).max().unwrap();
        let blue = self.bag.iter().map(|round| round.blue).max().unwrap();
        ColoredCubes { red, green, blue }
    }
}

fn parse_row(row: &str) -> Game {
    let mut split_id = row.split(':');
    let id: usize = split_id
        .next()
        .unwrap()
        .replace("Game ", "")
        .parse()
        .unwrap();
    let split_rounds = split_id.next().unwrap().split(';');
    let mut bag: Vec<ColoredCubes> = vec![];
    for x in split_rounds {
        let green = match RE_GREEN.captures(x) {
            Some(c) => c.name("number").unwrap().as_str().parse::<usize>().unwrap(),
            None => 0,
        };

        let red = match RE_RED.captures(x) {
            Some(c) => c.name("number").unwrap().as_str().parse::<usize>().unwrap(),
            None => 0,
        };

        let blue = match RE_BLUE.captures(x) {
            Some(c) => c.name("number").unwrap().as_str().parse::<usize>().unwrap(),
            None => 0,
        };
        bag.push(ColoredCubes { red, green, blue });
    }
    Game { id, bag }
}

pub fn part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|row| parse_row(row.trim()))
        .filter(Game::is_possible)
        .map(|g| g.id)
        .sum();
    format!("{result}")
}

pub fn part2(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|row| parse_row(row.trim()).find_minimal().power())
        .sum();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn parse_row_test() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let expected = Game {
            id: 3,
            bag: vec![
                ColoredCubes {
                    green: 8,
                    blue: 6,
                    red: 20,
                },
                ColoredCubes {
                    blue: 5,
                    red: 4,
                    green: 13,
                },
                ColoredCubes {
                    green: 5,
                    red: 1,
                    blue: 0,
                },
            ],
        };
        assert_eq!(expected, parse_row(input));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("8", part1(TEST_DATA));
    }

    #[test]
    fn find_minimal_cube() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_row(input).find_minimal();
        let expected = ColoredCubes {
            green: 2,
            blue: 6,
            red: 4,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn find_cubes_power() {
        let cube = ColoredCubes {
            green: 2,
            blue: 6,
            red: 4,
        };
        assert_eq!(48, cube.power());
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("2286", part2(TEST_DATA));
    }
}
