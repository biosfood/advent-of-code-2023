use std::env;
use std::fs;
use regex::Regex;

const NUMBERS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

fn check_row(lines: &Vec<String>, line: usize, check_left: bool, check_right: bool, value: &regex::Match) -> bool {
    let mut result = false;
    if check_left {
        if !NUMBERS.contains(&lines[line].chars().nth(value.start() - 1).unwrap()) {
            result = true;
        }
    }
    if check_right {
        if !NUMBERS.contains(&lines[line].chars().nth(value.end()).unwrap()) {
            result =  true;
        }
    }
    for j in value.start()..value.end() {
        if !NUMBERS.contains(&lines[line].chars().nth(j).unwrap()) {
            result = true;
        }
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();
    let number_of_lines = lines.len();
    let number: Regex = Regex::new(r"(\d+)").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    lines.iter().enumerate().for_each(|(i, line)| {
        let check_above = i > 0;
        let check_below = i < number_of_lines - 1;
        for capture in number.captures_iter(line) {
            let value = capture.get(1).unwrap();
            let number_value = value.as_str().parse::<i32>().unwrap();
            let check_left = value.start() > 0;
            let check_right = value.end()+1 < line.len();
            let mut found_symbol = false;
            if check_above {
                found_symbol = found_symbol || check_row(&lines, i-1, check_left, check_right, &value);
            }
            if check_below {
                found_symbol = found_symbol || check_row(&lines, i+1, check_left, check_right, &value);
            }
            found_symbol = found_symbol || check_row(&lines, i, check_left, check_right, &value);
            if found_symbol {
                println!("{}: {}", i, number_value);
                sum1 += number_value;
            }
        };
    });
    println!("Sum: part numbers: {sum1}, gear ratios: {sum2}");
}
