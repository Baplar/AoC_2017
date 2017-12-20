use regex::{Error, Regex};
use std::ops::{Add, Sub};
use std::collections::HashMap;

/// A point in 3D space
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    /// Manhattan distance between the point and the origin (0,0,0),
    /// aka the sum of the absolute values of each coordinate.
    fn manhattan(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// A particle, described by its position, velocity and acceleration.
#[derive(Clone, Copy)]
pub struct Particle {
    p: Point,
    v: Point,
    a: Point,
}

impl Particle {
    fn new(p: Point, v: Point, a: Point) -> Self {
        Particle { p, v, a }
    }

    /// Simulates a step of the particle in the simulation
    fn step(&mut self) {
        self.v = self.v + self.a;
        self.p = self.p + self.v;
    }
}

struct Parser {
    re: Regex,
}

impl Parser {
    fn new() -> Result<Self, Error> {
        let re = Regex::new(r"^p=<(.+),(.+),(.+)>, v=<(.+),(.+),(.+)>, a=<(.+),(.+),(.+)>$")?;
        Ok(Parser { re })
    }

    /// Parses a particle definition string
    fn parse_particle(&self, s: &str) -> Result<Particle, String> {
        let caps = self.re
            .captures(s.trim())
            .ok_or("Not a particle definition")?;

        let values: Vec<isize> = caps.iter()
            .skip(1)
            .take(9)
            .map(|s| s.unwrap().as_str().parse::<isize>().unwrap())
            .collect();

        let p = Point::new(values[0], values[1], values[2]);
        let v = Point::new(values[3], values[4], values[5]);
        let a = Point::new(values[6], values[7], values[8]);

        Ok(Particle::new(p, v, a))
    }

    /// Parses a list of particle definitions
    fn parse(&self, s: &str) -> Vec<Particle> {
        s.trim()
            .split('\n')
            .filter_map(|s| self.parse_particle(s.trim()).ok())
            .collect()
    }
}

/// Finds the particle which will be the closest to the origin
/// in the very long term.
///
/// The result is simply the particle
/// with the lowest acceleration
pub fn one(s: &str) -> String {
    let parser = if let Ok(parser) = Parser::new() {
        parser
    } else {
        return String::from("Could not create parser");
    };
    let particles = parser.parse(s);
    let i = if let Some((i, _)) = particles
        .into_iter()
        .enumerate()
        .min_by_key(|&(_, ref x)| x.a.manhattan())
    {
        i
    } else {
        0
    };
    i.to_string()
}

/// Runs a step in the particle simulation,
/// destroying colliding particles
pub fn sim_step(particles: Vec<Particle>) -> Vec<Particle> {
    let mut grid = HashMap::new();
    for mut part in particles {
        part.step();
        grid.entry(part.p).or_insert_with(|| vec![]).push(part);
    }
    grid.into_iter()
        .filter(|&(_, ref v)| v.len() == 1)
        .map(|(_, v)| v[0])
        .collect()
}

/// Runs the simulation until the number of particles
/// seems to stop changing, aka when collisions stop happening
/// for long enough.
pub fn two(s: &str) -> String {
    let parser = if let Ok(parser) = Parser::new() {
        parser
    } else {
        return String::from("Could not create parser");
    };
    let mut particles = parser.parse(s);

    let mut nb_particles = particles.len();
    let mut nb_iter_without_change = 0;
    while nb_iter_without_change < 1000 {
        particles = sim_step(particles);
        if particles.len() < nb_particles {
            nb_particles = particles.len();
            nb_iter_without_change = 0;
        } else {
            nb_iter_without_change += 1;
        }
    }

    nb_particles.to_string()
}
