use std::env;
use std::fs;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone, Debug, Hash, Copy)]
enum Tile {
    Empty,
    Rolling,
    Static,
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            'O' => Tile::Rolling,
            '#' => Tile::Static,
            _ => panic!("cannot parse {c} as a Tile!")
        }
    }

    fn from_string(string: &String) -> Vec<Tile> {
        string.chars().map(Tile::from).collect()
    }
}

fn tilt_north(field: &mut Vec<Vec<Tile>>) {
    for _ in 0..field.len() {
        for y in 1..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] == Tile::Rolling && field[y-1][x] == Tile::Empty {
                    field[y][x] = Tile::Empty;
                    field[y-1][x] = Tile::Rolling;
                }
            }
        }
    }
}

fn tilt_south(field: &mut Vec<Vec<Tile>>) {
    for _ in 0..field.len() {
        for y in (0..field.len()-1).rev() {
            for x in 0..field[y].len() {
                if field[y][x] == Tile::Rolling && field[y+1][x] == Tile::Empty {
                    field[y][x] = Tile::Empty;
                    field[y+1][x] = Tile::Rolling;
                }
            }
        }
    }
}

fn tilt_east(field: &mut Vec<Vec<Tile>>) {
    for _ in 0..field.len() {
        for y in 0..field.len() {
            for x in (0..field[y].len()-1).rev() {
                if field[y][x] == Tile::Rolling && field[y][x+1] == Tile::Empty {
                    field[y][x] = Tile::Empty;
                    field[y][x+1] = Tile::Rolling;
                }
            }
        }
    }
}

fn tilt_west(field: &mut Vec<Vec<Tile>>) {
    for _ in 0..field.len() {
        for y in 0..field.len() {
            for x in 1..field[y].len() {
                if field[y][x] == Tile::Rolling && field[y][x-1] == Tile::Empty {
                    field[y][x] = Tile::Empty;
                    field[y][x-1] = Tile::Rolling;
                }
            }
        }
    }
}

fn north_tension(field: &Vec<Vec<Tile>>) -> usize {
    let height = field.len();
    field.iter().enumerate().map(|(y, line)|
        (height-y) *
        line.iter().map(|tile| if *tile == Tile::Rolling { 1 } else { 0 }).sum::<usize>()).sum::<usize>()
}

fn get_hash(field: &Vec<Vec<Tile>>) -> u64 {
    let mut s = DefaultHasher::new();
    field.hash(&mut s);
    s.finish()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|s| !s.is_empty()).collect();
    let mut field: Vec<Vec<Tile>> = lines.iter().map(Tile::from_string).collect();
    println!("before rolling: {}", north_tension(&field));
    tilt_north(&mut field);
    println!("after rolling north once: {}", north_tension(&field));

    let mut transitions: HashMap<u64, u64> = HashMap::new();
    let mut fields: HashMap<u64, Vec<Vec<Tile>>> = HashMap::new();
    let mut tensions: HashMap<u64, usize> = HashMap::new();
    let mut current_hash = get_hash(&field);
    let start_hash = current_hash;
    let mut cycle_start = 0;
    let mut start_offset = 0;
    for i in 0..1000000000 {
        if let Some(value) = transitions.get(&current_hash) {
            current_hash = *value;
            cycle_start = current_hash;
            start_offset = i;
            break;
        }
        let old_hash = current_hash;
        tilt_north(&mut field);
        tilt_west(&mut field);
        tilt_south(&mut field);
        tilt_east(&mut field);
        current_hash = get_hash(&field);
        transitions.insert(old_hash, current_hash);
        fields.insert(current_hash, field.clone());
        let tension =  north_tension(&field);
        tensions.insert(current_hash, tension);
        println!("after {} cycles: {}", i+1, tension);
    }
    let mut cycle_length = 0;
    current_hash = cycle_start;
    current_hash = *transitions.get(&current_hash).unwrap();
    loop {
        if current_hash == cycle_start {
            break;
        }
        cycle_length += 1;
        current_hash = *transitions.get(&current_hash).unwrap();
    }
    let mut offset = 0;
    println!("cycle start: {} cycle length: {}", start_offset, cycle_length);
    let mut position = start_offset + (1000000000 - start_offset) % (cycle_length + 1);
    let mut hash = start_hash;
    for i in 0..position {
        hash = transitions[&hash];
    }
    println!("after 1000000000 cycles: {} -> {}", position, tensions[&hash]);
}
