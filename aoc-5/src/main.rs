use std::env;
use std::fs;
use regex::Regex;

fn translate(key: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination_start, source_start, length) in map {
        if (*source_start..(source_start + length)).contains(&key) {
            return destination_start + (key - source_start);
        }
    }
    return key;
}

fn translate_range(key_range: &(u64, u64), map: &[(u64, u64, u64)], output: &mut Vec<(u64, u64)>) {
    if key_range.1 == 0 {
        // nothing to do if the key range is empty
        return;
    }
    if map.len() == 0 {
        output.push(*key_range);
        println!("({}, {}) -> ({}, {}): ID", key_range.0, key_range.1, key_range.0, key_range.1);
        return;
    }
    let (key_start, key_length) = *key_range;
    let key_end = key_start + key_length;
    let (target_start, source_start, map_length) = map[0];
    let target_end = target_start + map_length;
    let source_end = source_start + map_length;
    if source_end < key_start || key_end < source_start {
        // no overlap
        translate_range(key_range, &map[1..], output);
        return;
    }
    if key_start <= source_start && source_end <= key_end {
        // complete overlap
        // output.push((key_start ));
        // return;
    }
    let start = if source_start <= key_start {
        target_start + key_start - source_start
    } else {
        target_start
    };
    let end = if source_end >= key_end {
        target_start + key_start + key_length - source_start
    } else {
        target_end
    };
    if start == end {
        // something went wrong here
        translate_range(key_range, &map[1..], output);
        return;
    }
    println!("({}, {}), ({}, {})-> ({}, {})", key_start, key_length, source_start, map_length, start, end-start);
    output.push((start, end - start));
    if source_start > key_start {
        translate_range(&(key_start, source_start - key_start - 1), &map[1..], output);
    }
    if source_end < key_end {
        translate_range(&(source_end, key_end - source_end), &map[1..], output);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).collect();

    let numbers: Regex = Regex::new(r"\d+").unwrap();
    let seeds = numbers.captures_iter(lines[0].as_str()).map(|x| x.get(0).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let seed_ranges = seeds.chunks(2).map(|x| (x[0], x[1])).collect::<Vec<(u64, u64)>>();

    let mut steps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut current_map: Vec<(u64, u64, u64)> = Vec::new();

    for line in &lines[1..] {
        if line.len() == 0 {
            continue;
        }
        if numbers.captures_iter(line.as_str()).count() == 0 {
            if current_map.len() > 0 {
                println!("collected {} entries", current_map.len());
                steps.push(current_map.clone());
            }
            println!("reading {}", line);
            current_map = Vec::new();
            continue;
        }
        let numbers_in_line = numbers.captures_iter(line.as_str()).map(|x| x.get(0).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        current_map.push((numbers_in_line[0], numbers_in_line[1], numbers_in_line[2]));
    }
    if current_map.len() > 0 {
        println!("collected {} entries", current_map.len());
        steps.push(current_map.clone());
    }
    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut value = seed;
        for step in &steps {
            value = translate(value, step);
        }
        if value < min_location {
            min_location = value;
        }
    }

    let mut min_from_range = u64::MAX;
    for seed_range in seed_ranges {
        println!("seed range: {:?}", seed_range);
        let mut current_ranges: Vec<(u64, u64)> = Vec::new();
        current_ranges.push(seed_range);

        for step in &steps {
            println!("step: {:?}", step);
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            for range in &current_ranges {
                translate_range(range, &step, &mut new_ranges);
            }
            if new_ranges.len() == 0 {
                println!("problem here!");
                break;
            }
            current_ranges = new_ranges;
        }
        print!("End ranges: ");
        for (start, size) in current_ranges {
            if start < min_from_range {
                min_from_range = start;
            }
            print!("({}, {}), ", start, size);
        }
        println!("");
    }
    println!("min: {}, {}", min_location, min_from_range);
}
