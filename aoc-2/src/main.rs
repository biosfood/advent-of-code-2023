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
    let mut sum1 = 0;
    let mut sum2 = 0;
    lines.iter().for_each(|line| {
        let Some(game_result) = game.captures(line) else {
            println!("parse error!");
            return;
        };
        let mut game_was_possible = true;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let groups = &game_result["data"].split(";").collect::<Vec<&str>>();
        for group in groups {
            let values_results = values.captures_iter(group);
            for value_result in values_results {
                let value = value_result["value"].parse::<i32>().unwrap();
                let color = value_result["color"].to_string();
                if color == "red"{
                    if value > 12 {
                        game_was_possible = false;
                    }
                    if value > min_red {
                        min_red = value;
                    }
                }
                if color == "green" {
                    if value > 13 {
                        game_was_possible = false;
                    }
                    if value > min_green {
                        min_green = value;
                    }
                }
                if color == "blue" {
                    if value > 14 {
                        game_was_possible = false;
                    }
                    if value > min_blue {
                        min_blue = value;
                    }
                }
            }
        };
        println!("Game with id {} is possible: {} => red: {}, green: {}, blue: {}", &game_result["id"], &game_result["data"], min_red, min_green, min_blue);
        if game_was_possible {
            sum1 += game_result["id"].parse::<i32>().unwrap();
        }
        sum2 += min_red * min_green * min_blue;
    });
    println!("Sum: part 1: {}, part 2: {}", sum1, sum2);
}