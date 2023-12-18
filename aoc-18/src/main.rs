use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::North,
            'R' => Direction::East,
            'D' => Direction::South,
            'L' => Direction::West,
            _ => panic!("Invalid direction {}", c),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn offset(self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    steps: usize,
    color: usize,
}

impl Instruction {
    fn new(s: &String) -> Instruction {
        let reg = Regex::new(r"(?<direction>\w) (?<steps>\d+) \(#(?<color>.+)\)").unwrap();
        let result = reg.captures(s).unwrap();
        let direction = Direction::from(result.get(1).unwrap().as_str().chars().next().unwrap());
        let steps = result.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let color = i64::from_str_radix(result.get(3).unwrap().as_str(), 16).unwrap() as usize;
        Instruction { direction, steps, color }
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

    let instructions = lines.iter().map(Instruction::new).collect::<Vec<Instruction>>();
    let mut outline: Vec<(isize, isize, Instruction)> = Vec::new();
    let mut position = (0, 0);
    for instruction in &instructions {
        outline.push((position.0, position.1, *instruction));
        for _ in 0..instruction.steps {
            position = instruction.direction.offset(position);
            outline.push((position.0, position.1, *instruction));
        }
    }
    let mut vertical_outlines = outline.iter().filter(|(_, _, instruction)| instruction.direction == Direction::North || instruction.direction == Direction::South).collect::<Vec<_>>();
    let mut visited = HashSet::<(isize, isize)>::new();
    while !vertical_outlines.is_empty() {
        println!("vertical outlines: {}", vertical_outlines.len());
        let y = vertical_outlines.iter().map(|(_, y, _)| *y).min().unwrap();
        let mut state = false;
        let mut x = vertical_outlines.iter().filter(|(_, outline_y, _)| *outline_y == y).map(|(x, _, _)| *x).min().unwrap();
        let mut direction = Direction::East;
        let mut toggle = false;
        while vertical_outlines.iter().find(|(_, o_y, _)|  *o_y == y).is_some() {
            if let Some(outline) = vertical_outlines.iter().find(|(o_x, o_y, _o_d)| *o_x == x && *o_y == y) {
                if outline.2.direction != direction {
                    toggle = !toggle;
                    direction = outline.2.direction
                }
                vertical_outlines.retain(|(v_x, v_y, _v_d)| !(*v_x == x && *v_y == y));
            } else if toggle {
                visited.insert((x, y));
            }
            x += 1;
        }
    }
    for (x, y, d) in outline {
        visited.insert((x, y));
    }
    // println!("Outline: {:?}", outline);
    // println!("end position: {:?}", position);
    println!("Visited: {:?}", visited.len());
}
