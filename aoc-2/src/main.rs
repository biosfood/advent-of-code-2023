use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let game: Regex = Regex::new(r"Game (?<id>\d+): (?<data>.*)").unwrap();
    let values: Regex = Regex::new(r"(?<value>\d+) (?<color>\w+)").unwrap();
    let lines:Vec<String> = fs::read_to_string(file_path)
        .unwrap().lines().map(String::from).collect();
    let mut sum = 0;
    lines.iter().for_each(|line| {
        let Some(game_result) = game.captures(line) else {
            println!("parse error!");
            return;
        };
        let groups = &game_result["data"].split(";").collect::<Vec<&str>>();
        for group in groups {
            let values_results = values.captures_iter(group);
            for value_result in values_results {
                let value = value_result["value"].parse::<i32>().unwrap();
                let color = value_result["color"].to_string();
                if color == "red" && value > 12 {
                    return;
                }
                if color == "green" && value > 13 {
                    return;
                }
                if color == "blue" && value > 14 {
                    return;
                }
            }
        };
        println!("Game with id {} is possible: {}", &game_result["id"], &game_result["data"]);
        sum += game_result["id"].parse::<i32>().unwrap();
    });
    println!("Sum: {}", sum);
}