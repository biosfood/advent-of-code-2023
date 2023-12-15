use std::env;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lens {
    label: String,
    strength: usize,
}

impl Lens {
    fn new(label: String, strength: usize) -> Lens {
        Lens {
            label,
            strength,
        }
    }
}

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
    let sum_1 = hashes.iter().sum::<usize>();
    let instructions = entries.iter().zip(hashes.iter()).map(|(x, y)| (x, *y)).collect::<Vec<(&String, usize)>>();
    let mut boxes = (0..256).map(|_| Vec::<Lens>::new()).collect::<Vec<Vec<Lens>>>();
    for (instruction, index) in instructions {
        if instruction.contains('-') {
            let name_to_delete = instruction.split('-').next().unwrap().to_string();
            let index = compute_hash(&name_to_delete);
            let mut lenses = boxes[index].clone();
            lenses.retain(|lens| lens.label != name_to_delete);
            boxes[index] = lenses;
        } else if instruction.contains('=') {
            let new_name = instruction.split('=').next().unwrap().to_string();
            let new_strength = instruction.split('=').last().unwrap().parse::<usize>().unwrap();
            let index = compute_hash(&new_name);
            let mut lenses = boxes[index].clone();
            if let Some(lens) = lenses.iter().find(|lens| lens.label == new_name) {
                let lens_index = lenses.iter().position(|x| x == lens).unwrap();
                lenses[lens_index].strength = new_strength;
            } else {
                lenses.push(Lens::new(new_name, new_strength));
            }
            boxes[index] = lenses;
        } else {
            panic!("Unknown instruction: {}", instruction);
        }
    }
    println!("Boxes: {:?}", boxes);
    let sum_2 = boxes.iter().enumerate().map(|(box_index, lenses)| 
        lenses.iter().enumerate().map(|(lens_position, lens)| (1+box_index)*(1+lens_position)*(lens.strength)).sum::<usize>()
    ).sum::<usize>();
    println!("Sum of hashes: {}, total focussing power: {}", sum_1, sum_2); 
}
