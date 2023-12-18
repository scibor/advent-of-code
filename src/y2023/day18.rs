use std::ops::AddAssign;

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
struct ColoredSegment {
    direction: Direction,
    length: u8,
    color: String,
}

#[derive(Debug, PartialEq)]
struct Polygon {
    vertices: Vec<Vec2>,
}

impl Polygon {
    // https://en.m.wikipedia.org/wiki/Shoelace_formula
    fn shoelace_area(&self) -> usize {
        let mut sum = 0;
        let mut counter_clockwise = self.vertices.clone();
        counter_clockwise.reverse();
        for window in counter_clockwise.windows(2) {
            sum += window[0].x * window[1].y;
            sum -= window[0].y * window[1].x;
        }
        (sum / 2).try_into().unwrap()
    }

    fn boundary_points(segments: &[ColoredSegment]) -> usize {
        let result: usize = segments.iter().map(|x| x.length as usize).sum();
        result
    }

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    fn internal_points(boundary: usize, area: usize) -> usize {
        area - boundary / 2 + 1
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_line(line: &str) -> ColoredSegment {
    let mut split = line.trim().split(' ');
    let direction = match split.next().unwrap() {
        "R" => Direction::Right,
        "U" => Direction::Up,
        "L" => Direction::Left,
        "D" => Direction::Down,
        _ => unreachable!(),
    };
    let length = split.next().unwrap().parse::<u8>().unwrap();
    let color = split.next().unwrap().replace("(#", "").replace(')', "");
    ColoredSegment {
        direction,
        length,
        color,
    }
}

fn parse_segments(input: &str) -> Vec<ColoredSegment> {
    input.lines().map(parse_line).collect()
}

fn parse_data(segments: &Vec<ColoredSegment>) -> Polygon {
    let mut currect_point = Vec2::new(0, 0);
    let mut vertices = vec![currect_point];
    for segment in segments {
        let change = match segment.direction {
            Direction::Up => Vec2::new(0, segment.length as isize),
            Direction::Down => Vec2::new(0, -(segment.length as isize)),
            Direction::Right => Vec2::new(segment.length as isize, 0),
            Direction::Left => Vec2::new(-(segment.length as isize), 0),
        };
        currect_point += change;
        vertices.push(currect_point);
    }
    Polygon { vertices }
}

// # Panics
#[must_use]
pub fn part1(input: &str) -> String {
    let segments = parse_segments(input);
    let polygon = parse_data(&segments);
    let area = polygon.shoelace_area();
    let boundary = Polygon::boundary_points(&segments);
    let result = boundary + Polygon::internal_points(boundary, area);
    format!("{result}")
}

fn parse_data_part2(segments: &Vec<ColoredSegment>) -> (Polygon, usize) {
    let mut currect_point = Vec2::new(0, 0);
    let mut vertices = vec![currect_point];
    let mut boundary_length = 0;
    for segment in segments {
        let length = isize::from_str_radix(&segment.color[0..5], 16).unwrap();
        boundary_length += length;
        let change = match segment.color.chars().nth(5).unwrap() {
            '3' => Vec2::new(0, length),
            '1' => Vec2::new(0, -length),
            '0' => Vec2::new(length, 0),
            '2' => Vec2::new(-length, 0),
            _ => unreachable!(),
        };
        currect_point += change;
        vertices.push(currect_point);
    }
    (Polygon { vertices }, boundary_length.try_into().unwrap())
}

// # Panics
#[must_use]
pub fn part2(input: &str) -> String {
    let segments = parse_segments(input);
    let (polygon, boundary) = parse_data_part2(&segments);
    let area = polygon.shoelace_area();
    let result = boundary + Polygon::internal_points(boundary, area);
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";

    #[test]
    fn test_case_part1() {
        assert_eq!("62", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("952408144115", part2(TEST_DATA));
    }
}
