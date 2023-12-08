use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;


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

    let mut i1 = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let direction = instructions.chars().nth(i1 % instructions.len()).unwrap();
        let (left, right) = nodes.get(current_node).unwrap();
        if direction == 'R' {
            current_node = right;
        } else {
            current_node = left;
        }
        i1 += 1;
    }

    let mut i2 = 0;
    let mut current_nodes = nodes.keys().filter(|node| node.chars().nth(node.len()-1).unwrap() == 'A').map(|node| node).collect::<Vec<&String>>();
    println!("starting at {} nodes that end with an A", current_nodes.len());
    let mut should_continue = true;
    while should_continue {
        let direction = instructions.chars().nth(i2 % instructions.len()).unwrap();
        let new_nodes = current_nodes.iter().map(|node| 
            if direction == 'R' { &nodes.get(*node).unwrap().1 }
            else { &nodes.get(*node).unwrap().0 }).collect::<Vec<&String>>();
        current_nodes = new_nodes;

        should_continue = false;
        for node in &current_nodes {
            if node.chars().nth(node.len()-1).unwrap() != 'Z' {
                should_continue = true;
                break;
            }
        }
        i2 += 1;
    }
    println!("Took {} steps for part one, {} for part two", i1, i2);
}
