use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

const PARAMETERS: [Parameter; 4] = [Parameter::X, Parameter::M, Parameter::A, Parameter::S];

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

#[derive(Eq, PartialEq, Hash, Debug)]
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

    fn modify_range(self: &Workflow, range: &mut HashMap<Parameter, (usize, usize)>, origin: usize) {
        let mut i = 0;
        println!("origin: {origin}");
        while i < self.rules.len() && i < origin {
            let (parameter, comparison, value, _) = self.rules[i];
            let (mut min, mut max) = range[&parameter];
            if comparison == Comparison::Greater {
                max = max.min(value);
            } else {
                min = min.max(value);
            }
            range.insert(parameter, (min, max));
            i+=1;
        }
        if origin < self.rules.len() {
            let (parameter, comparison, value, _) = self.rules[origin];
            let (mut min, mut max) = range[&parameter];
            if comparison == Comparison::Greater {
                min = min.max(value + 1);
            } else {
                max = max.min(value - 1);
            }
            range.insert(parameter, (min, max));
        }
    }

    fn process_range(self: &Workflow, range: &mut HashMap<Parameter, (usize, usize)>, workflows: &HashMap::<String, Workflow>, origin: usize) -> () {
        self.modify_range(range, origin);
        for (name, other) in workflows {
            if self == other && name != "in" {
                for (_, workflow) in workflows {
                    if &workflow.default_destination == name {
                        workflow.process_range(range, workflows, 100);
                        return;
                    }
                    if let Some(target) = workflow.rules.iter().enumerate().find(|(_, (_, _, _, result))| result == name) {
                        workflow.process_range(range, workflows, target.0);
                    }
                }
            }
        }
    }
}

fn fill_range(range: &mut HashMap<Parameter, (usize, usize)>) {
    range.insert(Parameter::X, (1, 4000));
    range.insert(Parameter::M, (1, 4000));
    range.insert(Parameter::A, (1, 4000));
    range.insert(Parameter::S, (1, 4000));
}

fn has_intersection(range1: &HashMap<Parameter, (usize, usize)>, range2: &HashMap<Parameter, (usize, usize)>) -> bool {
    for parameter in PARAMETERS {
        let (min1, max1) = range1[&parameter];
        let (min2, max2) = range2[&parameter];
        if min1 >= max2 || min2 >= max1 {
            return false;
        }
    }
    return true;
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

    let mut ranges_to_insert = Vec::<HashMap<Parameter, (usize, usize)>>::new();

    for workflow in workflows.values() {
        if workflow.default_destination == "A" {
            let mut range = HashMap::<Parameter, (usize, usize)>::new();
            fill_range(&mut range);
            workflow.process_range(&mut range, &workflows, 100);
            println!("new range: {range:?}");
            ranges_to_insert.push(range);
        }
        for target in workflow.rules.iter().enumerate().filter(|(_, (_, _, _, result))| result == "A") {
            let mut range = HashMap::<Parameter, (usize, usize)>::new();
            fill_range(&mut range);
            workflow.process_range(&mut range, &workflows, target.0);
            println!("new range: {range:?}");
            ranges_to_insert.push(range);
        }
    }
    let mut current_ranges = Vec::<HashMap<Parameter, (usize, usize)>>::new();
    while !ranges_to_insert.is_empty() {
        let range = &ranges_to_insert[0].clone();
        ranges_to_insert.remove(0);

        let mut still_insert = true;
        for other in &current_ranges {
            if !has_intersection(&range, &other) {
                continue;
            }
            still_insert = false;
            println!("intersection: {range:?} and {other:?}");
            for parameter in PARAMETERS {
                // box A: range, box B: other
                let intersection_start = range[&parameter].0.max(other[&parameter].0);
                let intersection_end = range[&parameter].1.min(other[&parameter].1);

                if intersection_start < intersection_end {
                    let mut new_range = range.clone();
                    let mut r = new_range[&parameter];
                    r.1 = intersection_start;
                    new_range.insert(parameter, r);
                    ranges_to_insert.push(new_range);

                    let mut new_range = range.clone();
                    let mut r = new_range[&parameter];
                    r.0 = intersection_end;
                    new_range.insert(parameter, r);
                    ranges_to_insert.push(new_range);
                }
            }
            break;
        }
        for parameter in PARAMETERS {
            if range[&parameter].0 > range[&parameter].1 {
                // invalid range
                still_insert = false;
            }
        }
        if still_insert {
            current_ranges.push(range.clone());
        }
    }
    let mut sum2 = 0;
    for range in &current_ranges {
        let mut partial_sum = 1;
        for parameter in PARAMETERS {
            partial_sum *= range[&parameter].1 - range[&parameter].0 + 1;
        }
        // partial_sum *= (range[&Parameter::X].0 + range[&Parameter::X].1 + 
        //     range[&Parameter::M].0 + range[&Parameter::M].1 + 
        //     range[&Parameter::A].0 + range[&Parameter::A].1 + 
        //     range[&Parameter::S].0 + range[&Parameter::S].1) as f64;
        println!("range: {range:?} -> {partial_sum}");
        sum2 += partial_sum as usize;
    }
    println!("Sum part one: {sum}, part two: {sum2}");
}
