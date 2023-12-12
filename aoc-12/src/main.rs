use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

fn get_solution_count(states: &[State], groups: &[usize], preceding_damaged: usize, cache: &mut HashMap<(Vec<State>, Vec<usize>, usize), usize>) -> usize {
    if states.len() == 0{
        return if groups.len() == 0 && preceding_damaged == 0 || groups.len() == 1 && groups[0] == preceding_damaged {
            1
        } else {
            0
        }
    }
    let key = (states.to_vec(), groups.to_vec(), preceding_damaged);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap()
    }
    let result = match states[0] {
        State::Damaged => {
            get_solution_count(&states[1..], &groups[..], preceding_damaged + 1, cache)
        }
        State::Operational => {
            if preceding_damaged != 0 {
                if groups.len() == 0 || groups[0] != preceding_damaged {
                    0
                } else {
                    get_solution_count(&states[1..], &groups[1..], 0, cache)
                }
            } else {
                get_solution_count(&states[1..], &groups[..], 0, cache)
            }
        }
        State::Unknown => {
            get_solution_count(&states[1..], &groups[..], preceding_damaged + 1, cache) +
            if preceding_damaged != 0 {
                if groups.len() == 0 || groups[0] != preceding_damaged {
                    0
                } else {
                    get_solution_count(&states[1..], &groups[1..], 0, cache)
                }
            } else {
                get_solution_count(&states[1..], &groups[..], 0, cache)
            }
        }
    };
    cache.insert(key, result);
    result
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
    let mut solution_count = 0;
    let mut unfolded_count = 0;
    for line in lines {
        let result = parts_regex.captures(&line).unwrap();
        let groups = result["groups"].split(",").map(str::parse).map(Result::unwrap).collect::<Vec<usize>>();
        let states = result["records"].chars().map(State::from_char).collect::<Vec<State>>();
        let mut cache: HashMap<(Vec<State>, Vec<usize>, usize), usize> = HashMap::new();
        let solutions = get_solution_count(states.as_slice(), groups.as_slice(), 0, &mut cache);
        let mut unfolded_states: Vec<State> = Vec::new();
        let mut unfolded_groups: Vec<usize> = Vec::new();
        for _i in 0..4 {
            unfolded_states.append(&mut states.clone());
            unfolded_states.push(State::Unknown);
            unfolded_groups.append(&mut groups.clone());
        }
        unfolded_states.append(&mut states.clone());
        unfolded_groups.append(&mut groups.clone());
        let mut cache: HashMap<(Vec<State>, Vec<usize>, usize), usize> = HashMap::new();
        let unfolded = get_solution_count(unfolded_states.as_slice(), unfolded_groups.as_slice(), 0, &mut cache);
        println!("Found {solutions} solutions part 1, unfolded: {}", unfolded);
        solution_count += solutions;
        unfolded_count += unfolded;
    }
    println!("Found {} solutions, unfolded: {}", solution_count, unfolded_count);
}