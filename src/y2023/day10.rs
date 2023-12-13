#[derive(Debug, PartialEq)]
enum Direction {
    W,
    E,
    N,
    S,
}

const N_LETTERS: [char; 3] = ['|', '7', 'F'];
const S_LETTERS: [char; 3] = ['|', 'L', 'J'];
const E_LETTERS: [char; 3] = ['-', 'J', '7'];
const W_LETTERS: [char; 3] = ['-', 'L', 'F'];

fn parse_data(input: &str) -> Vec<Vec<char>> {
    let width = input.lines().next().unwrap().len() + 2;
    let height = input.lines().count() + 2;
    let mut result = Vec::with_capacity(height);
    result.push(vec!['.'; width]);
    for line in input.lines() {
        let mut row = Vec::with_capacity(width);
        row.push('.');
        for c in line.trim().chars() {
            row.push(c);
        }
        row.push('.');
        result.push(row);
    }
    result.push(vec!['.'; width]);
    result
}

fn find_start(board: &[Vec<char>]) -> (usize, usize) {
    let y = board.iter().position(|r| r.contains(&'S')).unwrap();
    let x = board
        .get(y)
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();
    (x, y)
}

fn make_first_step(board: &[Vec<char>]) -> ((usize, usize), Direction) {
    let (start_x, start_y) = find_start(board);

    let up = board.get(start_y - 1).unwrap().get(start_x).unwrap();
    if N_LETTERS.contains(up) {
        return ((start_x, start_y - 1), Direction::N);
    }

    let down = board.get(start_y + 1).unwrap().get(start_x).unwrap();
    if S_LETTERS.contains(down) {
        return ((start_x, start_y + 1), Direction::S);
    }

    let left = board.get(start_y).unwrap().get(start_x - 1).unwrap();
    if S_LETTERS.contains(left) {
        return ((start_x - 1, start_y), Direction::W);
    }

    let right = board.get(start_y).unwrap().get(start_x + 1).unwrap();
    if S_LETTERS.contains(right) {
        return ((start_x + 1, start_y), Direction::E);
    }

    unreachable!()
}

fn make_step(
    board: &[Vec<char>],
    x: usize,
    y: usize,
    direction: &Direction,
) -> ((usize, usize), Direction) {
    match direction {
        Direction::N => {
            let new_x = x;
            let new_y = y - 1;
            let letter = board.get(new_y).unwrap().get(new_x).unwrap();
            let new_direction = match letter {
                '|' | 'S' => Direction::N,
                '7' => Direction::W,
                'F' => Direction::E,
                _ => unreachable!(),
            };
            ((new_x, new_y), new_direction)
        }
        Direction::S => {
            let new_x = x;
            let new_y = y + 1;
            let letter = board.get(new_y).unwrap().get(new_x).unwrap();
            let new_direction = match letter {
                '|' | 'S' => Direction::S,
                'L' => Direction::E,
                'J' => Direction::W,
                _ => unreachable!(),
            };
            ((new_x, new_y), new_direction)
        }
        Direction::W => {
            let new_x = x - 1;
            let new_y = y;
            let letter = board.get(new_y).unwrap().get(new_x).unwrap();
            let new_direction = match letter {
                '-' | 'S' => Direction::W,
                'L' => Direction::N,
                'F' => Direction::S,
                _ => unreachable!(),
            };
            ((new_x, new_y), new_direction)
        }
        Direction::E => {
            let new_x = x + 1;
            let new_y = y;
            let letter = board.get(new_y).unwrap().get(new_x).unwrap();
            let new_direction = match letter {
                '-' | 'S' => Direction::E,
                'J' => Direction::N,
                '7' => Direction::S,
                _ => unreachable!(),
            };
            ((new_x, new_y), new_direction)
        }
    }
}

pub fn part1(input: &str) -> String {
    let board = parse_data(input);
    let (start_x, start_y) = find_start(&board);
    let ((mut x, mut y), mut direction) = make_first_step(&board);

    let mut number_of_steps = 1;

    while (x, y) != (start_x, start_y) {
        ((x, y), direction) = make_step(&board, x, y, &direction);

        number_of_steps += 1;
    }

    let result = number_of_steps / 2;
    format!("{result}")
}

// XXX: After multiple failures with trying to use Jordan curve theorem I created programatically a
// bmp file of this network, filled it from the outside with color in MS Paint and programatically
// counted area that were inside. It requires manually doing something in MS Paint so I don't want
// to post this code in here. The trick to do it this this way is to not draw bmp file pixel by pixel but
// with 3x3 grids to allow spaces between pipes. I would gladly solve it the normal way after
// learning how to do this. Visualization for my input is in visualizations folder in repository.
pub fn part2(_input: &str) -> String {
    let result = 0;
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA1: &str = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";

    const TEST_DATA2: &str = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

    #[test]
    fn find_start_test1() {
        let expected = (2, 2);
        assert_eq!(expected, find_start(&parse_data(TEST_DATA1)));
    }

    #[test]
    fn find_start_test2() {
        let expected = (1, 3);
        assert_eq!(expected, find_start(&parse_data(TEST_DATA2)));
    }

    #[test]
    fn test_case_part1_2() {
        assert_eq!("8", part1(TEST_DATA2));
    }
}
