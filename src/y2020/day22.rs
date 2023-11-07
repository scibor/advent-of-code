use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
struct Player {
    deck: VecDeque<u8>,
}

impl Player {
    fn new(deck: VecDeque<u8>) -> Self {
        Player { deck }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RoundResult {
    StillPlaying,
    Player1Won,
    Player2Won,
}

#[derive(Debug, PartialEq)]
struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    fn new(player1: Player, player2: Player) -> Self {
        Game { player1, player2 }
    }

    fn play_round(&mut self) -> RoundResult {
        if self.player1.deck.is_empty() {
            return RoundResult::Player2Won;
        }

        if self.player2.deck.is_empty() {
            return RoundResult::Player1Won;
        }

        let player1_card = self.player1.deck.pop_front().unwrap();
        let player2_card = self.player2.deck.pop_front().unwrap();

        match player1_card.cmp(&player2_card) {
            std::cmp::Ordering::Less => {
                self.player2.deck.push_back(player2_card);
                self.player2.deck.push_back(player1_card);
            }
            std::cmp::Ordering::Greater => {
                self.player1.deck.push_back(player1_card);
                self.player1.deck.push_back(player2_card);
            }
            std::cmp::Ordering::Equal => todo!(),
        };
        RoundResult::StillPlaying
    }

    fn get_end_result(&self, result: RoundResult) -> usize {
        match result {
            RoundResult::Player1Won => Self::calculate_score(&self.player1.deck),
            RoundResult::Player2Won => Self::calculate_score(&self.player2.deck),
            RoundResult::StillPlaying => {
                panic!("Impossible state");
            }
        }
    }

    fn find_final_score(&mut self) -> usize {
        let mut game_state = RoundResult::StillPlaying;

        while game_state == RoundResult::StillPlaying {
            game_state = self.play_round();
        }
        self.get_end_result(game_state)
    }

    fn calculate_score(deck: &VecDeque<u8>) -> usize {
        let length = deck.len();
        deck.iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (length - i) * (*x as usize))
    }
}

fn parse_player_deck(input: &str) -> Player {
    let deck: VecDeque<u8> = input
        .lines()
        .skip(1)
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect();
    Player::new(deck)
}

fn parse_game_data(input: &str) -> Game {
    let mut split = input.split("\n\n");
    let player1 = parse_player_deck(split.next().unwrap());
    let player2 = parse_player_deck(split.next().unwrap());

    Game::new(player1, player2)
}

pub fn part1(input: &str) -> usize {
    let mut game = parse_game_data(input);
    game.find_final_score()
}

pub fn part2(_: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_CASE_INPUT: &str = "Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10";

    #[test]
    fn parse_player_info() {
        let input = "Player 1:
        9
        2
        6
        3
        1";

        let expected = Player::new(VecDeque::from([9, 2, 6, 3, 1]));
        assert_eq!(expected, parse_player_deck(input));
    }

    #[test]
    fn parse_data() {
        let expected = Game::new(
            Player::new(VecDeque::from([9, 2, 6, 3, 1])),
            Player::new(VecDeque::from([5, 8, 4, 7, 10])),
        );
        assert_eq!(expected, parse_game_data(TEST_CASE_INPUT));
    }

    #[test]
    fn play_one_round() {
        let mut game = Game::new(
            Player::new(VecDeque::from([9, 2, 6, 3, 1])),
            Player::new(VecDeque::from([5, 8, 4, 7, 10])),
        );

        let expected = Game::new(
            Player::new(VecDeque::from([2, 6, 3, 1, 9, 5])),
            Player::new(VecDeque::from([8, 4, 7, 10])),
        );

        let round_result = game.play_round();

        assert_eq!(expected, game);
        assert_eq!(RoundResult::StillPlaying, round_result);
    }

    #[test]
    fn play_three_rounds() {
        let mut game = Game::new(
            Player::new(VecDeque::from([9, 2, 6, 3, 1])),
            Player::new(VecDeque::from([5, 8, 4, 7, 10])),
        );

        let expected = Game::new(
            Player::new(VecDeque::from([3, 1, 9, 5, 6, 4])),
            Player::new(VecDeque::from([7, 10, 8, 2])),
        );

        let _ = game.play_round();
        let _ = game.play_round();
        let round_result = game.play_round();

        assert_eq!(expected, game);
        assert_eq!(RoundResult::StillPlaying, round_result);
    }

    #[test]
    fn player1_won() {
        let mut game = Game::new(
            Player::new(VecDeque::from([1])),
            Player::new(VecDeque::new()),
        );

        let round_result = game.play_round();
        assert_eq!(RoundResult::Player1Won, round_result);
    }

    #[test]
    fn player2_won() {
        let mut game = Game::new(
            Player::new(VecDeque::new()),
            Player::new(VecDeque::from([1])),
        );

        let round_result = game.play_round();
        assert_eq!(RoundResult::Player2Won, round_result);
    }

    #[test]
    fn calculate_score_from_deck() {
        let deck = VecDeque::from([3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(306, Game::calculate_score(&deck));
    }

    #[test]
    fn test_case_part1() {
        let mut game = parse_game_data(TEST_CASE_INPUT);
        let result = game.find_final_score();
        assert_eq!(306, result);
    }
}
