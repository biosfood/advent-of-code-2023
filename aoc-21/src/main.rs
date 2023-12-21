use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::option::Option;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Starting, Garden, Rock,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Garden,
            '#' => Tile::Rock,
            'S' => Tile::Starting,
            _ => panic!("Unexpected character: {c}"),
        }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Starting => "S",  
                Tile::Garden => ".",
                Tile::Rock => "#",
            }
        )
    }
}

enum Directions {
    North, East, South, West
}

impl Directions {
    fn offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Directions::North => (x, y - 1),
            Directions::East => (x + 1, y),
            Directions::South => (x, y + 1),
            Directions::West => (x - 1, y),
        }
    }
}

const DIRECTIONS: [Directions; 4] = [
    Directions::North,
    Directions::East,
    Directions::South,
    Directions::West
];

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let mut tiles = HashMap::<(isize, isize), Tile>::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let character = lines[y].chars().nth(x).unwrap();
            tiles.insert((x as isize, y as isize), Tile::from_char(character));
        }
    }
    let mut active_positions = tiles.iter().filter(|(_, tile)| **tile == Tile::Starting).map(|(pos, _)| *pos).collect::<Vec<_>>();
    for i in 0..64 {
        let mut next_positions = HashSet::<(isize, isize)>::new();
        for pos in &active_positions {
            for direction in &DIRECTIONS {
                let next_position = direction.offset(*pos);
                if let Some(tile) = tiles.get(&next_position) {
                    if *tile != Tile::Rock {
                        next_positions.insert(next_position);
                    }
                }
            }
        }
        active_positions = next_positions.iter().map(|pos| *pos).collect::<Vec<_>>();
    }
    println!("Active positions: {}", active_positions.len());
}
