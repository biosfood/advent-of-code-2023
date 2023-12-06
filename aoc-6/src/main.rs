use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let numbers: Regex = Regex::new(r"\d+").unwrap();
    let times = numbers.captures_iter(lines[0].as_str()).map(|x| x[0].parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let distances = numbers.captures_iter(lines[1].as_str()).map(|x| x[0].parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let scores = times.iter().zip(distances.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(usize, usize)>>();
    let mut product = 1;
    for (time, target_distance) in scores {
        let mut number_of_winning_times = 0;
        for time_accelerating in 0..time {
            let speed = time_accelerating;
            let time_remaining = time - time_accelerating;
            let distance = speed * time_remaining;
            if distance > target_distance {
                number_of_winning_times += 1;
            }
        }
        product *= number_of_winning_times;
    }
    let combined_time = numbers.captures(lines[0].as_str().replace(" ", "").as_str()).unwrap()[0].parse::<usize>().unwrap();
    let combined_distance = numbers.captures(lines[1].as_str().replace(" ", "").as_str()).unwrap()[0].parse::<usize>().unwrap();
    let mut number_of_winning_times = 0;
    for time_accelerating in 0..combined_time {
        let speed = time_accelerating;
        let time_remaining = combined_time - time_accelerating;
        let distance = speed * time_remaining;
        if distance > combined_distance {
            number_of_winning_times += 1;
        }
    }
    println!("{product}, {combined_time} -> {combined_distance} => {number_of_winning_times}");
}