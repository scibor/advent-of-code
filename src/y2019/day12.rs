use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Vector3(i64, i64, i64);

#[derive(Debug, PartialEq)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn move_one_timestep(&self) -> Self {
        Moon {
            position: Vector3(
                self.position.0 + self.velocity.0,
                self.position.1 + self.velocity.1,
                self.position.2 + self.velocity.2,
            ),
            velocity: self.velocity.clone(),
        }
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn potential_energy(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"<x=(?<x>-?[0-9]+), y=(?<y>-?[0-9]+), z=(?<z>-?[0-9]+)").unwrap();
}

fn parse_row(row: &str) -> Moon {
    let captures = RE.captures(row).unwrap();
    Moon {
        position: Vector3(
            captures["x"].parse::<i64>().unwrap(),
            captures["y"].parse::<i64>().unwrap(),
            captures["z"].parse::<i64>().unwrap(),
        ),
        velocity: Vector3(0, 0, 0),
    }
}

pub fn part1(_: &str) -> u64 {
    0
}
pub fn part2(_: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_INPUT1: &str = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    const TEST_CASE_INPUT2: &str = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    #[test]
    fn parse_row_test() {
        let row = "<x=-8, y=-10, z=0>";
        assert_eq!(
            Moon {
                position: Vector3(-8, -10, 0),
                velocity: Vector3(0, 0, 0)
            },
            parse_row(row)
        );
    }

    #[test]
    fn move_moon_without_velocity() {
        let moon = Moon {
            position: Vector3(-8, -10, 0),
            velocity: Vector3(0, 0, 0),
        };
        assert_eq!(moon, moon.move_one_timestep());
    }

    #[test]
    fn move_moon_with_velocity() {
        let moon = Moon {
            position: Vector3(-8, -10, 0),
            velocity: Vector3(1, -2, 3),
        };
        assert_eq!(
            Moon {
                position: Vector3(-7, -12, 3),
                velocity: Vector3(1, -2, 3),
            },
            moon.move_one_timestep()
        );
    }

    #[test]
    fn kinetic_energy_test() {
        let moon = Moon {
            position: Vector3(-8, -10, 0),
            velocity: Vector3(10, 5, -4),
        };
        assert_eq!(19, moon.kinetic_energy());
    }

    #[test]
    fn potential_energy_test() {
        let moon = Moon {
            position: Vector3(-8, 10, 0),
            velocity: Vector3(10, 5, -4),
        };
        assert_eq!(18, moon.potential_energy());
    }
}
