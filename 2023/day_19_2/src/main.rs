use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut workflows: HashMap<String, (Vec<(String, char, u64, String)>, String)> = HashMap::new();
    for line in buff.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let workflow = parse_workflow(line);
        workflows.insert(workflow.0, (workflow.1, workflow.2));
    }
    let mut parts: HashMap<String, (u64, u64)> = HashMap::new();
    for part in ["x", "m", "a", "s"] {
        parts.insert(part.to_string(), (1, 4000));
    }

    let sum = pass_through_workflows(&workflows, &"in".to_string(), &mut parts);

    println!("Sum: {}", sum);

    return Ok(());
}

fn pass_through_workflows(
    workflows: &HashMap<String, (Vec<(String, char, u64, String)>, String)>,
    current_workflow_name: &String,
    parts: &mut HashMap<String, (u64, u64)>,
) -> u64 {
    if current_workflow_name == "A" {
        let mut product = 1;

        for part in parts.values() {
            let partial = part.1 - part.0 + 1;
            product *= partial;
        }

        return product;
    } else if current_workflow_name == "R" {
        return 0;
    }


    let current_workflow = workflows.get(current_workflow_name).unwrap();
    let mut sum = 0;

    for (workflow_variable_name, workflow_operator, workflow_value, to_workflow_name) in current_workflow.0.iter() {
        let part_range = parts.get(workflow_variable_name).unwrap();

        if workflow_operator == &'<' {
            let accepting_range = (part_range.0, u64::min(part_range.1, *workflow_value) - 1);
            if accepting_range.0 < accepting_range.1 {
                let mut parts = parts.clone();
                parts.insert(workflow_variable_name.to_string(), accepting_range);

                sum += pass_through_workflows(workflows, to_workflow_name, &mut parts);
            }
            let rest_of_the_range = (u64::min(part_range.1, *workflow_value), part_range.1);
            if rest_of_the_range.0 > rest_of_the_range.1 {
                break;
            }
            parts.insert(workflow_variable_name.to_string(), rest_of_the_range);
        } else {
            let accepting_range = (u64::min(part_range.1, *workflow_value) + 1, part_range.1);
            if accepting_range.0 < accepting_range.1 {
                let mut parts = parts.clone();
                parts.insert(workflow_variable_name.to_string(), accepting_range);

                sum += pass_through_workflows(workflows, to_workflow_name, &mut parts);
            }
            let rest_of_the_range = (part_range.0, u64::min(part_range.1, *workflow_value));
            if rest_of_the_range.0 > rest_of_the_range.1 {
                break;
            }
            parts.insert(workflow_variable_name.to_string(), rest_of_the_range);
        }
    }

    return sum + pass_through_workflows(workflows, &current_workflow.1, parts);
}

fn parse_workflow(line: String) -> (String, Vec<(String, char, u64, String)>, String) {
    let line = line.replace('}', "");
    let mut split = line.split('{');

    let name= split.next().unwrap();
    let split: Vec<&str> = split.next().unwrap().split(',').collect();

    let mut else_task = "".to_string();
    let mut tasks = vec![];
    for task in split {
        if !task.contains(':') {
            else_task = task.to_string();
            break;
        }

        let mut split = task.split(':');
        let condition = split.next().unwrap();
        let condition = parse_condition(condition.to_string());

        let result = split.next().unwrap();

        tasks.push((condition.0, condition.1, condition.2, result.trim().to_string()));
    }

    (name.trim().to_string(), tasks, else_task)
}

fn parse_condition(condition: String) -> (String, char, u64) {
    let mut operator = '>';
    if condition.contains('<') {
        operator = '<';
    }

    let mut split = condition.split(operator);
    let left = split.next().unwrap().trim().to_string();
    let right: u64 = split.next().unwrap().trim().parse().unwrap();

    (left, operator, right)
}