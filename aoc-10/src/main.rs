use std::env;
use std::fs;

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
        }
    }
}

struct Stretch {
    min_x: usize,
    max_x: usize,
    has_north: bool,
    has_south: bool
}

fn find_stretch(visited_nodes: &mut Vec<&Node>, y: isize) -> Stretch {
    let mut result = Stretch {
        min_x: usize::MAX,
        max_x: usize::MIN,
        has_north: false,
        has_south: false
    };
    let min_x = visited_nodes.iter().filter(|node| node.position.1 == y).map(|node| node.position.0).min().unwrap();
    let mut node = visited_nodes.iter().find(|node| node.position.1 == y && node.position.0 == min_x).unwrap();
    result.min_x = min_x as usize;
    result.has_north |= node.connector.directions[Direction::North.index()];
    result.has_south |= node.connector.directions[Direction::South.index()];
    let mut max_x = min_x;
    while node.connector.directions[Direction::East.index()] {
        max_x += 1;
        node = visited_nodes.iter().find(|new_node| new_node.position.1 == y && new_node.position.0 == max_x).unwrap();
        result.has_north |= node.connector.directions[Direction::North.index()];
        result.has_south |= node.connector.directions[Direction::South.index()];
    }
    result.max_x = max_x as usize;
    for x in result.min_x..result.max_x + 1 {
        visited_nodes.remove(visited_nodes.iter().position(|node| node.position.1 == y && node.position.0 == x as isize).unwrap());
    }
    result
}

fn find_range(visited_nodes: &mut Vec<&Node>, y: isize) -> (usize, usize) {
    let first_stretch = find_stretch(visited_nodes, y);
    if first_stretch.has_north != first_stretch.has_south {
        return (first_stretch.min_x, first_stretch.max_x + 1);
    }
    let start = first_stretch.min_x;
    let mut stretch = find_stretch(visited_nodes, y);
    while !(stretch.has_north && stretch.has_south) {
        stretch = find_stretch(visited_nodes, y);
    }
    return (start, stretch.max_x + 1);
}

fn find_enclosed_area(visited_nodes: &mut Vec<&Node>, input: &Vec<String>) -> usize {
    let mut result = 0;
    let original_nodes = visited_nodes.clone();
    loop {
        if visited_nodes.is_empty() {
            return result;
        }
        let min_y = visited_nodes.iter().map(|node| node.position.1).min().unwrap();
        let (min_x, max_x) = find_range(visited_nodes, min_y);
        let text = input[min_y as usize].chars().collect::<Vec<char>>()[min_x..max_x].iter().collect::<String>();
        if text.contains('.') {
            println!("line {}, {} -> {}: \"{}\"", min_y, min_x, max_x, text);
        }

        for x in min_x..max_x {
            if original_nodes.iter().find(|node| node.position.1 == min_y && node.position.0 == x as isize).is_none() {
                result += 1;
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

    static mut NODES: Vec<Node> = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let character = lines[y].chars().nth(x).unwrap();
            if character == '.' {
                continue;
            }
            unsafe {
                NODES.push(Node::new((x as isize, y as isize), character));
            }
        }
    }
    unsafe {
        for i in 2..8 {
            CONNECTORS[0].directions = CONNECTORS[i].directions;
            println!("checking with S as {}", CONNECTORS[i].symbol);
            let mut already_visited: Vec<&Node> = Vec::new();
            let start_node = NODES.iter().find(|node| node.connector.symbol == 'S').unwrap();
            let mut node = start_node;
            let mut previous_direction = &DIRECTIONS[0];
            let mut length = 1;
            loop {
                if already_visited.contains(&node) {
                    println!("Found a loop after {} steps (already visited) => half length: {}, enclosed area: {}", length, length / 2,
                             find_enclosed_area(&mut already_visited, &lines));
                    break;
                }
                already_visited.push(node);
                let next_direction_index = (0..4).find(|direction_index|
                    node.connector.directions[*direction_index] &&
                    DIRECTIONS[*direction_index] != *previous_direction).unwrap();
                let next_direction = &DIRECTIONS[next_direction_index];
                let next_node_position = next_direction.offset(node.position);
                let next_node = NODES.iter().find(|node| node.position == next_node_position);
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