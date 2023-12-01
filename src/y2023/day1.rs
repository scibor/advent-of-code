fn get_calibration_value(s: &str) -> usize {
    let mut first = ' ';
    let mut last = ' ';
    for c in s.chars() {
        if c.is_ascii_digit() {
            last = c;
            if first == ' ' {
                first = c;
            }
        }
    }
    format!("{first}{last}").parse().unwrap()
}

pub fn part1(input: &str) -> String {
    let result: usize = input.lines().map(|l| get_calibration_value(l.trim())).sum();
    format!("{result}")
}

fn get_calibration_value2(s: &str, digit_map: &[(&str, char)]) -> usize {
    let mut first = ' ';
    let mut last = ' ';
    for (i, c) in s.chars().enumerate() {
        if let Some(x) = starts_with_digit(&s[i..], digit_map) {
            last = x.1;
            if first == ' ' {
                first = x.1;
            }
        }
        if c.is_ascii_digit() {
            last = c;
            if first == ' ' {
                first = c;
            }
        }
    }
    format!("{first}{last}").parse().unwrap()
}

fn starts_with_digit<'a>(
    s: &'a str,
    digit_map: &'a [(&'a str, char)],
) -> Option<&'a (&'a str, char)> {
    digit_map.iter().find(|(k, _)| s.starts_with(k))
}

pub fn part2(input: &str) -> String {
    let digit_map = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let result: usize = input
        .lines()
        .map(|l| get_calibration_value2(l.trim(), &digit_map))
        .sum();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    const TEST_DATA2: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    const DIGIT_MAP: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    #[test]
    fn calibration_value_when_1_digit() {
        let x = "treb7uchet";
        assert_eq!(77, get_calibration_value(x));
    }

    #[test]
    fn calibration_value_when_many_digits() {
        let x = "a1b2c3d4e5f";
        assert_eq!(15, get_calibration_value(x));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("142", part1(TEST_DATA));
    }

    #[test]
    fn starts_with_spelled_digit() {
        let x = "one1osidjgosij";
        assert_eq!(Some(&("one", '1')), starts_with_digit(x, &DIGIT_MAP));
    }

    #[test]
    fn starts_with_regular_digit() {
        let x = "1one1osidjgosij";
        assert_eq!(None, starts_with_digit(x, &DIGIT_MAP));
    }

    #[test]
    fn calibration_value_when_1_digit_v2() {
        let x = "treb7uchet";
        assert_eq!(77, get_calibration_value2(x, &DIGIT_MAP));
    }

    #[test]
    fn calibration_value_when_many_digits_v2() {
        let x = "a1b2c3d4e5f";
        assert_eq!(15, get_calibration_value2(x, &DIGIT_MAP));
    }

    #[test]
    fn calibration_value_v2_with_spelled_digit() {
        let x = "abcone2threexyz";
        assert_eq!(13, get_calibration_value2(x, &DIGIT_MAP));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("281", part2(TEST_DATA2));
    }
}
