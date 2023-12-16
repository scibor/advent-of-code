use num::integer::Roots;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Ray {
    direction: Direction,
    position: usize,
}

fn parse_input(input: &str) -> [char; 10 * 10] {
    let mut result = ['.'; 10 * 10];
    let mut id = 0;
    for line in input.lines() {
        for c in line.trim().chars() {
            if c != '.' {
                result[id] = c;
            }
            id += 1;
        }
    }
    result
}

fn print_board(board: &[char]) {
    let size = board.len().sqrt();
    for i in 0..size {
        for j in 0..size {
            print!("{}", board[i * size + j]);
        }
        println!()
    }
}

fn try_next_move(
    board: &[char],
    x: usize,
    y: usize,
    direction: &Direction,
) -> Option<(usize, char)> {
    let size = board.len().sqrt();
    match direction {
        Direction::Up => {
            if y == 0 {
                return None;
            }
            let new_id = (y - 1) * size + x;
            return Some((new_id, board[new_id]));
        }
        Direction::Down => {
            if y == size - 1 {
                return None;
            }
            let new_id = (y + 1) * size + x;
            return Some((new_id, board[new_id]));
        }
        Direction::Left => {
            if x == 0 {
                return None;
            }
            let new_id = y * size + (x - 1);
            return Some((new_id, board[new_id]));
        }
        Direction::Right => {
            if x == size - 1 {
                return None;
            }
            let new_id = y * size + (x + 1);
            return Some((new_id, board[new_id]));
        }
    }
}

fn make_move(board: &mut [char], ray: Ray) {
    let size = board.len().sqrt();
    let (x, y) = (ray.position % size, ray.position / size);
    let next_move = try_next_move(&board, x, y, &ray.direction);
    match (ray.direction, next_move) {
        (_, None) => {
            return;
        }
        (_, Some((_, '.'))) => todo!(),
        (_, _) => unreachable!(),
    }
}

pub fn part1(input: &str) -> String {
    let characters = parse_input(input);
    print_board(&characters);
    let result = 0;
    format!("{result}")
}

pub fn part2(_input: &str) -> String {
    let result = 0;
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";

    #[test]
    fn test_case_part1() {
        assert_eq!("46", part1(TEST_DATA));
    }

    #[test]
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("1", part1(TEST_DATA));
    }
}
