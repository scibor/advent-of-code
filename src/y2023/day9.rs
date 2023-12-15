fn is_list_constant(numbers: &[isize]) -> bool {
    let first_value = numbers.first().unwrap();
    numbers.iter().skip(1).all(|x| x == first_value)
}

fn process_number_list(numbers: &[isize], forward: bool) -> isize {
    let mut lists: Vec<Vec<isize>> = Vec::new();
    let mut current_list = numbers.to_vec().clone();
    lists.push((numbers).to_vec());

    while !is_list_constant(&current_list) {
        let new_vec: Vec<isize> = current_list
            .windows(2)
            .map(|window| window.get(1).unwrap() - window.first().unwrap())
            .collect();
        lists.push(new_vec.clone());
        current_list = new_vec;
    }

    let mut new_value = 0;
    for list in lists.iter().rev() {
        if forward {
            new_value += list.last().unwrap();
        } else {
            new_value = list.first().unwrap() - new_value;
        }
    }

    new_value
}
/// # Panics
#[must_use]
pub fn part1(input: &str) -> String {
    let number_lines: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let result: isize = number_lines
        .iter()
        .map(|line| process_number_list(line, true))
        .sum();
    format!("{result}")
}

/// # Panics
#[must_use]
pub fn part2(input: &str) -> String {
    let number_lines: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let result: isize = number_lines
        .iter()
        .map(|line| process_number_list(line, false))
        .sum();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

    #[test]
    fn test_case_part1() {
        assert_eq!("114", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!(5, process_number_list(&vec![10, 13, 16, 21, 30, 45], false));
    }
}
