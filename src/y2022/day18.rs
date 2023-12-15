use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point3D(usize, usize, usize);

impl From<&str> for Point3D {
    fn from(input: &str) -> Self {
        let mut split = input.trim().split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        let z = split.next().unwrap().parse::<usize>().unwrap();
        Point3D(x, y, z)
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Side(Point3D, Point3D);

struct Cube {
    origin: Point3D,
    sides: [Side; 6],
}

impl Cube {
    fn new(origin: Point3D) -> Self {
        let sides = Self::generate_sides(&origin);
        Cube { origin, sides }
    }

    fn generate_sides(origin: &Point3D) -> [Side; 6] {
        let &Point3D(x, y, z) = origin;
        [
            Side(Point3D(x, y, z), Point3D(x, y + 1, z + 1)),
            Side(Point3D(x, y, z), Point3D(x + 1, y, z + 1)),
            Side(Point3D(x, y, z), Point3D(x + 1, y + 1, z)),
            Side(Point3D(x + 1, y, z), Point3D(x + 1, y + 1, z + 1)),
            Side(Point3D(x, y + 1, z), Point3D(x + 1, y + 1, z + 1)),
            Side(Point3D(x, y, z + 1), Point3D(x + 1, y + 1, z + 1)),
        ]
    }
}

fn find_outer_sides(cubes: Vec<Cube>) -> usize {
    let mut histogram = HashMap::new();
    let sides: Vec<Side> = cubes.into_iter().flat_map(|c| c.sides).collect();
    for side in sides {
        let counter = histogram.entry(side).or_insert(0);
        *counter += 1;
    }
    histogram.into_iter().filter(|(_, v)| *v == 1).count()
}

#[must_use] pub fn part1(input: &str) -> usize {
    let cubes: Vec<Cube> = input.lines().map(|l| Cube::new(Point3D::from(l))).collect();
    find_outer_sides(cubes)
}

#[must_use] pub fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5";

    #[test]
    fn parse_point_test() {
        let input = "2,2,2";
        let expected = Point3D(2, 2, 2);
        assert_eq!(expected, Point3D::from(input));
    }

    #[test]
    fn create_sides_test() {
        let origin = Point3D(1, 1, 1);
        let expected = [
            Side(Point3D(1, 1, 1), Point3D(1, 2, 2)),
            Side(Point3D(1, 1, 1), Point3D(2, 1, 2)),
            Side(Point3D(1, 1, 1), Point3D(2, 2, 1)),
            Side(Point3D(2, 1, 1), Point3D(2, 2, 2)),
            Side(Point3D(1, 2, 1), Point3D(2, 2, 2)),
            Side(Point3D(1, 1, 2), Point3D(2, 2, 2)),
        ];
        assert_eq!(expected, Cube::generate_sides(&origin));
    }

    #[test]
    fn parse_cube_test() {
        let origin = Point3D(1, 1, 1);
        let expected = [
            Side(Point3D(1, 1, 1), Point3D(1, 2, 2)),
            Side(Point3D(1, 1, 1), Point3D(2, 1, 2)),
            Side(Point3D(1, 1, 1), Point3D(2, 2, 1)),
            Side(Point3D(2, 1, 1), Point3D(2, 2, 2)),
            Side(Point3D(1, 2, 1), Point3D(2, 2, 2)),
            Side(Point3D(1, 1, 2), Point3D(2, 2, 2)),
        ];
        let cube = Cube::new(origin);
        assert_eq!(expected, cube.sides);
    }

    #[test]
    fn test_case_part1() {
        let cubes: Vec<Cube> = TEST_DATA
            .lines()
            .map(|l| Cube::new(Point3D::from(l)))
            .collect();

        let result = find_outer_sides(cubes);
        assert_eq!(64, result);
    }
}
