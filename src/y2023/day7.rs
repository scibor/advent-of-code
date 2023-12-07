use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandValue {
    FiveOfAKind = 700,
    FourOfAKind = 600,
    FullHouse = 500,
    ThreeOfAKind = 400,
    TwoPairs = 300,
    OnePair = 200,
    HighCard = 100,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: [Card; 5],
    value: HandValue,
    bid: usize,
}

fn parse_cards(input: &str) -> [Card; 5] {
    let mut result = [Card::A, Card::A, Card::A, Card::A, Card::A];
    for (i, c) in input.chars().enumerate() {
        let card = match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        };
        result[i] = card;
    }
    result
}

fn calculate_value(cards: &[Card]) -> HandValue {
    let mut number_of_cards = [0; 15];
    for card in cards {
        number_of_cards[*card as usize] += 1;
    }

    if number_of_cards.contains(&5) {
        return HandValue::FiveOfAKind;
    }

    if number_of_cards.contains(&4) {
        return HandValue::FourOfAKind;
    }

    if number_of_cards.contains(&3) && number_of_cards.contains(&2) {
        return HandValue::FullHouse;
    }

    if number_of_cards.contains(&3) {
        return HandValue::ThreeOfAKind;
    }

    if number_of_cards.iter().filter(|&&x| x == 2).count() == 2 {
        return HandValue::TwoPairs;
    }

    if number_of_cards.contains(&2) {
        return HandValue::OnePair;
    }
    HandValue::HighCard
}

impl Hand {
    fn parse_hand(input: &str) -> Hand {
        let mut split = input.trim().split(' ');
        let cards = parse_cards(split.next().unwrap());
        let value = calculate_value(&cards);
        let bid: usize = split.next().unwrap().parse().unwrap();
        Hand { cards, value, bid }
    }

    fn hand_sorter(hand1: &Self, hand2: &Self) -> Ordering {
        if let Some(o) = hand1.value.partial_cmp(&hand2.value) {
            if o != Ordering::Equal {
                return o;
            }
        }
        for i in 0..5 {
            if let Some(o) = hand1.cards[i].partial_cmp(&hand2.cards[i]) {
                if o != Ordering::Equal {
                    return o;
                }
            }
        }
        unreachable!()
    }
}

fn calculate_result(hands: &[Hand]) -> usize {
    let mut result = 0;
    for (i, h) in hands.iter().enumerate() {
        result += (i + 1) * h.bid;
    }
    result
}

pub fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse_hand).collect();
    hands.sort_by(Hand::hand_sorter);
    let result = calculate_result(&hands);
    format!("{result}")
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum CardPart2 {
    A = 14,
    K = 13,
    Q = 12,
    J = 0,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, PartialEq)]
struct HandPart2 {
    cards: [CardPart2; 5],
    value: HandValue,
    bid: usize,
}

fn parse_cards_part2(input: &str) -> [CardPart2; 5] {
    let mut result = [
        CardPart2::A,
        CardPart2::A,
        CardPart2::A,
        CardPart2::A,
        CardPart2::A,
    ];
    for (i, c) in input.chars().enumerate() {
        let card = match c {
            'A' => CardPart2::A,
            'K' => CardPart2::K,
            'Q' => CardPart2::Q,
            'J' => CardPart2::J,
            'T' => CardPart2::T,
            '9' => CardPart2::Nine,
            '8' => CardPart2::Eight,
            '7' => CardPart2::Seven,
            '6' => CardPart2::Six,
            '5' => CardPart2::Five,
            '4' => CardPart2::Four,
            '3' => CardPart2::Three,
            '2' => CardPart2::Two,
            _ => unreachable!(),
        };
        result[i] = card;
    }
    result
}

fn calculate_value_part2(cards: &[CardPart2]) -> HandValue {
    let mut number_of_cards = [0; 15];
    for card in cards {
        number_of_cards[*card as usize] += 1;
    }

    let jokers = &number_of_cards[0];

    if number_of_cards.contains(&5)
        || (number_of_cards[1..].contains(&4) && *jokers == 1)
        || (number_of_cards[1..].contains(&3) && *jokers == 2)
        || (number_of_cards[1..].contains(&2) && *jokers == 3)
        || *jokers == 4
    {
        return HandValue::FiveOfAKind;
    }

    if number_of_cards.contains(&4)
        || (number_of_cards[1..].contains(&3) && *jokers == 1)
        || (number_of_cards[1..].contains(&2) && *jokers == 2)
        || *jokers == 3
    {
        return HandValue::FourOfAKind;
    }

    if number_of_cards.contains(&3) && number_of_cards.contains(&2)
        || (number_of_cards[1..].iter().filter(|&&x| x == 2).count() == 2 && *jokers == 1)
    {
        return HandValue::FullHouse;
    }

    if number_of_cards.contains(&3)
        || (number_of_cards[1..].contains(&2) && *jokers == 1)
        || *jokers == 2
    {
        return HandValue::ThreeOfAKind;
    }

    if number_of_cards.iter().filter(|&&x| x == 2).count() == 2 {
        return HandValue::TwoPairs;
    }

    if number_of_cards.contains(&2) || *jokers == 1 {
        return HandValue::OnePair;
    }
    HandValue::HighCard
}

impl HandPart2 {
    fn parse_hand(input: &str) -> HandPart2 {
        let mut split = input.trim().split(' ');
        let cards = parse_cards_part2(split.next().unwrap());
        let value = calculate_value_part2(&cards);
        let bid: usize = split.next().unwrap().parse().unwrap();
        HandPart2 { cards, value, bid }
    }

    fn hand_sorter(hand1: &Self, hand2: &Self) -> Ordering {
        if let Some(o) = hand1.value.partial_cmp(&hand2.value) {
            if o != Ordering::Equal {
                return o;
            }
        }
        for i in 0..5 {
            if let Some(o) = hand1.cards[i].partial_cmp(&hand2.cards[i]) {
                if o != Ordering::Equal {
                    return o;
                }
            }
        }
        unreachable!()
    }
}

fn calculate_result_part2(hands: &[HandPart2]) -> usize {
    let mut result = 0;
    for (i, h) in hands.iter().enumerate() {
        result += (i + 1) * h.bid;
    }
    result
}

pub fn part2(input: &str) -> String {
    let mut hands: Vec<HandPart2> = input.lines().map(HandPart2::parse_hand).collect();
    hands.sort_by(HandPart2::hand_sorter);
    let result = calculate_result_part2(&hands);
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    #[test]
    fn card_comparinson_test() {
        assert!(Card::A > Card::Three);
    }

    #[test]
    fn hand_value_comparinson_test() {
        assert!(HandValue::FiveOfAKind > HandValue::TwoPairs);
    }

    #[test]
    fn parse_cards_test() {
        let input = "T55J5";
        let expected = [Card::T, Card::Five, Card::Five, Card::J, Card::Five];
        assert_eq!(expected, parse_cards(input));
    }

    #[test]
    fn parse_hand_test() {
        let input = "KTJJT 220";
        let expected = Hand {
            cards: [Card::K, Card::T, Card::J, Card::J, Card::T],
            value: HandValue::TwoPairs,
            bid: 220,
        };
        assert_eq!(expected, Hand::parse_hand(input))
    }

    #[test]
    fn calculate_value_test() {
        assert_eq!(
            HandValue::FiveOfAKind,
            calculate_value(&[Card::A, Card::A, Card::A, Card::A, Card::A])
        );
        assert_eq!(
            HandValue::FourOfAKind,
            calculate_value(&[Card::A, Card::A, Card::J, Card::A, Card::A])
        );
        assert_eq!(
            HandValue::ThreeOfAKind,
            calculate_value(&[Card::A, Card::Two, Card::J, Card::A, Card::A])
        );
        assert_eq!(
            HandValue::FullHouse,
            calculate_value(&[Card::A, Card::J, Card::J, Card::A, Card::A])
        );
        assert_eq!(
            HandValue::TwoPairs,
            calculate_value(&[Card::Two, Card::J, Card::J, Card::A, Card::A])
        );
        assert_eq!(
            HandValue::OnePair,
            calculate_value(&[Card::Two, Card::J, Card::J, Card::Eight, Card::A])
        );
        assert_eq!(
            HandValue::HighCard,
            calculate_value(&[Card::Two, Card::J, Card::T, Card::Eight, Card::A])
        );
    }

    #[test]
    fn hand_sorter_test() {
        let hand1 = Hand::parse_hand("TTKK2 100");
        let hand2 = Hand::parse_hand("TTKK3 100");
        assert_eq!(Ordering::Less, Hand::hand_sorter(&hand1, &hand2));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!("6440", part1(TEST_DATA));
    }

    #[test]
    fn calculate_value_test_part2() {
        assert_eq!(
            HandValue::FiveOfAKind,
            calculate_value_part2(&[
                CardPart2::A,
                CardPart2::A,
                CardPart2::A,
                CardPart2::A,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::FiveOfAKind,
            calculate_value_part2(&[
                CardPart2::A,
                CardPart2::A,
                CardPart2::J,
                CardPart2::A,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::FourOfAKind,
            calculate_value_part2(&[
                CardPart2::A,
                CardPart2::Two,
                CardPart2::J,
                CardPart2::A,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::FiveOfAKind,
            calculate_value_part2(&[
                CardPart2::A,
                CardPart2::J,
                CardPart2::J,
                CardPart2::A,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::FourOfAKind,
            calculate_value_part2(&[
                CardPart2::Two,
                CardPart2::J,
                CardPart2::J,
                CardPart2::A,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::ThreeOfAKind,
            calculate_value_part2(&[
                CardPart2::Two,
                CardPart2::J,
                CardPart2::J,
                CardPart2::Eight,
                CardPart2::A
            ])
        );
        assert_eq!(
            HandValue::OnePair,
            calculate_value_part2(&[
                CardPart2::Two,
                CardPart2::J,
                CardPart2::T,
                CardPart2::Eight,
                CardPart2::A
            ])
        );
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("5905", part2(TEST_DATA));
    }
}
