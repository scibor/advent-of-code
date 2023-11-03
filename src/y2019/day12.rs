use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Vector3(i64, i64, i64);

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn move_one_timestep(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
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

fn calculate_gravity_for_moon(moons: &[Moon], i: usize) -> Moon {
    let current_moon = &moons[i];
    let mut velocity = current_moon.velocity.clone();
    let other: Vec<Moon> = moons
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, x)| x)
        .cloned()
        .collect();

    for moon in other.iter() {
        // x
        match current_moon.position.0.cmp(&moon.position.0) {
            std::cmp::Ordering::Less => velocity.0 += 1,
            std::cmp::Ordering::Greater => velocity.0 -= 1,
            std::cmp::Ordering::Equal => {}
        }
        // y
        match current_moon.position.1.cmp(&moon.position.1) {
            std::cmp::Ordering::Less => velocity.1 += 1,
            std::cmp::Ordering::Greater => velocity.1 -= 1,
            std::cmp::Ordering::Equal => {}
        }
        // z
        match current_moon.position.2.cmp(&moon.position.2) {
            std::cmp::Ordering::Less => velocity.2 += 1,
            std::cmp::Ordering::Greater => velocity.2 -= 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    Moon {
        position: current_moon.position.clone(),
        velocity: velocity.clone(),
    }
}

fn calculate_gravity(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut result = Vec::new();
    for i in 0..moons.len() {
        let moon = calculate_gravity_for_moon(moons, i);
        result.push(moon);
    }
    result
}

fn energy_after_n_steps(moons: Vec<Moon>, n: usize) -> u64 {
    let mut current_moons: Vec<Moon> = moons;
    for _i in 0..n {
        current_moons = calculate_gravity(&current_moons);
        for moon in &mut current_moons {
            moon.move_one_timestep();
        }
    }
    current_moons.iter().fold(0, |acc, moon| {
        acc + (moon.kinetic_energy() as u64) * (moon.potential_energy() as u64)
    })
}

pub fn part1(input: &str) -> u64 {
    let moons: Vec<Moon> = input.lines().map(parse_row).collect();
    energy_after_n_steps(moons, 1000)
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    let mut bigger = x;
    let mut smaller = y;
    if smaller > bigger {
        std::mem::swap(&mut x, &mut y);
    }

    loop {
        let res = bigger % smaller;
        if res == 0 {
            return smaller;
        }

        bigger = smaller;
        smaller = res;
    }
}

fn find_cycle(moons: &[Moon]) -> u64 {
    let mut x_cycle = 0;
    let mut y_cycle = 0;
    let mut z_cycle = 0;

    let xs: Vec<i64> = moons.iter().map(|x| x.position.0).collect();
    let ys: Vec<i64> = moons.iter().map(|x| x.position.1).collect();
    let zs: Vec<i64> = moons.iter().map(|x| x.position.2).collect();

    let mut counter = 0;

    let mut current_moons: Vec<Moon> = moons.to_vec();
    while x_cycle * y_cycle * z_cycle == 0 {
        counter += 1;
        current_moons = calculate_gravity(&current_moons);
        for moon in &mut current_moons {
            moon.move_one_timestep();
        }
        if x_cycle == 0
            && current_moons
                .iter()
                .map(|x| x.position.0)
                .collect::<Vec<i64>>()
                == xs
        {
            x_cycle = counter + 1;
        }

        if y_cycle == 0
            && current_moons
                .iter()
                .map(|x| x.position.1)
                .collect::<Vec<i64>>()
                == ys
        {
            y_cycle = counter + 1;
        }

        if z_cycle == 0
            && current_moons
                .iter()
                .map(|x| x.position.2)
                .collect::<Vec<i64>>()
                == zs
        {
            z_cycle = counter + 1;
        }
    }
    lcm(lcm(x_cycle, y_cycle), z_cycle)
}

pub fn part2(input: &str) -> u64 {
    let moons: Vec<Moon> = input.lines().map(parse_row).collect();
    find_cycle(&moons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        let mut moon = Moon {
            position: Vector3(-8, -10, 0),
            velocity: Vector3(0, 0, 0),
        };
        moon.move_one_timestep();
        assert_eq!(moon.position, Vector3(-8, -10, 0));
        assert_eq!(moon.velocity, Vector3(0, 0, 0));
    }

    #[test]
    fn move_moon_with_velocity() {
        let mut moon = Moon {
            position: Vector3(-8, -10, 0),
            velocity: Vector3(1, -2, 3),
        };
        moon.move_one_timestep();
        assert_eq!(moon.position, Vector3(-7, -12, 3),);
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

    #[test]
    fn calculate_gravity_for_moon_test() {
        let moons: Vec<Moon> = TEST_CASE_INPUT1.lines().map(|x| parse_row(x)).collect();
        let expected = Moon {
            position: Vector3(-1, 0, 2),
            velocity: Vector3(3, -1, -1),
        };
        assert_eq!(expected, calculate_gravity_for_moon(&moons, 0));
    }

    #[test]
    fn calculate_gravity_test() {
        let moons: Vec<Moon> = TEST_CASE_INPUT1.lines().map(|x| parse_row(x)).collect();
        let expected = vec![
            Moon {
                position: Vector3(-1, 0, 2),
                velocity: Vector3(3, -1, -1),
            },
            Moon {
                position: Vector3(2, -10, -7),
                velocity: Vector3(1, 3, 3),
            },
            Moon {
                position: Vector3(4, -8, 8),
                velocity: Vector3(-3, 1, -3),
            },
            Moon {
                position: Vector3(3, 5, -1),
                velocity: Vector3(-1, -3, 1),
            },
        ];
        assert_eq!(expected, calculate_gravity(&moons));
    }

    #[test]
    fn test_case_1() {
        let moons: Vec<Moon> = TEST_CASE_INPUT1.lines().map(|x| parse_row(x)).collect();
        assert_eq!(179, energy_after_n_steps(moons, 10))
    }

    #[test]
    fn test_case_2() {
        let moons: Vec<Moon> = TEST_CASE_INPUT2.lines().map(|x| parse_row(x)).collect();
        assert_eq!(1940, energy_after_n_steps(moons, 100))
    }

    #[test]
    fn find_cycle_test1() {
        let moons: Vec<Moon> = TEST_CASE_INPUT1.lines().map(|x| parse_row(x)).collect();
        assert_eq!(2772, find_cycle(&moons))
    }

    #[test]
    fn find_cycle_test2() {
        let moons: Vec<Moon> = TEST_CASE_INPUT2.lines().map(|x| parse_row(x)).collect();
        assert_eq!(4686774924, find_cycle(&moons))
    }

    #[test]
    fn test_gcd() {
        assert_eq!(
            2 * 3 * 7 * 5 * 5,
            gcd(
                2 * 5 * 7 * 5 * 3 * 11 * 17,
                2 * 3 * 7 * 5 * 5 * 8 * 7 * 3 * 5 * 7
            )
        )
    }

    #[test]
    fn test_lcm() {
        assert_eq!(2 * 3 * 7 * 5 * 5, lcm(2 * 3 * 5, 7 * 5 * 5 * 3))
    }
}
