use std::env;
use std::fs;
use regex::Regex;


fn main() {
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
