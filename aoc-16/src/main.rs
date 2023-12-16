use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    SplitterHorizontal,
    SplitterVertical,
    MirrorSE,
    MirrorNE,
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            '/' => Tile::MirrorNE,
            '\\' => Tile::MirrorSE,
            _ => panic!("Invalid character {c}"),
        }
    }
}

fn get_next_directions(tile: &Tile, direction: Direction) -> Vec<Direction> {
    match tile {
        Tile::Empty => {
            vec!(direction)
        },
        Tile::SplitterHorizontal => {
            match direction {
                Direction::North | Direction::South => {
                    vec!(Direction::East, Direction::West)
                },
                Direction::East | Direction::West => {
                    vec!(direction)
                }
            }
        },
        Tile::SplitterVertical => {
            match direction {
                Direction::East | Direction::West => vec!(Direction::North, Direction::South),
                Direction::North | Direction::South => vec!(direction),
            }
        },
        Tile::MirrorSE => match direction {
                Direction::North => vec!(Direction::West),
                Direction::East => vec!(Direction::South),
                Direction::South => vec!(Direction::East),
                Direction::West => vec!(Direction::North),
            },
        Tile::MirrorNE => match direction {
                Direction::North => vec!(Direction::East),
                Direction::East => vec!(Direction::North),
                Direction::South => vec!(Direction::West),
                Direction::West => vec!(Direction::South),
            },
    }
}

fn get_energized_tiles(field: &Vec<Vec<Tile>>, start: (isize, isize, Direction)) -> usize {
    let mut current_beams = vec![start];
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut past_beams: HashSet<(isize, isize, Direction)> = HashSet::new();
    while !current_beams.is_empty() {
        let mut next_beams = Vec::<(isize, isize, Direction)>::new();
        for beam in current_beams {
            if past_beams.contains(&beam) {
                continue;
            }
            past_beams.insert(beam);
            let (x, y, direction) = beam;
            visited_positions.insert((x, y));
            for direction in get_next_directions(&field[y as usize][x as usize], direction) {
                let (next_x, next_y) = direction.offset((x, y));
                if next_x < 0 || next_y < 0 || next_x >= field[0].len() as isize || next_y >= field.len() as isize {
                    continue;
                }
                next_beams.push((next_x, next_y, direction));
            }
        }
        current_beams = next_beams;
    }
    visited_positions.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let field: Vec<Vec<Tile>> = lines.iter().map(|line| line.chars().map(Tile::from).collect()).collect();
    let part_one = get_energized_tiles(&field, (0, 0, Direction::East));
    let mut part_two = part_one;
    for x in 0..field[0].len() {
        part_two = part_two.max(get_energized_tiles(&field, (x as isize, 0, Direction::South)));
        part_two = part_two.max(get_energized_tiles(&field, (x as isize, field.len() as isize - 1, Direction::North)));
    }
    for y in 0..field.len() {
        part_two = part_two.max(get_energized_tiles(&field, (0, y as isize, Direction::East)));
        part_two = part_two.max(get_energized_tiles(&field, (field[0].len() as isize - 1, y as isize, Direction::West)));
    }
    println!("Part 1: {part_one}, Part 2: {part_two}");
}