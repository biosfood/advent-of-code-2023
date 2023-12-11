use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Eq, PartialEq)]
struct Star {
    x: isize,
    y: isize,
}

impl Star {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Star) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let mut stars = Vec::<Star>::new();
    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                stars.push(Star::new(x as isize, y as isize));
            }
        }
    }
    let occupied_rows: HashSet<isize> = HashSet::from_iter(stars.iter().map(|s| s.y));
    let occupied_cols: HashSet<isize> = HashSet::from_iter(stars.iter().map(|s| s.x));
    for star in &mut stars {
        print!("{},{} -> ", star.x, star.y);
        star.x += (0..star.x).filter(|x| !occupied_cols.contains(x)).count() as isize;
        star.y += (0..star.y).filter(|y| !occupied_rows.contains(y)).count() as isize;
        println!("{},{}", star.x, star.y);
    }
    let mut distance_sum = 0;
    let mut pairs = 0;
    for (i, star) in stars.iter().enumerate() {
        let mut distances = stars.iter().skip(i+1).map(|other_star| {
            pairs += 1;
            star.distance(other_star)
        }).sum::<isize>();
        println!("star {i} has distances summing to {distances}");
        distance_sum += distances;
    }
    println!("total distance sum: {distance_sum}, total pairs: {pairs}, test: {}", &stars[4].distance(&stars[8]));
}