use std::env;
use std::fs;

trait Vector {
    fn parse(line: &str) -> Self;
    fn add(&self, other: &Self) -> Self;
}

type Position = [isize; 3];

impl Vector for Position {
    fn parse(line: &str) -> Self {
        let parts = line.split(",").collect::<Vec<&str>>();
        [ parts[0].parse::<isize>().unwrap(), parts[1].parse::<isize>().unwrap(), parts[2].parse::<isize>().unwrap() ]
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = [0; 3];
        for i in 0..3 {
            result[i] = self[i] + other[i];
        }
        result
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Block {
    min: Position,
    max: Position,
}

impl Block {
    fn new(line: &String) -> Self {
        let parts = line.split("~").collect::<Vec<&str>>();
        Block { min: Position::parse(parts[0]), max: Position::parse(parts[1]) }
    }

    fn intersects(&self, other: &Block) -> bool {
        for i in 0..3 {
            if self.min[i] > other.max[i] || self.max[i] < other.min[i] {
                return false;
            }
        }
        true
    }

    fn offset(&mut self, x: isize, y: isize, z: isize) {
        self.min = self.min.add(&[ x, y, z ]);
        self.max = self.max.add(&[ x, y, z ]);
    }
}

fn fall(blocks: &mut Vec<Block>) {
    let mut keep_going = true;
    while keep_going {
        keep_going = false;
        for i in 0..blocks.len() {
            if blocks[i].min[2] == 1 {
                continue;
            }
            blocks[i].offset(0, 0, -1);
            let mut had_overlap = false;
            for j in 0..blocks.len() {
                if i == j {
                    continue;
                }
                if blocks[i].intersects(&blocks[j]) {
                    blocks[i].offset(0, 0, 1);
                    had_overlap = true;
                    break;
                }
            }
            if !had_overlap {
                keep_going = true;
            }
        }
    }
}

fn is_stable(blocks: &mut Vec<Block>, block_index: usize) -> bool {
    for i in 0..blocks.len() {
        if i == block_index {
            continue;
        }
        if blocks[i].min[2] == 1 {
            continue;
        }
        blocks[i].offset(0, 0, -1);
        let mut had_overlap = false;
        for j in 0..blocks.len() {
            if j == i || j == block_index {
                continue;
            }
            if blocks[i].intersects(&blocks[j]) {
                had_overlap = true;
            }
        }
        blocks[i].offset(0, 0, 1);
        if !had_overlap {
            return false;
        }
    }
    true
}

fn get_falling_blocks(blocks: &Vec<Block>, block_index: usize) -> usize {
    let mut new_blocks = blocks.iter().enumerate().filter(|(i, _)| *i != block_index).map(|(_, block)| block.clone()).collect::<Vec<Block>>();
    fall(&mut new_blocks);
    let mut displacements = 0;
    for i in 0..blocks.len() {
        if i == block_index {
            continue;
        }
        let old_block = blocks[i];
        let new_block = if i < block_index {
            new_blocks[i]
        } else {
            new_blocks[i-1]
        };
        if old_block != new_block {
            displacements += 1;
        }
    }
    displacements
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let mut blocks = lines.iter().map(Block::new).collect::<Vec<Block>>();
    fall(&mut blocks);
    println!("blocks: {blocks:?}");
    let mut stable_count = 0;
    let mut falling_count = 0;
    for i in 0..blocks.len() {
        if is_stable(&mut blocks, i) {
            println!("Block {} is stable", i);
            stable_count += 1;
        } else {
            let displacements = get_falling_blocks(&blocks, i);
            println!("removing block {} will cause {} blocks to fall", i, displacements);
            falling_count += displacements;
        }
    }
    println!("Stable blocks: {}, falling blocks: {}", stable_count, falling_count);
}
