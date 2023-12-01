use std::env;
use std::fs;
use regex::Regex;

fn stage1() {
    let digit: Regex = Regex::new(r"\d").unwrap();
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines:Vec<String> = fs::read_to_string(file_path)
        .unwrap().lines().map(String::from).collect();
    
    let mut sum = 0;

    lines.iter().for_each(|line| {
        let result: Vec<i32> = digit.find_iter(line).filter_map(|digits| digits.as_str().parse::<i32>().ok()).collect();
        println!("{} -> {}{}", line, &result[0], &result[result.len() - 1]);
        sum += &result[0] * 10 + &result[result.len()- 1];
    });
    println!("Sum: {}", sum);
}

fn main() {
    let digit: Regex = Regex::new(r"(\d)").unwrap();
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines:Vec<String> = fs::read_to_string(file_path)
        .unwrap().lines().map(String::from).collect();
    
    let mut sum = 0;

    let replacements = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    lines.iter().for_each(|line| {
        let mut processed = line.to_lowercase().to_string();
        for (from, to) in replacements.iter() {
            processed = processed.replace(from, to);
        }
        let numbers: Vec<i32> = digit.find_iter(processed.as_str()).map(|result| result.as_str().parse::<i32>().unwrap()).collect();
        if numbers.len() == 0 {
            println!("{} -> 0", line);
            return;
        }
        println!("{} -> {}{}", line, &numbers[0], &numbers[numbers.len() - 1]);
        sum += &numbers[0] * 10 + &numbers[numbers.len()- 1];
    });
    println!("Sum: {}", sum);

}