#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn count_winning_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn find_score(&self) -> usize {
        let count = self.count_winning_numbers();
        if count == 0 {
            return 0;
        }
        2_usize.pow((count - 1).try_into().unwrap())
    }
}

fn parse_row(row: &str) -> Card {
    let mut split_id = row.split(':');
    let id: usize = split_id
        .next()
        .unwrap()
        .replace("Card", "")
        .trim()
        .parse()
        .unwrap();
    let mut numbers_split = split_id.next().unwrap().split('|');
    let winning: Vec<usize> = numbers_split
        .next()
        .unwrap()
        .trim()
        .replace("  ", " ")
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let numbers: Vec<usize> = numbers_split
        .next()
        .unwrap()
        .trim()
        .replace("  ", " ")
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    Card {
        id,
        winning,
        numbers,
    }
}

#[must_use] pub fn part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|row| parse_row(row.trim()))
        .fold(0, |acc, x| acc + x.find_score());

    format!("{result}")
}

#[must_use] pub fn part2(input: &str) -> String {
    let cards: Vec<Card> = input.lines().map(|row| parse_row(row.trim())).collect();
    let mut number_of_cards = vec![1; cards.len()];

    let max = cards.len();

    for (i, card) in cards.into_iter().enumerate() {
        let winnings = card.count_winning_numbers();
        for j in 1..=winnings {
            if i + j <= max {
                number_of_cards[i + j] += number_of_cards[i];
            }
        }
    }
    let result: usize = number_of_cards.iter().sum();
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parse_row_test() {
        let row = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let expected = Card {
            id: 4,
            winning: vec![41, 92, 73, 84, 69],
            numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(expected, parse_row(row));
    }

    #[test]
    fn find_number_of_winning_cards() {
        let row = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        assert_eq!(1, parse_row(row).count_winning_numbers());
        let row = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(2, parse_row(row).count_winning_numbers());
    }

    #[test]
    fn find_score_test() {
        let row = "Card 4: 41 92 73 84 69 | 41 84 76 51 58 92 54 83";
        assert_eq!(4, parse_row(row).find_score());
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("13", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("30", part2(TEST_DATA));
    }
}
