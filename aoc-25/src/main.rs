use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let mut node_set = HashSet::<String>::new();
    for line in &lines {
        line.replace(":", "").split(" ").map(String::from).for_each(|s| { node_set.insert(s); });
    }
    let nodes = node_set.iter().map(|node| node.to_string()).collect::<Vec<String>>();
    let mut adjacencies = (0..nodes.len()).map(|_| (0..nodes.len()).map(|_| 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    print!("g:= {{");
    let mut started = false;
    for line in &lines {
        let mut parts = line.split(": ");
        let from_name = parts.next().unwrap();
        let from = nodes.iter().position(|node| node == from_name).unwrap();
        for to in parts.next().unwrap().split(" ").map(|s| nodes.iter().position(|node| node == s).unwrap()) {
            adjacencies[from][to] = 1;
            adjacencies[to][from] = 1;
            if !started {
                started = true;
            } else {
                print!(", ");
            }
            print!("{from} <-> {to}");
        }
    }
    println!("}}");
    println!("part := FindMinimumCut[g][[2]]");
    println!("Length[part[[1]]] * Length[part[[2]]]");
}
