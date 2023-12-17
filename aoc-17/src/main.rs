use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::Builder;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn opposite(&self) -> &Direction {
        match self {
            Direction::Up => &Direction::Down,
            Direction::Down => &Direction::Up,
            Direction::Left => &Direction::Right,
            Direction::Right => &Direction::Left,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

fn find_best_path(field: &Vec<Vec<usize>>, buffer: &mut HashMap<(usize, usize, Direction), usize>, start: (usize, usize, Direction), visited: &mut HashSet<(usize, usize)>) -> usize {
    if visited.contains(&(start.0, start.1)) {
        return 1000000000;
    }
    if let Some(result) = buffer.get(&(start.0, start.1, start.2)) {
        return *result;
    }
    if start.0 == field[0].len() - 1 && start.1 == field.len() - 1 {
        return 0;
    }
    let mut result = usize::MAX;
    visited.insert((start.0, start.1));
    let mut result_data = "".to_string();
    for direction in DIRECTIONS {
        if direction == start.2 || direction == *start.2.opposite() {
            continue;
        }
        let mut position = (start.0 as isize, start.1 as isize);
        let mut total_cost = 0;
        let mut visited_cloned = visited.clone();
        for dist in 0..3 {
            position = direction.offset(position);
            if position.0 < 0 || position.1 < 0 || position.0 >= field[0].len() as isize || position.1 >= field.len() as isize {
                break;
            }
            total_cost += field[position.1 as usize][position.0 as usize];
            let value = total_cost + find_best_path(field, buffer, (position.0 as usize, position.1 as usize, direction),&mut visited_cloned);
            result = result.min(value);
            if result == value {
                result_data = format!("({}, {}: {} {:?}): {}: {}", position.0, position.1, dist + 1 , direction, total_cost, value);
            }
            visited_cloned.insert((position.0 as usize, position.1 as usize));
        }
    }
    buffer.insert((start.0, start.1, start.2), result);
    buffer.insert((start.0, start.1, *start.2.opposite()), result);
    if result < 1000000000 {
        // println!("best path: ({}, {}) -> {}", start.0, start.1, result_data);
    }
    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Horizontal,
    Vertical,
}

impl Dir {
    fn opposite(&self) -> &Dir {
        match self {
            Dir::Horizontal => &Dir::Vertical,
            Dir::Vertical => &Dir::Horizontal,
        }
    }

    fn compatible(&self, other: &Direction) -> bool {
        match self {
            Dir::Horizontal => {
                match other {
                    Direction::Up | Direction::Down => false,
                    _ => true
                }
            },
            Dir::Vertical => {
                match other {
                    Direction::Left | Direction::Right => false,
                    _ => true
                }
            }
        }
    }

    fn from(d: &Direction) -> Dir {
        match d {
            Direction::Left | Direction::Right => Dir::Horizontal,
            Direction::Up | Direction::Down => Dir::Vertical,
        }
    }
}

const DIR: [Dir; 2] = [Dir::Horizontal, Dir::Vertical];

fn find_best_path_alternative(field: &Vec<Vec<usize>>) -> usize {
    let mut Q = Vec::<(usize, usize, Dir)>::new();
    let mut distances = HashMap::<(usize, usize, Dir), usize>::new();
    let mut previous = HashMap::<(usize, usize, Dir), (usize, usize, Dir)>::new();
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            for d in DIR {
                distances.insert((x, y, d), usize::MAX / 2);
            }
        }
    }
    for d in DIR {
        distances.insert((0, 0, d), 0);
        Q.push((0, 0, d));
    }
    let mut done = HashSet::<(usize, usize, Dir)>::new();
    while !Q.is_empty() {
        println!("step: {}", Q.len());
        let min_distance = Q.iter().map(|v| distances[v]).min().unwrap();
        let u = Q.iter().find(|x| distances[x] == min_distance).unwrap().clone();
        Q.retain(|x| *x != u);
        let (x, y, d) = u;
        if x == field[0].len() - 1 && y == field.len() - 1 {
            return min_distance;
        }
        if done.contains(&(x, y, d)) {
            continue;
        }
        done.insert((x, y, d));
        for direction in DIRECTIONS {
            if d.compatible(&direction) {
                continue;
            }
            let mut position = (x as isize, y as isize);
            let mut total_cost = 0;
            for dist in 0..3 {
                position = direction.offset(position);
                if position.0 < 0 || position.1 < 0 || position.0 >= field[0].len() as isize || position.1 >= field.len() as isize {
                    break;
                }
                total_cost += field[position.1 as usize][position.0 as usize];
                let alt = min_distance + total_cost;
                let new_d = Dir::from(&direction);
                if alt < distances[&(position.0 as usize, position.1 as usize, new_d)] {
                    distances.insert((position.0 as usize, position.1 as usize, new_d), alt);
                    // previous.insert((position.0 as usize, position.1 as usize, new_d), (x, y, d));
                }
                Q.push((position.0 as usize, position.1 as usize, new_d));
            }
        }
    }
    panic!("no path found");
}

fn run(field: &Vec<Vec<usize>>) {
    println!("cheapest path: {}", find_best_path_alternative(field));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let field = lines.iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect::<Vec<Vec<usize>>>();

    let child = Builder::new()
    .stack_size(8000000000)
    .spawn(move || run(&field))
    .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
