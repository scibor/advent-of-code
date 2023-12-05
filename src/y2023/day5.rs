#[derive(Debug, PartialEq)]
struct Range(isize, isize, isize);

impl Range {
    fn from_str(input: &str) -> Range {
        let mut split = input.trim().split(' ');
        let x = split.next().unwrap().parse::<isize>().unwrap();
        let y = split.next().unwrap().parse::<isize>().unwrap();
        let z = split.next().unwrap().parse::<isize>().unwrap();
        Range(x, y, z)
    }
}

fn parse_seeds(input: &str) -> Vec<isize> {
    input
        .lines()
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .trim()
        .split(' ')
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect()
}

fn get_next_value(v: isize, next_part: &str) -> isize {
    let ranges: Vec<Range> = next_part.lines().skip(1).map(Range::from_str).collect();
    for range in ranges {
        if v >= range.1 && v <= range.1 + range.2 {
            return v + (range.0 - range.1);
        }
    }
    v
}

pub fn part1(input: &str) -> String {
    let seeds = parse_seeds(input);
    let parts: Vec<&str> = input.split("\n\n").skip(1).collect();
    let mut seed_final_values: Vec<isize> = Vec::new();
    for seed in seeds {
        let mut current_value = seed;
        for part in &parts {
            current_value = get_next_value(current_value, part);
        }
        seed_final_values.push(current_value);
    }
    let result = &seed_final_values.iter().min().unwrap();
    format!("{result}")
}

#[derive(Debug, PartialEq, Clone)]
struct Interval {
    from: isize,
    to: isize,
}

#[derive(Debug, PartialEq)]
struct MappingInterval {
    from: isize,
    to: isize,
    change: isize,
}

impl MappingInterval {
    fn from_str(input: &str) -> MappingInterval {
        let mut split = input.trim().split(' ');
        let x = split.next().unwrap().parse::<isize>().unwrap();
        let y = split.next().unwrap().parse::<isize>().unwrap();
        let z = split.next().unwrap().parse::<isize>().unwrap();
        MappingInterval {
            from: y,
            to: y + z - 1,
            change: x - y,
        }
    }
}

fn parse_seeds_part2(input: &str) -> Vec<Interval> {
    let temp = input.lines().next().unwrap().replace("seeds: ", "");
    let mut numbers = temp.trim().split(' ');
    let mut result = Vec::new();
    while let Some(f) = numbers.next() {
        let from: isize = f.parse().unwrap();
        let to = from + numbers.next().unwrap().parse::<isize>().unwrap() - 1;
        result.push(Interval { from, to });
    }
    result
}

fn parse_mapping_intervals(part: &str) -> Vec<MappingInterval> {
    part.lines()
        .skip(1)
        .map(MappingInterval::from_str)
        .collect()
}

fn is_between(x: isize, a: isize, b: isize) -> bool {
    a <= x && x <= b
}

fn have_common_elements(i: &Interval, mi: &MappingInterval) -> bool {
    is_between(i.from, mi.from, mi.to) || is_between(i.to, mi.from, mi.to)
}

fn split_interval(i: &Interval, mi: &MappingInterval) -> (Interval, Vec<Interval>) {
    // i in mi or equal
    if is_between(i.from, mi.from, mi.to) && is_between(i.to, mi.from, mi.to) {
        return (
            Interval {
                from: i.from + mi.change,
                to: i.to + mi.change,
            },
            Vec::new(),
        );
    }
    // mi inside i
    if is_between(mi.from, i.from, i.to) && is_between(mi.to, i.from, i.to) {
        if mi.to == i.to {
            return (
                Interval {
                    from: mi.from,
                    to: mi.to,
                },
                vec![Interval {
                    from: i.from,
                    to: mi.from - 1,
                }],
            );
        }
        if mi.from == i.from {
            return (
                Interval {
                    from: mi.from,
                    to: mi.to,
                },
                vec![Interval {
                    from: mi.to + 1,
                    to: i.to,
                }],
            );
        }
        return (
            Interval {
                from: mi.from,
                to: mi.to,
            },
            vec![
                Interval {
                    from: i.from,
                    to: mi.from - 1,
                },
                Interval {
                    from: mi.to + 1,
                    to: i.to,
                },
            ],
        );
    }
    // mi left to i with overlap
    if mi.from < i.from && is_between(mi.to, i.from, i.to) {
        return (
            Interval {
                from: i.from + mi.change,
                to: mi.to + mi.change,
            },
            vec![Interval {
                from: mi.to + 1,
                to: i.to,
            }],
        );
    }
    // mi right to i with overlap
    if mi.to > i.to && is_between(mi.from, i.from, i.to) {
        return (
            Interval {
                from: mi.from + mi.change,
                to: i.to + mi.change,
            },
            vec![Interval {
                from: i.from,
                to: mi.from - 1,
            }],
        );
    }
    unreachable!()
}

fn get_next_intervals(
    intervals: &[Interval],
    mapping_intervals: &[MappingInterval],
) -> Vec<Interval> {
    let mut new_intervals = Vec::new();
    let mut left_intervals = intervals.to_owned();
    while !left_intervals.is_empty() {
        //let i = left_intervals.pop().unwrap();
        while let Some(i) = left_intervals.pop() {
            if mapping_intervals
                .iter()
                .any(|x| have_common_elements(&i, x))
            {
                let mi = mapping_intervals
                    .iter()
                    .find(|x| have_common_elements(&i, x))
                    .unwrap();
                let (common, mut left) = split_interval(&i, mi);
                new_intervals.push(common);
                left_intervals.append(&mut left);
                continue;
            }
            new_intervals.push(i);
        }
    }
    new_intervals
}

fn get_intervals_minimum(intervals: &[Interval]) -> isize {
    intervals
        .iter()
        .map(|interval| interval.from)
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> String {
    let seeds = parse_seeds_part2(input);
    let parts: Vec<&str> = input.split("\n\n").skip(1).collect();
    let mut minimums: Vec<isize> = Vec::new();
    for seed in seeds {
        let mut current_intervals = vec![seed];
        for part in &parts {
            let mapping_intervals = parse_mapping_intervals(part);
            current_intervals = get_next_intervals(&current_intervals, &mapping_intervals);
        }
        minimums.push(get_intervals_minimum(&current_intervals));
    }
    let result = &minimums.iter().min().unwrap();

    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

    #[test]
    fn parse_seeds_test() {
        assert_eq!(vec![79, 14, 55, 13], parse_seeds(TEST_DATA));
    }

    #[test]
    fn get_next_value_test() {
        let next_part = "seed-to-soil map:
        50 98 2
        52 50 48";
        assert_eq!(81, get_next_value(79, next_part));
        assert_eq!(1, get_next_value(1, next_part));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("35", part1(TEST_DATA));
    }

    #[test]
    fn parse_seeds_part2_test() {
        assert_eq!(
            vec![Interval { from: 79, to: 92 }, Interval { from: 55, to: 67 }],
            parse_seeds_part2(TEST_DATA)
        );
    }

    #[test]
    fn parsing_mapping_interval() {
        let input = "88 18 7";
        let expected = MappingInterval {
            from: 18,
            to: 24,
            change: 70,
        };
        assert_eq!(expected, MappingInterval::from_str(input));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("46", part2(TEST_DATA));
    }
}
