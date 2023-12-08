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

    let mut i = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let direction = instructions.chars().nth(i % instructions.len()).unwrap();
        let (left, right) = nodes.get(current_node).unwrap();
        if direction == 'R' {
            current_node = right;
        } else {
            current_node = left;
        }
        i += 1;
    }
    println!("Took {} steps for part one!", i);
}
