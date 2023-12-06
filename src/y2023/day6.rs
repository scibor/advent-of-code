#[derive(Debug, PartialEq)]
struct TimeDistance {
    time: usize,
    distance: usize,
}

impl TimeDistance {
    fn count_ways_to_win(&self) -> usize {
        let mut i = 1;
        while i * (self.time - i) <= self.distance {
            i += 1;
        }
        self.time - 2 * i + 1
    }
}

fn parse_data(input: &str) -> Vec<TimeDistance> {
    let mut lines = input.lines();
    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut result = Vec::new();
    for (i, x) in times.iter().enumerate() {
        result.push(TimeDistance {
            time: *x,
            distance: *distances.get(i).unwrap(),
        });
    }
    result
}

pub fn part1(input: &str) -> String {
    let data = parse_data(input);
    let result: usize = data
        .iter()
        .map(TimeDistance::count_ways_to_win)
        //.fold(1, |acc, x| acc * x);
        .product();
    format!("{result}")
}

fn parse_data_part2(input: &str) -> TimeDistance {
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance: usize = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    TimeDistance { time, distance }
}

pub fn part2(input: &str) -> String {
    let data = parse_data_part2(input);
    let result = data.count_ways_to_win();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "Time:      7  15   30
        Distance:  9  40  200";

    #[test]
    fn parse_data_test() {
        let expected = vec![
            TimeDistance {
                time: 7,
                distance: 9,
            },
            TimeDistance {
                time: 15,
                distance: 40,
            },
            TimeDistance {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(expected, parse_data(TEST_DATA));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("288", part1(TEST_DATA));
    }

    #[test]
    fn parse_data_part2_test() {
        let expected = TimeDistance {
            time: 71530,
            distance: 940200,
        };
        assert_eq!(expected, parse_data_part2(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("71503", part2(TEST_DATA));
    }
}
