use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Comparison {
    Greater, Less
}

impl Comparison {
    fn from(c: char) -> Comparison {
        match c {
            '<' => Comparison::Less,
            '>' => Comparison::Greater,
            _ => panic!("cannot read comparison {}", c)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Parameter {
    X, M, A, S
}

impl Parameter {
    fn from(c: char) -> Parameter {
        match c {
            'x' => Parameter::X,
            'm' => Parameter::M,
            'a' => Parameter::A,
            's' => Parameter::S,
            _ => panic!("cannot make parameter {}", c)
        }
    }
}

struct Workflow {
    rules: Vec<(Parameter, Comparison, usize, String)>,
    default_destination: String,
}

type Part = HashMap<Parameter, usize>;

impl Workflow {
    fn new(data: &str) -> Workflow {
        let regex = Regex::new(r"(?<groups>(\w(<|>)\d+:\w+,)*)(?<default_destination>\w+)").unwrap();
        let result = regex.captures(data).unwrap();
        let regex = Regex::new(r"(?<parameter>\w)(?<comparison>(<|>))(?<value>\d+):(?<target>\w+),").unwrap();
        let mut rules = Vec::<(Parameter, Comparison, usize, String)>::new();
        for result in regex.captures_iter(&result["groups"]) {
            let parameter = Parameter::from(result["parameter"].chars().next().unwrap());
            let comparison = Comparison::from(result["comparison"].chars().next().unwrap());
            let value = result["value"].parse::<usize>().unwrap();
            let target = &result["target"];
            rules.push((parameter, comparison, value, target.to_string()));
        }
        Workflow {
            rules,
            default_destination: result["default_destination"].to_string(),
        }
    }

    fn process(self: &Workflow, part: &Part) -> &String {
        for (parameter, comparison, value, result) in self.rules.iter() {
            let part_value = part[&parameter];
            if *comparison == Comparison::Greater && part_value > *value {
                return result;
            }
            if *comparison == Comparison::Less && part_value < *value {
                return result;
            }
        }
        return &self.default_destination;
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

    let mut workflows = HashMap::<String, Workflow>::new();
    let mut i = 0;
    while !lines[i].is_empty() {
        let parts: Vec<&str> = lines[i].split("{").collect();
        let value = Workflow::new(parts[1].split("}").next().unwrap());
        workflows.insert(parts[0].to_string(), value);
        i += 1;
    }
    i += 1;
    let inner_regex = Regex::new(r"\{(?<content>.*)\}").unwrap();
    let parameter_regex = Regex::new(r"(?<parameter>\w)=(?<value>\d+)").unwrap();
    let mut sum = 0;
    while i < lines.len() {
        if lines[i].is_empty() {
            continue;
        }
        let mut part: Part = HashMap::new();
        let inner = inner_regex.captures(lines[i].as_str()).unwrap();
        for result in parameter_regex.captures_iter(&inner["content"]) {
            part.insert(Parameter::from(result["parameter"].chars().next().unwrap()), result["value"].to_string().parse::<usize>().unwrap());
        }
        let mut current_workflow = &"in".to_string();
        while !(current_workflow == "A" || current_workflow == "R") {
            current_workflow = workflows[current_workflow].process(&part);
        }
        if current_workflow == "A" {
            println!("accepted part {part:?}");
            sum += part.into_values().sum::<usize>();
        } else {
            println!("rejected part {part:?}");
        }
        i+=1;
    }
    println!("Sum part one: {sum}");
}
