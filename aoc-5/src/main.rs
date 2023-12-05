use std::env;
use std::fs;
use std::iter;
use regex::Regex;
use std::cmp::min;

fn translate(key: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination_start, source_start, length) in map {
        if (*source_start..(source_start + length)).contains(&key) {
            return destination_start + (key - source_start);
        }
    }
    return key;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let numbers: Regex = Regex::new(r"\d+").unwrap();
    let seeds = numbers.captures_iter(lines[0].as_str()).map(|x| x.get(0).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut steps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut current_map: Vec<(u64, u64, u64)> = Vec::new();

    for line in &lines[1..] {
        if line.len() == 0 {
            continue;
        }
        if numbers.captures_iter(line.as_str()).count() == 0 {
            if current_map.len() > 0 {
                println!("collected {} entries", current_map.len());
                steps.push(current_map.clone());
            }
            println!("reading {}", line);
            current_map = Vec::new();
            continue;
        }
        let numbers_in_line = numbers.captures_iter(line.as_str()).map(|x| x.get(0).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        current_map.push((numbers_in_line[0], numbers_in_line[1], numbers_in_line[2]));
    }
    if current_map.len() > 0 {
        println!("collected {} entries", current_map.len());
        steps.push(current_map.clone());
    }
    let mut minLocation = u64::MAX;
    for seed in seeds {
        let mut value = seed;
        println!("{}", seed);
        for step in &steps {
            value = translate(value, step);
            println!("-> {}", value);
        }
        if value < minLocation {
            minLocation = value;
        }
    }
    println!("min: {}", minLocation);
}
