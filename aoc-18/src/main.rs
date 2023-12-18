use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [ Direction::East, Direction::South, Direction::West, Direction:: North ];

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
    steps: isize,
    color: usize,
}

impl Instruction {
    fn new(s: &String) -> Instruction {
        let reg = Regex::new(r"(?<direction>\w) (?<steps>\d+) \(#(?<color>.+)\)").unwrap();
        let result = reg.captures(s).unwrap();
        let direction = Direction::from(result.get(1).unwrap().as_str().chars().next().unwrap());
        let steps = result.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let color = i64::from_str_radix(result.get(3).unwrap().as_str(), 16).unwrap() as usize;
        Instruction { direction, steps, color }
    }
}

fn get_flooded_area(instructions: &Vec<Instruction>) -> isize {
    let mut outline: Vec<(isize, isize, isize, Direction)> = Vec::new();
    let mut position = (0_isize, 0_isize);
    let mut visited_count = 0;
    let mut outline_size = 0;
    let mut shoelace_area = 0;
    let mut shoelace_position = (0_isize, 0_isize);
    for instruction in instructions {
        if instruction.direction == Direction::North {
            outline.push((position.1 - instruction.steps, position.1, position.0, instruction.direction));
        } else if instruction.direction == Direction::South {
            outline.push((position.1, position.1 + instruction.steps, position.0, instruction.direction));
        }
        for _ in 0..instruction.steps {
            position = instruction.direction.offset(position);
            outline_size += 1;
        }
        shoelace_area += shoelace_position.0 * position.1 - shoelace_position.1 * position.0;
        shoelace_position = position;
    }
    println!("shoelace area: {}", (shoelace_area+1)/2 + outline_size / 2 + 1);
    println!("outline size: {}", outline_size);
    outline.sort_by(|(_, _, x_1, _), (_, _, x_2, _)| x_1.partial_cmp(x_2).unwrap());
    let min_y = outline.iter().map(|(y_start, y_end, x, direction)| *y_start).min().unwrap() - 10;
    let max_y = outline.iter().map(|(y_start, y_end, x, direction)| *y_end).max().unwrap() + 10;
    let mut cache: HashMap<Vec<(isize, isize, isize, Direction)>, isize> = HashMap::new();
    println!("y range: {} .. {}", min_y, max_y);
    for y in min_y..max_y {
        let mut relevant_ranges = outline.iter().filter(|(y_start, y_end, x, direction)| y >= *y_start && y <= *y_end).map(|x| *x).collect::<Vec<(isize, isize, isize, Direction)>>();
        if let Some(result) = cache.get(&relevant_ranges) {
            visited_count += *result;
            continue;
        }
        let relevant_ranges_insert = relevant_ranges.clone();
        let mut row_sum = 0;
        while relevant_ranges.len() > 0 {
            if relevant_ranges[0].3 == relevant_ranges[1].3 {
                row_sum += relevant_ranges[1].2 - relevant_ranges[0].2;
                relevant_ranges.remove(0);
                continue;
            }
            row_sum += relevant_ranges[1].2 - relevant_ranges[0].2 + 1;
            if relevant_ranges.len() > 2 && relevant_ranges[1].3 == relevant_ranges[2].3 {
                row_sum += relevant_ranges[2].2 - relevant_ranges[1].2;
                relevant_ranges.remove(0);
                relevant_ranges.remove(0);
                relevant_ranges.remove(0);
            } else {
                relevant_ranges.remove(0);
                relevant_ranges.remove(0);
            }
        }
        cache.insert(relevant_ranges_insert, row_sum);
        visited_count += row_sum;
        // println!("relevant: {:?}", relevant_ranges);
    }
    visited_count
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
    println!("Part one: {}", get_flooded_area(&instructions));
    let instructions = instructions.iter().map(|instruction| Instruction {
        color: 0,
        steps: (instruction.color / 16) as isize,
        direction: DIRECTIONS[instruction.color % 16]
    }).collect::<Vec<Instruction>>();
    // println!("new instruction: {:?}", instructions);
    println!("Part two: {}", get_flooded_area(&instructions));
    
}
