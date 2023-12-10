use std::env;
use std::fs;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    North, East, South, West
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West
];

impl Direction {
    fn opposite(&self) -> &Direction {
        match self {
            Direction::North => &Direction::South,
            Direction::East => &Direction::West,
            Direction::South => &Direction::North,
            Direction::West => &Direction::East
        }
    }

    fn offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y)
        }
    }

    fn index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3
        }
    }
}

#[derive(Eq, PartialEq)]
struct ConnectorType {
    directions: [bool; 4],
    symbol: char
}

#[derive(Clone, Eq, PartialEq)]
struct Node<'a> {
    position: (isize, isize),
    connector: &'a ConnectorType,
    connections: [Option<&'a Node<'a>>; 4],
}

static mut CONNECTORS: [ConnectorType; 8] = [
    ConnectorType { directions: [false, false, false, false], symbol: 'S' },
    ConnectorType { directions: [false, false, false, false], symbol: '.' },
    ConnectorType { directions: [true, false, true, false], symbol: '|' },
    ConnectorType { directions: [false, true, false, true], symbol: '-' },
    
    ConnectorType { directions: [true, true, false, false], symbol: 'L' },
    ConnectorType { directions: [true, false, false, true], symbol: 'J' },
    ConnectorType { directions: [false, false, true, true], symbol: '7' },
    ConnectorType { directions: [false, true, true, false], symbol: 'F' },
];

impl Node<'_> {
    unsafe fn new(position: (isize, isize), character: char) -> Node<'static> {
        let connector = CONNECTORS.iter().find(|c| c.symbol == character).unwrap();
        Node {
            position: position,
            connector: &connector,
            connections: [None, None, None, None]
        }
    }

}

fn get_new_connections<'a>(nodes: &'a Vec<Node>) -> Vec<[Option<&'static Node<'a>>; 4]> {
    let mut new_connections: Vec<[Option<&'static Node<'a>>; 4]> = Vec::new();
    for node in nodes {
        new_connections.push([None, None, None, None]);
        for direction_index in 0..4 {
            if !node.connector.directions[direction_index] {
                continue;
            }
            let direction = &DIRECTIONS[direction_index];
            let other = nodes.iter().find(|node| node.position == direction.offset(node.position));
            new_connections.last_mut().unwrap()[direction_index] = other;
        }
    }
    new_connections
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    static mut nodes: Vec<Node> = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let character = lines[y].chars().nth(x).unwrap();
            if character == '.' {
                continue;
            }
            unsafe {
                nodes.push(Node::new((x as isize, y as isize), character));
            }
        }
    }
    unsafe {
        for i in 2..8 {
            CONNECTORS[0].directions = CONNECTORS[i].directions;
            println!("checking with S as {}", CONNECTORS[i].symbol);
            //let new_connections = get_new_connections(&nodes);
            //for i in 0..nodes.len() {
                // nodes[i].connections.clone_from(&new_connections[i]);
            // }
            let mut already_visited: Vec<&Node> = Vec::new();
            let start_node = nodes.iter().find(|node| node.connector.symbol == 'S').unwrap();
            let mut node = start_node;
            let mut previous_direction = &DIRECTIONS[0];
            let mut length = 1;
            loop {
                // println!("position: {}, {}", node.position.0, node.position.1);
                if already_visited.contains(&node) {
                    println!("Found a loop after {} steps (already visited) => half length: {}", length, length / 2);
                    break;
                }
                already_visited.push(node);
                let next_direction_index = (0..4).find(|direction_index|
                    node.connector.directions[*direction_index] &&
                    DIRECTIONS[*direction_index] != *previous_direction).unwrap();
                let next_direction = &DIRECTIONS[next_direction_index];
                // println!("next direction: {:?}, next node position: {:?}", next_direction, next_direction.offset(node.position));
                let next_node_position = next_direction.offset(node.position);
                let next_node = nodes.iter().find(|node| node.position == next_node_position);
                if next_node.is_none() {
                    println!("Ran into a dead end after {} steps (no node)", length);
                    break;
                }
                let next_node = next_node.unwrap();
                if !next_node.connector.directions[next_direction.opposite().index()] {
                    println!("Ran into a dead end after {} steps (incompatible receiver)", length);
                    break;
                }
                node = next_node;
                length += 1;
                previous_direction = next_direction.opposite();
            }
        }
    }
}