use lazy_static::lazy_static;
use regex::Regex;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector3(isize, isize, isize);

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector3(x1, y1, z1) = self;
        let Vector3(x2, y2, z2) = rhs;
        Vector3(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        let Vector3(x, y, z) = rhs;
        self.0 += x;
        self.1 += y;
        self.2 += z;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    GettingCloser,
    FlyingAway,
}

#[derive(Debug, PartialEq)]
struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    id: usize,
    moving_direction: Direction,
    rate_of_position_change: isize,
}

impl Particle {
    fn new(id: usize, position: Vector3, velocity: Vector3, acceleration: Vector3) -> Self {
        Particle {
            id,
            position,
            velocity,
            acceleration,
            moving_direction: Direction::GettingCloser,
            rate_of_position_change: 0,
        }
    }

    fn distance_from_origin(&self) -> isize {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn move_particle(&mut self) {
        let old_position = self.distance_from_origin();
        self.velocity += self.acceleration;
        self.position += self.velocity;
        let new_position = self.distance_from_origin();

        if new_position < old_position {
            self.moving_direction = Direction::GettingCloser;
            self.rate_of_position_change = old_position - new_position;
        } else {
            self.moving_direction = Direction::FlyingAway;
            self.rate_of_position_change = new_position - old_position;
        }
    }

    fn acceleration_rate(&self) -> isize {
        self.acceleration.0.abs() + self.acceleration.1.abs() + self.acceleration.2.abs()
    }
}

lazy_static! {
    static ref REGEX_ROW: Regex =
        Regex::new(r"p=(?<position><.*>), v=(?<velocity><.*>), a=(?<acceleration><.*>)").unwrap();
    static ref REGEX_VECTOR: Regex =
        Regex::new(r"<(?<x>-?[0-9]+),(?<y>-?[0-9]+),(?<z>-?[0-9]+)>").unwrap();
}

fn parse_vector(input: &str) -> Vector3 {
    let captures = REGEX_VECTOR.captures(input).unwrap();
    let x: isize = captures["x"].parse().unwrap();
    let y: isize = captures["y"].parse().unwrap();
    let z: isize = captures["z"].parse().unwrap();
    Vector3(x, y, z)
}

fn parse_row(id: usize, input: &str) -> Particle {
    let captures = REGEX_ROW.captures(input).unwrap();
    let position = parse_vector(&captures["position"]);
    let velocity = parse_vector(&captures["velocity"]);
    let acceleration = parse_vector(&captures["acceleration"]);
    Particle::new(id, position, velocity, acceleration)
}

fn parse_data(input: &str) -> Vec<Particle> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_row(i, line))
        .collect()
}

fn find_particles_with_minimal_acc(particles: Vec<Particle>) -> Vec<Particle> {
    let minimal_acceleration_rate = particles
        .iter()
        .min_by(|p1, p2| p1.acceleration_rate().cmp(&p2.acceleration_rate()))
        .map(|p| p.acceleration_rate())
        .unwrap();
    particles
        .into_iter()
        .filter(|p| p.acceleration_rate() == minimal_acceleration_rate)
        .collect()
}

fn find_particle_with_minimal_rate(particles: Vec<Particle>) -> usize {
    particles
        .iter()
        .min_by(|p1, p2| p1.rate_of_position_change.cmp(&p2.rate_of_position_change))
        .map(|p| p.id)
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let particles: Vec<Particle> = parse_data(input);
    let mut minimal_acceleration_particles = find_particles_with_minimal_acc(particles);

    let mut are_getting_closer = true;

    while are_getting_closer {
        let mut directions: Vec<Direction> =
            Vec::with_capacity(minimal_acceleration_particles.len());
        for particle in &mut minimal_acceleration_particles {
            particle.move_particle();
            directions.push(particle.moving_direction);
        }
        are_getting_closer = directions.iter().any(|d| *d == Direction::GettingCloser);
    }

    find_particle_with_minimal_rate(minimal_acceleration_particles)
}

pub fn part2(_: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_vector_test() {
        let input = "<-478,1930,2092>";
        assert_eq!(Vector3(-478, 1930, 2092), parse_vector(input));
    }

    #[test]
    fn parse_row_test() {
        let input = "p=<-478,1930,2092>, v=<20,-35,-114>, a=<0,-4,2>";
        let expected = Particle::new(
            0,
            Vector3(-478, 1930, 2092),
            Vector3(20, -35, -114),
            Vector3(0, -4, 2),
        );
        assert_eq!(expected, parse_row(0, input));
    }

    #[test]
    fn distance_from_origin_test() {
        let input = "p=<-478,1930,2092>, v=<20,-35,-114>, a=<0,-4,2>";
        assert_eq!(
            478 + 1930 + 2092,
            parse_row(0, input).distance_from_origin()
        );
    }

    #[test]
    fn move_particle_test() {
        let mut input = parse_row(0, "p=<113,-112,111>, v=<-13,12,11>, a=<3,2,-1>");
        input.move_particle();
        let mut expected = parse_row(0, "p=<103,-98,121>, v=<-10,14,10>, a=<3,2,-1>");
        expected.rate_of_position_change = 14;
        assert_eq!(expected, input);
        assert_eq!(Direction::GettingCloser, input.moving_direction);
    }

    #[test]
    fn move_particle_test2() {
        let mut input = parse_row(0, "p=<1,1,1>, v=<1,1,1>, a=<1,1,1>");
        input.move_particle();
        let mut expected = parse_row(0, "p=<3,3,3>, v=<2,2,2>, a=<1,1,1>");
        expected.rate_of_position_change = 6;
        expected.moving_direction = Direction::FlyingAway;
        assert_eq!(expected, input);
        assert_eq!(Direction::FlyingAway, input.moving_direction);
    }

    #[test]
    fn acceleration_rate_test() {
        let input = "p=<-478,1930,2092>, v=<20,-35,-114>, a=<0,-4,2>";
        assert_eq!(6, parse_row(0, input).acceleration_rate());
    }

    #[test]
    fn vector_addition() {
        let v1 = Vector3(1, 4, 13);
        let v2 = Vector3(-8, 8, -10);
        assert_eq!(Vector3(-7, 12, 3), v1 + v2);
    }

    #[test]
    fn vector_add_assign() {
        let mut v1 = Vector3(1, 4, 13);
        let v2 = Vector3(-8, 8, -10);
        v1 += v2;
        assert_eq!(Vector3(-7, 12, 3), v1);
    }
}
