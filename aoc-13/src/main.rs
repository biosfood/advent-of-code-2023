use std::env;
use std::fs;
use std::collections::HashSet;

fn get_symmetry(pattern: &Vec<String>, avoid_return: usize) -> usize {
    for test_symmetry_position in 1..pattern.len() {
        let range = if test_symmetry_position <= pattern.len() / 2 {
            0..test_symmetry_position
        } else {
            test_symmetry_position..pattern.len()
        };
        let mut is_y_symmetric = true;
        for y in range {
            if pattern[y] != pattern[2*test_symmetry_position - y - 1] {
                is_y_symmetric = false;
                break;
            }
        }
        if is_y_symmetric && test_symmetry_position * 100 != avoid_return {
            return test_symmetry_position * 100;
        }
    }
    for test_symmetry_position in 1..pattern[0].len() {
        let range = if test_symmetry_position <= pattern[0].len() / 2 {
            0..test_symmetry_position
        } else {
            test_symmetry_position..pattern[0].len()
        };
        let mut is_x_symmetric = true;
        for x in range {
            for line in pattern {
                if line.chars().nth(x) != line.chars().nth(2*test_symmetry_position - x - 1) {
                    is_x_symmetric = false;
                    break;
                }
            }
            if !is_x_symmetric {
                break;
            }
        }
        if is_x_symmetric && test_symmetry_position != avoid_return {
            return test_symmetry_position;
        }
    }
    // println!("Failed to find symmetry in {:?}", pattern);
    return 0;
}

fn smudge(original: String, position: usize) -> String {
    let mut smudged = original.clone();
    if original.chars().nth(position).unwrap() == '.' {
        smudged.replace_range(position..position+1, "#");
    } else {
        smudged.replace_range(position..position+1, ".");
    }
    return smudged;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let mut patterns = Vec::<Vec<String>>::new();
    let mut current_pattern = Vec::<String>::new();
    for line in lines {
        if line.is_empty() && current_pattern.len() != 0 {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
            continue;
        }
        if !line.is_empty() {
            current_pattern.push(line);
        }
    }
    if current_pattern.len() != 0 {
        patterns.push(current_pattern);
    }
    let mut normal_sum = 0;
    let mut smudged_sum = 0;
    for pattern in patterns {
        // check for symmetry in y-direction
        let normal_symmetry = get_symmetry(&pattern, 0);
        normal_sum += normal_symmetry;
        let mut smudged_solutions = HashSet::<usize>::new();
        for y in 0..pattern.len() {
            for x in 0..pattern[y].len() {
                let mut smudged_pattern = pattern.clone();
                smudged_pattern[y] = smudge(pattern[y].clone(), x);
                let symmetry = get_symmetry(&smudged_pattern, normal_symmetry);
                if symmetry != 0 && symmetry != normal_symmetry {
                    smudged_solutions.insert(symmetry);
                }
            }
        }
        if smudged_solutions.is_empty() {
            panic!("no smudged solutions for {pattern:?}");
        } else if smudged_solutions.len() == 1 {
            println!("one smudged solution for {pattern:?}: {smudged_solutions:?}");
            smudged_sum += smudged_solutions.iter().next().unwrap();
        } else {
            panic!("multiple smudged solutions for {pattern:?}: {smudged_solutions:?}");
        }
    }
    println!("Sum: {normal_sum}, smudged sum: {smudged_sum}");
}
