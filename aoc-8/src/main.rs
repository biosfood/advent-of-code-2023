use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn get_path_length<T>(instructions: &str, start_node: String, nodes: &HashMap<String, (String, String)>, end_condition: T) -> usize 
where T: Fn(&String) -> bool {
    let mut i = 0;
    let mut current_node = &start_node;
    while !end_condition(current_node) {
        let direction = instructions.chars().nth(i % instructions.len()).unwrap();
        let (left, right) = nodes.get(current_node).unwrap();
        if direction == 'R' {
            current_node = right;
        } else {
            current_node = left;
        }
        i += 1;
    }
    i
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcd(x: usize, y: usize) -> usize {
    (x*y)/gcd(x, y)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let instructions = lines[0].as_str();
    let mut nodes = HashMap::<String, (String, String)>::new();

    let line_regex = Regex::new(r"(?<key>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
    for line in &lines[2..] {
        if line.is_empty() {
            continue;
        }
        let result = line_regex.captures(line).unwrap();
        let key = result["key"].to_owned();
        let left = result["left"].to_owned();
        let right = result["right"].to_owned();    
        nodes.insert(key, (left, right));
    }

    let i1 = get_path_length(instructions, "AAA".to_string(), &nodes, |node| node == "ZZZ");
    let start_nodes = nodes.keys().filter(|node| node.chars().nth(node.len()-1).unwrap() == 'A').map(|node| node).collect::<Vec<&String>>();
    let distances = start_nodes.iter().map(|node| get_path_length(instructions, node.to_string(), &nodes, |node| node.chars().nth(node.len()-1).unwrap() == 'Z')).collect::<Vec<usize>>();
    let mut i2 = 1;
    for distance in &distances {
        i2 = lcd(i2, *distance);
    }

    println!("Took {} steps for part one, {:?} -> {} for part two", i1, distances, i2);
}
