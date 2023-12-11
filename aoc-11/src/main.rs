use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone)]
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

fn compute_distances_sum(stars: &Vec<Star>) -> isize {
    stars.iter().enumerate().map(|(i, star)|
        stars.iter().skip(i+1).map(|other_star| {
            star.distance(other_star)
        }).sum::<isize>()
    ).sum::<isize>()
}

fn expand_stars(stars: &mut Vec<Star>, occupied_rows: &HashSet<isize>, occupied_cols: &HashSet<isize>, distance: isize) {
    for star in stars {
        star.x += (0..star.x).filter(|x| !occupied_cols.contains(x)).count() as isize * (distance-1);
        star.y += (0..star.y).filter(|y| !occupied_rows.contains(y)).count() as isize * (distance-1);
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
    let mut stars_part_one = stars.clone();
    let mut stars_part_two = stars.clone();
    expand_stars(&mut stars_part_one, &occupied_rows, &occupied_cols, 2);
    expand_stars(&mut stars_part_two, &occupied_rows, &occupied_cols, 1000000);
    println!("total distance sum for expansion factor 2: {}, 1000: {}", compute_distances_sum(&stars_part_one), compute_distances_sum(&stars_part_two));
}