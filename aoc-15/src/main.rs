use std::env;
use std::fs;

fn compute_hash(data: &String) -> usize {
    let mut result = 0;
    for c in data.chars() {
        result += c as usize;
        result *= 17;
        result %= 256;
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
    let entries = lines[0].split(",").map(String::from).collect::<Vec<String>>();
    let hashes = entries.iter().map(compute_hash).collect::<Vec<usize>>();
    let sum = hashes.iter().sum::<usize>();
    println!("Entries: {:?}, Hashes: {:?}, Sum: {}", entries, hashes, sum);
}
