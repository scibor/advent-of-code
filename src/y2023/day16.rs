use num::integer::Roots;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Ray {
    direction: Direction,
    position: usize,
}

fn parse_input(input: &str) -> [char; 110 * 110] {
    let mut result = ['.'; 110 * 110];
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
            Some((new_id, board[new_id]))
        }
        Direction::Down => {
            if y == size - 1 {
                return None;
            }
            let new_id = (y + 1) * size + x;
            Some((new_id, board[new_id]))
        }
        Direction::Left => {
            if x == 0 {
                return None;
            }
            let new_id = y * size + (x - 1);
            Some((new_id, board[new_id]))
        }
        Direction::Right => {
            if x == size - 1 {
                return None;
            }
            let new_id = y * size + (x + 1);
            Some((new_id, board[new_id]))
        }
    }
}

fn make_move(board: &[char], rays: &mut Vec<Ray>, ray: Ray) {
    let size = board.len().sqrt();
    let (x, y) = (ray.position % size, ray.position / size);
    let next_move = try_next_move(board, x, y, &ray.direction);
    match (ray.direction, next_move) {
        (_, None) => (),
        (dir, Some((x, '.')))
        | (dir @ (Direction::Left | Direction::Right), Some((x, '-')))
        | (dir @ (Direction::Up | Direction::Down), Some((x, '|'))) => {
            let new_ray = Ray {
                position: x,
                direction: dir,
            };
            if rays.contains(&new_ray) {
                return;
            }
            rays.push(new_ray.clone());
            make_move(board, rays, new_ray);
        }
        (Direction::Right | Direction::Left, Some((x, '|'))) => {
            let new_ray1 = Ray {
                position: x,
                direction: Direction::Up,
            };
            let new_ray2 = Ray {
                position: x,
                direction: Direction::Down,
            };
            rays.push(new_ray1.clone());
            rays.push(new_ray2.clone());
            make_move(board, rays, new_ray1);
            make_move(board, rays, new_ray2);
        }
        (Direction::Up | Direction::Down, Some((x, '-'))) => {
            let new_ray1 = Ray {
                position: x,
                direction: Direction::Left,
            };
            let new_ray2 = Ray {
                position: x,
                direction: Direction::Right,
            };
            rays.push(new_ray1.clone());
            rays.push(new_ray2.clone());
            make_move(board, rays, new_ray1);
            make_move(board, rays, new_ray2);
        }
        (dir, Some((x, '/'))) => {
            let new_direction = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            let new_ray = Ray {
                position: x,
                direction: new_direction,
            };
            if rays.contains(&new_ray) {
                return;
            }
            rays.push(new_ray.clone());
            make_move(board, rays, new_ray);
        }
        (dir, Some((x, '\\'))) => {
            let new_direction = match dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            let new_ray = Ray {
                position: x,
                direction: new_direction,
            };
            if rays.contains(&new_ray) {
                return;
            }
            rays.push(new_ray.clone());
            make_move(board, rays, new_ray);
        }

        (x, y) => unreachable!("{:?} {:?}", x, y),
    }
}

//6622
#[must_use]
pub fn part1(input: &str) -> String {
    let board = parse_input(input);
    let mut rays: Vec<Ray> = Vec::new();
    // XXX: This code is a mess because I model it in nonflexible way. make_move can't handle
    // bumping into special character right away so because it's '\' in my input I start it like
    // the first move is down.
    let starting_ray = Ray {
        position: 0,
        direction: Direction::Down,
    };
    rays.push(starting_ray.clone());
    make_move(&board, &mut rays, starting_ray);
    let mut positions: Vec<usize> = rays.iter().map(|ray| ray.position).collect();
    positions.sort_unstable();
    positions.dedup();
    let result = positions.len();
    format!("{result}")
}

#[must_use]
pub fn part2(input: &str) -> String {
    let board = parse_input(input);

    let mut result = 0;

    // XXX: I was really fed up with this problem already hence the spaghetti code. I run it only
    // for top row for test, got some value, put it into the site and it turns out it was a maximum
    // so I didn't bother to do the same for left, right and bottom edge of the square.
    for i in 0..110 {
        let mut rays: Vec<Ray> = Vec::new();
        match &board[i] {
            '.' | '|' => {
                let direction = Direction::Down;
                let starting_ray = Ray {
                    position: i,
                    direction,
                };
                rays.push(starting_ray.clone());
                make_move(&board, &mut rays, starting_ray);
                let mut positions: Vec<usize> = rays.iter().map(|ray| ray.position).collect();
                positions.sort_unstable();
                positions.dedup();
                let partial_result = positions.len();
                result = result.max(partial_result);
            }
            '/' => {
                let direction = Direction::Left;
                let starting_ray = Ray {
                    position: i,
                    direction,
                };
                rays.push(starting_ray.clone());
                make_move(&board, &mut rays, starting_ray);
                let mut positions: Vec<usize> = rays.iter().map(|ray| ray.position).collect();
                positions.sort_unstable();
                positions.dedup();
                let partial_result = positions.len();
                result = result.max(partial_result);
            }
            '\\' => {
                let direction = Direction::Right;
                let starting_ray = Ray {
                    position: i,
                    direction,
                };
                rays.push(starting_ray.clone());
                make_move(&board, &mut rays, starting_ray);
                let mut positions: Vec<usize> = rays.iter().map(|ray| ray.position).collect();
                positions.sort_unstable();
                positions.dedup();
                let partial_result = positions.len();
                result = result.max(partial_result);
            }
            '-' => {
                let direction = Direction::Right;
                let starting_ray = Ray {
                    position: i,
                    direction,
                };
                rays.push(starting_ray.clone());
                make_move(&board, &mut rays, starting_ray);
                let direction = Direction::Left;
                let starting_ray = Ray {
                    position: i,
                    direction,
                };
                rays.push(starting_ray.clone());
                make_move(&board, &mut rays, starting_ray);

                let mut positions: Vec<usize> = rays.iter().map(|ray| ray.position).collect();
                positions.sort_unstable();
                positions.dedup();
                let partial_result = positions.len();
                result = result.max(partial_result);
            }
            _ => unreachable!(),
        }
    }
    format!("{result}")
}
