use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn calculate_modularity(adj_matrix: &Vec<Vec<u32>>, partition: &HashMap<usize, usize>, adj_sums: &Vec<f64>) -> f64 {
    let num_nodes = adj_matrix.len() as f64;
    let mut modularity = 0.0;
    for i in 0..adj_matrix.len() {
        for j in 0..adj_matrix[i].len() {
            if i == j || partition[&i] == partition[&j] {
                modularity += adj_matrix[i][j] as f64 * 2.0 * num_nodes - adj_sums[i] * adj_sums[j];
            }
        }
    }
    modularity
}

fn louvain_two_groups(adj_matrix: &Vec<Vec<u32>>) -> HashMap<usize, usize> {
    let adj_sums = adj_matrix.iter().map(|row| row.iter().sum::<u32>() as f64).collect::<Vec<f64>>();
    let mut partition: HashMap<usize, usize> = (0..adj_matrix.len()).map(|i| (i, 0)).collect(); // Initialize all nodes to group 0
    let mut modularity = calculate_modularity(adj_matrix, &partition, &adj_sums);

    for i in 0..adj_matrix.len() {
        partition.insert(i, 1);

        let new_modularity = calculate_modularity(adj_matrix, &partition, &adj_sums);

        if new_modularity > modularity {
            modularity = new_modularity;
        } else {
            partition.insert(i, 0);
        }
        println!("{}: {}", i, modularity);
    }

    partition
}

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
    for line in &lines {
        let mut parts = line.split(": ");
        let from_name = parts.next().unwrap();
        let from = nodes.iter().position(|node| node == from_name).unwrap();
        for to in parts.next().unwrap().split(" ").map(|s| nodes.iter().position(|node| node == s).unwrap()) {
            adjacencies[from][to] = 1;
            adjacencies[to][from] = 1;
            print!("{from} <-> {to},");
        }
    }
    println!("}}");
    println!("part := FindMinimumCut[g][[2]]");
    println!("Length[part[[1]]] * Length[part[[2]]]");
    // let partition = louvain_two_groups(&adjacencies);
    // println!("Final Partition: {:?}", partition);
    // let mut group_1 = Vec::<usize>::new();
    // let mut group_2 = Vec::<usize>::new();
    // for i in 0..nodes.len() {
    //     if partition[&i] == 0 {
    //         group_1.push(i);
    //     } else {
    //         group_2.push(i);
    //     }
    // }
    // println!("Group 1: {:?}", group_1);
    // println!("Group 2: {:?}", group_2);
    // println!("Part one: {}", group_1.len() * group_2.len())
}
