use std::env;
use std::fs;
use regex::Regex;

const NUMBERS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

fn check_row(line_string: &String, line: usize, check_left: bool, check_right: bool, value: &regex::Match, potential_gears: &mut Vec<(usize, usize, usize)>, number_value: usize) -> bool {
    let mut result = false;
    let line_data: Vec<char> = line_string.chars().collect();
    if check_left {
        let character = line_data[value.start() - 1];
        if !NUMBERS.contains(&character) {
            result = true;
        }
        if character == '*' {
            potential_gears.push((line, value.start() - 1, number_value));
        }
    }
    if check_right {
        let character = line_data[value.end()];
        if !NUMBERS.contains(&character) {
            result =  true;
        }
        if character == '*' {
            potential_gears.push((line, value.end(), number_value));
        }
    }
    for j in value.range() {
        let character = line_data[j];
        if !NUMBERS.contains(&character) {
            result = true;
        }
        if character == '*' {
            potential_gears.push((line, j, number_value));
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
    let mut potential_gears: Vec<(usize, usize, usize)> = Vec::new();
    lines.iter().enumerate().for_each(|(i, line)| {
        let check_above = i > 0;
        let check_below = i + 2 < number_of_lines;
        for capture in number.captures_iter(line) {
            let value = capture.get(1).unwrap();
            let number_value = value.as_str().parse::<usize>().unwrap();
            let check_left = value.start() > 0;
            let check_right = value.end() < line.len();
            let mut found_symbol = false;
            if check_above {
                found_symbol = found_symbol || check_row(&lines[i-1], i-1, check_left, check_right, &value, &mut potential_gears, number_value);
            }
            if check_below {
                found_symbol = found_symbol || check_row(&lines[i+1], i+1, check_left, check_right, &value, &mut potential_gears, number_value);
            }
            found_symbol = found_symbol || check_row(&lines[i], i, check_left, check_right, &value, &mut potential_gears, number_value);
            if found_symbol {
                // println!("part: {}: {} in interval ({}, {})", i, number_value, value.start(), value.end());
                sum1 += number_value;
            }
        };
    });
    for (i, (line, column, number_value)) in potential_gears.iter().enumerate() {
        // println!("potential gear at {}, {}: {}", line, column, number_value);
        let mut neighbour_count = 0;
        let mut neighbour_value = 0;
        for (other_line, other_column, other_number_value) in potential_gears.iter().skip(i + 1) {
            if line == other_line && column == other_column {
                neighbour_count += 1;
                neighbour_value = *other_number_value;
            }
        }
        if neighbour_count == 1 {
            println!("found a gear at {}, {}: {}", line, column, neighbour_value * number_value);
            sum2 += neighbour_value * number_value;
        }
    }
    println!("Sum: part numbers: {sum1}, gear ratios: {sum2}, {} potential gears", potential_gears.len());
}
