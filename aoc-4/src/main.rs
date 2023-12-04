use std::env;
use std::fs;
use std::iter;
use regex::Regex;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();


    let card: Regex = Regex::new(r"Card\s*(\d+): ([\d| ]*)\|(.*)").unwrap();
    let number: Regex = Regex::new(r"\d+").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    let number_of_original_cards = lines.len();
    let mut number_of_cards: Vec<usize> = iter::repeat(1).take(number_of_original_cards).collect();
    for line in lines {
        let Some(card_result) = card.captures(line.as_str()) else {
            panic!("cannot parse line   {line}");
        };
        let winning_numbers = number.captures_iter(&card_result[2]).map(|x| x[0].parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let values = number.captures_iter(&card_result[3]).map(|x| x[0].parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut number_of_matches = 0;
        let card_number = &card_result[1].parse::<usize>().unwrap();
        for value in &values {
            if winning_numbers.contains(&value) {
                number_of_matches += 1;
            }
        }
        let points = if number_of_matches == 0 { 0 } else { 1 << (number_of_matches - 1) };
        println!("card {}: winning: {}, values: {}, {} matches => {}", &card_result[1], winning_numbers.len(), values.len(), number_of_matches, points);
        sum1 += points;
        for i in *card_number..min(card_number+number_of_matches, number_of_original_cards) {
            number_of_cards[i] += number_of_cards[card_number - 1];
        }
    }
    for cards in number_of_cards {
        println!("{cards}");
        sum2 += cards;
    }
    println!("sum of points: {}, number of cards: {}", sum1, sum2);
}
