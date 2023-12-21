use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::option::Option;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Pulse {
    High, Low
}

impl Pulse {
    fn index(self: &Self) -> usize {
        match self {
            Pulse::High => 1,
            Pulse::Low => 0
        }
    }
}

trait Module: Debug {
    fn process(self: &mut Self, pulse: Pulse, from: &String) -> Option<Pulse>;
    fn register_input(self: &mut Self, from: &String);
}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
}

impl FlipFlopModule {
    fn new() -> Self {
        FlipFlopModule {
            state: false,
        }
    }
}

impl Module for FlipFlopModule {
    fn register_input(self: &mut Self, from: &String) {
    }

    fn process(self: &mut Self, pulse: Pulse, from: &String) -> Option<Pulse> {
        if pulse == Pulse::High {
            return None;
        }
        self.state = !self.state;
        if self.state {
            return Some(Pulse::High);
        }
        return Some(Pulse::Low);
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    sources: HashMap<String, Pulse>,
}

impl ConjunctionModule {
    fn new() -> Self {
        ConjunctionModule {
            sources: HashMap::new(),
        }
    }
}

impl Module for ConjunctionModule {
    fn register_input(self: &mut Self, from: &String) {
        self.sources.insert(from.to_string(), Pulse::Low);
    }

    fn process(self: &mut Self, pulse: Pulse, from: &String) -> Option<Pulse> {
        self.sources.insert(from.to_string(), pulse);
        for (to, pulse) in self.sources.iter() {
            if *pulse != Pulse::High {
                return Some(Pulse::High);
            }
        }
        Some(Pulse::Low)
    }
}

#[derive(Debug)]
struct SourceModule {
}

impl SourceModule {
    fn new() -> Self {
        SourceModule {
        }
    }
}

impl Module for SourceModule {
    fn register_input(self: &mut Self, from: &String) {
    }

    fn process(self: &mut Self, pulse: Pulse, from: &String) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug)]
struct DefaultModule {
}

impl DefaultModule {
    fn new() -> Self {
        DefaultModule {
        }
    }
}

impl Module for DefaultModule {
    fn register_input(self: &mut Self, from: &String) {
    }

    fn process(self: &mut Self, pulse: Pulse, from: &String) -> Option<Pulse> {
        None
    }
}

fn get_counts(modules: &mut HashMap<String, Box<dyn Module>>, destinations: &HashMap<String, Vec<String>>, notify: &Vec<String>, i: usize, deps: &mut HashMap<String, usize>) -> (usize, usize) {
    let mut counts = vec![0, 0];
    let mut current_pulses: Vec<(String, String, Pulse)> = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
    while !current_pulses.is_empty() {
        let (from, to, pulse) = current_pulses.remove(0);
        if notify.contains(&from) && pulse == Pulse::High {
            deps.insert(from.clone(), i);
            println!("{from} -{pulse:?} -> {to} ({i})");
        }
        if let Some(module) = modules.get_mut(&to) {
            // println!("{from} -{pulse:?} -> {to}");
            if let Some(out_pulse) = module.process(pulse, &from) {
                for destination in &destinations[&to] {
                    current_pulses.push((to.clone(), destination.clone(), out_pulse));
                }
            }
        } else {
            // println!("{from} -{pulse:?} -> {to} (DEAD END)");
        }
        counts[pulse.index()] += 1;
    }
    (counts[0], counts[1])
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcd(x: usize, y: usize) -> usize {
    (x*y)/gcd(x, y)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected exactly one argument");
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let lines: Vec<String> = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("Could not read file {file_path}");
    }).lines().map(String::from).filter(|line| !line.is_empty()).collect();

    let module_regex = Regex::new(r"(?<name>(%|&)?\w+) -> (?<destinations>.*)").unwrap();
    let mut modules = HashMap::<String, Box<dyn Module>>::new();
    let mut destinations = HashMap::<String, Vec<String>>::new();
    for line in lines {
        let result = module_regex.captures(&line).unwrap();
        let mut name = result["name"].to_owned();

        let mut module: Box<dyn Module> = if name.chars().nth(0).unwrap() == '&' {
            name = name[1..].to_string();
            Box::new(ConjunctionModule::new())
        } else if name.chars().nth(0).unwrap() == '%' {
            name = name[1..].to_string();
            Box::new(FlipFlopModule::new())
        } else if name == "broadcaster" {
            Box::new(SourceModule::new())
        } else {
            println!("inserting default module {name};");
            Box::new(DefaultModule::new())
        };
        modules.insert(name.clone(), module);
        
        let dest = result["destinations"].to_owned();
        destinations.insert(name.clone(), dest.split(", ").map(String::from).collect());
    }

    for (name, module) in &mut modules {
        for (destination_name, destination_list) in &destinations {
            if destination_list.contains(&name) {
                module.register_input(destination_name);
            }
        }
    }
    let mut counts = vec![0, 0];
    let mut rx_parent = destinations.iter().find(|(_, dest)| dest.contains(&"rx".to_string())).unwrap().0.clone();
    let mut rx_parent_dependencies = destinations.iter().filter(|(_, dest)| dest.contains(&rx_parent)).map(|(name, _)| name.clone()).collect::<Vec<String>>();
    let mut deps: HashMap<String, usize> = HashMap::new();
    println!("rx parent: {rx_parent}: {rx_parent_dependencies:?}");
    for i in 0..1000 {
        let (a, b) = get_counts(&mut modules, &mut destinations, &rx_parent_dependencies, i, &mut deps);
        counts[0] += a;
        counts[1] += b;
    }
    let mut i = 1000;
    loop {
        get_counts(&mut modules, &mut destinations, &rx_parent_dependencies, i, &mut deps);
        i+=1;
        let mut keep_going = false;
        for dep in &rx_parent_dependencies {
            if let Some(i) = deps.get(dep) {
                if i == &0 {
                    keep_going = true;
                }
            } else {
                keep_going = true;
            }
        }
        if !keep_going {
            break;
        }
    }
    let mut i2 = 1;
    for dep in &rx_parent_dependencies {
        if let Some(i) = deps.get(dep) {
            i2 = lcd(i2, *i+1);
        }
    }
    println!("counts: {:?}: {} -> {}", counts, counts[0] * counts[1], i2);


}