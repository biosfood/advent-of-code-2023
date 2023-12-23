use std::env;
use std::fs;
use std::iter;
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

fn find_longest_path(tiles: &Vec<Vec<Tile>>, position: (isize, isize), visited: &mut HashSet<(isize, isize)>, disregard_slopes: bool) -> usize {
    let mut distance = 1;
    let mut current_position = position;
    visited.insert(current_position);
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
                if !disregard_slopes {
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
                }
            } else {
                continue;
            }
            next_positions.push(test_position);
        }
        match next_positions.len() {
            0 => {
                return if current_position.1 >= tiles.len() as isize - 2 {
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
                    max_distance = max_distance.max(find_longest_path(tiles, pos, &mut visited.clone(), disregard_slopes));
                }
                return distance + max_distance;
            }
        }
    }
}

fn longest_path(position: usize, visited: &mut HashSet<usize>, adjacencies: &Vec<Vec<usize>>) -> usize {
    if visited.contains(&position) {
        return 0;
    }
    if position == adjacencies.len() - 1 {
        return 1000_000;
    }
    visited.insert(position);
    let mut max_distance = 0;
    let mut max_index = 0;
    for i in 0..adjacencies.len() {
        if adjacencies[position][i] == 0 {
            continue;
        }
        let mut visited_cloned = visited.clone();
        let new = adjacencies[position][i] + longest_path(i, &mut visited_cloned, adjacencies);
        if new > max_distance {
            max_index = i;
            max_distance = max_distance.max(new);
        }
    }
    max_distance
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
    println!("Longest path: {}", find_longest_path(&tiles, start_position, &mut HashSet::new(), false) - 1000_001);
    let mut nodes = Vec::<(isize, isize)>::new();
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if tiles[y][x] == Tile::Forest {
                continue;
            }
            let mut connections = 0;
            for direction in &DIRECTIONS {
                let test_position = direction.offset((x as isize, y as isize));
                if let Some(tile) = tiles.get(test_position.1 as usize).and_then(|row| row.get(test_position.0 as usize)) {
                    if *tile != Tile::Forest {
                        connections += 1;
                    }
                }
            }
            if connections != 2 {
                nodes.push((x as isize, y as isize));
            }
        }
    }
    let mut adjacencies = (0..nodes.len()).map(|_| (0..nodes.len()).map(|_| 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    for i in 0..nodes.len() {
        let node = &nodes[i];
        // println!("Going from {:?}", node);
        for d in &DIRECTIONS {
            let mut previous = *node;
            let mut current = d.offset(*node);
            let mut distance = 1;
            if let Some(tile) = tiles.get(current.1 as usize).and_then(|row| row.get(current.0 as usize)) {
                if *tile == Tile::Forest {
                    continue;
                }
            } else {
                continue;
            }
            println!("Going from {:?} in direction {:?}", node, d);
            loop {
                if let Some(j) = nodes.iter().position(|n| n.0 == current.0 && n.1 == current.1) {
                    adjacencies[i][j] = distance;
                    adjacencies[j][i] = distance;
                    break;
                }
                for direction in &DIRECTIONS {
                    let next = direction.offset(current);
                    if next == previous {
                        continue;
                    }
                    if let Some(tile) = tiles.get(next.1 as usize).and_then(|row| row.get(next.0 as usize)) {
                        if *tile == Tile::Forest {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    previous = current;
                    current = next;
                    break;
                }
                distance += 1;
            }
        }
    }
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if adjacencies[i][j] != 0 {
                println!("{:?} -> {:?}: {:?}", nodes[i], nodes[j], adjacencies[i][j]);
            }
        }
    }
    println!("nodes: {:?}, adjacencies: {:?}", nodes, adjacencies);
    println!("Longest path part 2: {}", longest_path(0, &mut HashSet::new(), &adjacencies) - 1000_000);
}
