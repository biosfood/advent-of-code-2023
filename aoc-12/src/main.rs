use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl State {
    fn from_char(c: char) -> State {
        match c {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => panic!("Unexpected character: {c}"),
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

    let parts_regex = Regex::new(r"(?<records>[\.#\?]+) (?<groups>(\d+,)*\d+)").unwrap();
    let mut soulution_count = 0;
    for line in lines {
        let result = parts_regex.captures(&line).unwrap();
        let groups = result["groups"].split(",").map(str::parse).map(Result::unwrap).collect::<Vec<isize>>();
        let states = result["records"].chars().map(State::from_char).collect::<Vec<State>>();
        let number_of_unknowns = states.iter().filter(|state| **state == State::Unknown).count();
        // println!("{:?}: {:?}, {} unknowns", states, groups, number_of_unknowns);
        for i in 0..(1 << (number_of_unknowns)) {
            let mut unknown_index = 0;
            let current_states = states.iter().map(|state| {
                if *state == State::Unknown {
                    if i & 1 << unknown_index == 0 {
                        unknown_index += 1;
                        State::Operational
                    } else {
                        unknown_index += 1;
                        State::Damaged
                    }
                } else {
                    *state
                }
            }).collect::<Vec<State>>();
            let mut current_groups = Vec::<isize>::new();
            let mut current_length = 0;
            for state in current_states.iter() {
                if *state == State::Damaged {
                    current_length += 1;
                } else if current_length != 0 {
                    current_groups.push(current_length);
                    current_length = 0;
                }
            }
            if current_length > 0 {
                current_groups.push(current_length);
            }
            // println!("{:?} -> {:?}", current_states, current_groups);
            if current_groups == groups {
                soulution_count += 1;
                // println!("Found solution: {:?}", current_states);
            }
        }
    }
    println!("Found {} solutions", soulution_count);
}