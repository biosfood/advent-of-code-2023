use std::env;
use std::fs;
use regex::Regex;

fn compute_derivatives(history: Vec<isize>) -> Vec<Vec<isize>> {
    let mut result: Vec<Vec<isize>> = Vec::new();
    result.push(history);

    while result[result.len()-1].iter().any(|x| *x != 0) {
        let previous = &result[result.len()-1];
        let mut next = Vec::<isize>::new();
        for i in 0..(previous.len()-1) {
            next.push(previous[i+1] - previous[i]);
        }
        result.push(next);
    }
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();
    let number_regex = Regex::new(r"-?\d+").unwrap();
    let histories = lines.iter().filter(|line| !line.is_empty()). map(|line| {
        let result = number_regex.captures_iter(line).map(|result| result[0].parse::<isize>().unwrap()).collect::<Vec<isize>>();
        result
    });
    let derivatives = histories.map(compute_derivatives).collect::<Vec<Vec<Vec<isize>>>>();
    let mut sum_forward = 0;
    let mut sum_backwards = 0;
    for value in derivatives {
        let mut forward_derivative = 0;
        let mut backward_derivative = 0;
        for i in (0..value.len()).rev() {
            forward_derivative = value[i][value[i].len() - 1] + forward_derivative;
            backward_derivative = value[i][0] - backward_derivative;
        }
        sum_forward += forward_derivative;
        sum_backwards += backward_derivative;
        println!("before first: {}, next value: {}", backward_derivative, forward_derivative);
    }
    println!("sum forward: {}, sum backwards: {}", sum_forward, sum_backwards);
}