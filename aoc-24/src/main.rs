use std::env;
use std::fs;
use f128::f128;

trait Vector {
    fn parse(line: &str) -> Self;
    fn add(&self, other: &Self) -> Self;
}

type V3 = [i128; 3];

impl Vector for V3 {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(",");
        let mut result = [0 as i128; 3];
        for i in 0..3 {
            result[i] = parts.next().unwrap().replace(" ", "").parse::<i128>().unwrap();
        }
        result
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = [0 as i128; 3];
        for i in 0..3 {
            result[i] = self[i] + other[i];
        }
        result
    }
}

#[derive(Debug)]
struct Particle {
    position: V3,
    velocity: V3,
    collision_time: f128,
    collision_partner: usize,
}

impl Particle {
    fn new(position: V3, velocity: V3) -> Particle {
        Particle { position: position, velocity: velocity, collision_time: f128::MAX, collision_partner: usize::MAX }
    }

    fn parse(line: &str) -> Particle {
        let mut parts = line.split(" @ ");
        let position = V3::parse(parts.next().unwrap());
        let velocity = V3::parse(parts.next().unwrap());
        Particle::new(position, velocity)
    }

    fn get_collision_time(&self, other: &Particle) -> (f128, f128) {
        let t_1_divisor = self.velocity[0] * other.velocity[1] - self.velocity[1] * other.velocity[0];
        if t_1_divisor == (0) {
            return (f128::from(-1), f128::from(-1));
        }
        let a = f128::from( self.velocity[1]) /  f128::from( self.velocity[0]);
        let c = f128::from( self.position[1]) - a * f128::from( self.position[0]);
        let b = f128::from(other.velocity[1]) / f128::from(other.velocity[0]);
        let d = f128::from(other.position[1]) - b * f128::from(other.position[0]);
        let p1 = (d-c)/(a-b);
        let t1 = (p1-f128::from( self.position[0])) / f128::from( self.velocity[0]);
        let t2 = (p1-f128::from(other.position[0])) / f128::from(other.velocity[0]);
        (t1, t2)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let mut particles = lines.iter().map(|line| Particle::parse(line)).collect::<Vec<Particle>>();
    let mut intersections = 0;

    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            let (t1, t2) = particles[i].get_collision_time(&particles[j]);
            if t1 < f128::from(0) || t2 < f128::from(0) {
                continue;
            }
            if t1 < particles[i].collision_time && t2 < particles[j].collision_time {
                particles[i].collision_time = t1;
                particles[i].collision_partner = j;
                particles[j].collision_time = t2;
                particles[j].collision_partner = i;
            }
            let position = (0..2).map(|index| f128::from(particles[i].position[index]) + t1 * f128::from(particles[i].velocity[index])).collect::<Vec<f128>>();
            let mut in_bounds = true;
            for pos in &position {
                if *pos < f128::from(200000000000000 as i128) || *pos > f128::from(400000000000000 as i128) {
                    in_bounds = false;
                }
            }
            if !in_bounds {
                continue;
            }
            intersections += 1;
        }
    }
    println!("Intersections: {}", intersections);
    println!("Please use a smarter piece of software to solve this system of equations: ");
    for i in 0..3 {
        for j in 0..3 {
            println!("p{j} + v{j} * t{i} == {} + {} * t{i} &&", particles[i].position[j], particles[i].velocity[j]);
        }
    }
}
