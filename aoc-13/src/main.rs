use std::env;
use std::fs;

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
    let mut sum = 0;
    for pattern in patterns {
        // check for symmetry in y-direction
        let mut y_symmetry_position = -1_isize; // failed
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
            if is_y_symmetric {
                y_symmetry_position = test_symmetry_position as isize;
            }
        }
        let mut x_symmetry_position = -1_isize;
        for test_symmetry_position in 1..pattern[0].len() {
            let range = if test_symmetry_position <= pattern[0].len() / 2 {
                0..test_symmetry_position
            } else {
                test_symmetry_position..pattern[0].len()
            };
            let mut is_x_symmetric = true;
            for x in range {
                for line in &pattern {
                    if line.chars().nth(x) != line.chars().nth(2*test_symmetry_position - x - 1) {
                        is_x_symmetric = false;
                        break;
                    }
                }
                if !is_x_symmetric {
                    break;
                }
            }
            if is_x_symmetric {
                x_symmetry_position = test_symmetry_position as isize;
            }
        }
        println!("Pattern {:?}", pattern);
        println!("y symmetry at {y_symmetry_position}, x symmetry at {x_symmetry_position}");
        if y_symmetry_position != -1 {
            sum += y_symmetry_position * 100;
        } else if x_symmetry_position != -1 {
            sum += x_symmetry_position;
        }
    }
    println!("Sum: {sum}");
}
