use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Path, Forest, SlopeN, SlopeS, SlopeE, SlopeW
}

impl Tile {
    fn from_char(character: char) -> Tile {
        match character {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '>' => Tile::SlopeE,
            'v' => Tile::SlopeS,
            '<' => Tile::SlopeW,
            '^' => Tile::SlopeN,
            _ => panic!("Unknown character {character}"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West
];

fn find_longest_path(tiles: &Vec<Vec<Tile>>, position: (isize, isize), visited: &mut HashSet<(isize, isize)>) -> usize {
    let mut distance = 1;
    let mut current_position = position;
    println!("checking from {position:?}");
    loop {
        let mut next_positions = Vec::<(isize, isize)>::new();
        for direction in DIRECTIONS {
            let test_position = direction.offset(current_position);
            if visited.contains(&test_position) {
                continue;
            }
            if let Some(tile) = tiles.get(test_position.1 as usize).and_then(|row| row.get(test_position.0 as usize)) {
                if *tile == Tile::Forest {
                    continue;
                }
                if *tile == Tile::SlopeE && direction != Direction::East {
                    continue;
                }
                if *tile == Tile::SlopeN && direction != Direction::North {
                    continue;
                }
                if *tile == Tile::SlopeW && direction != Direction::West {
                    continue;
                }
                if *tile == Tile::SlopeS && direction != Direction::South {
                    continue;
                }
            } else {
                continue;
            }
            next_positions.push(test_position);
        }
        match next_positions.len() {
            0 => {
                println!("position: {position:?}");
                return if position.1 >= tiles.len() as isize - 2 {
                    distance + 1000_000
                } else {
                    distance
                }
                },
            1 => {
                visited.insert(next_positions[0]);
                current_position = next_positions[0];
                distance += 1;
            },
            _ => {
                let mut max_distance = 0;
                for pos in next_positions {
                    max_distance = max_distance.max(find_longest_path(tiles, pos, &mut visited.clone()));
                }
                return distance + max_distance;
            }
        }
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

    let tiles = lines.iter().map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>()).collect::<Vec<_>>();
    let start_position = (tiles[0].iter().position(|tile| *tile == Tile::Path).unwrap() as isize, 0);
    println!("Longest path: {}", find_longest_path(&tiles, start_position, &mut HashSet::new()) - 1);
}
