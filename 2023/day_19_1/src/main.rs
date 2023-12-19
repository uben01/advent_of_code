use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;

    let mut workflow_setup = true;
    let mut workflows: HashMap<String, (Vec<(String, char, u64, String)>, String)> = HashMap::new();
    for line in buff.lines() {
        let line = line?;
        if line.is_empty() {
            workflow_setup = false;

            continue;
        }
        if workflow_setup {
            let workflow = parse_workflow(line);
            workflows.insert(workflow.0, (workflow.1, workflow.2));
        } else {
            let parts = &parse_parts(line);

            let result = pass_through_workflows(&workflows, &"in".to_string(), parts);
            if result {
                sum += parts.iter().map(|(_, value)| *value).sum::<u64>();
            }
        }

    }

    println!("Sum: {}", sum);

    return Ok(());
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

fn parse_parts(line: String) -> HashMap<String, u64> {
    let line = line.replace('}', "");
    let line = line.replace('{', "");

    let split = line.split(',');

    let mut parts = HashMap::new();
    for element in split.into_iter() {
        let mut split = element.trim().split('=');
        let left = split.next().unwrap().to_string();
        let right: u64 = split.next().unwrap().parse().unwrap();

        parts.insert(left, right);
    }

    parts
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

fn pass_through_workflows(
    workflows: &HashMap<String, (Vec<(String, char, u64, String)>, String)>,
    current_workflow: &String,
    parts: &HashMap<String, u64>,
) -> bool {
    let current_workflow = workflows.get(current_workflow).unwrap();
    for (workflow_variable_name, workflow_operator, workflow_value, to_workflow_name) in current_workflow.0.iter() {
        let part_value = parts.get(workflow_variable_name).unwrap();

        if workflow_operator == &'<' {
            if part_value < workflow_value {
                if let Some(result) = decide_pass_reject(&to_workflow_name) {
                    return result;
                }

                return pass_through_workflows(workflows, to_workflow_name, parts);
            }
        } else {
            if part_value > workflow_value {
                if let Some(result) = decide_pass_reject(&to_workflow_name) {
                    return result;
                }

                return pass_through_workflows(workflows, to_workflow_name, parts);
            }
        }

    }

    let else_task = &current_workflow.1.clone();
    if let Some(result) = decide_pass_reject(else_task) {
        return result;
    }

    return pass_through_workflows(workflows, else_task, parts);
}

fn decide_pass_reject(task: &String) -> Option<bool> {
    if task == "A" {
        return Some(true);
    } else if task == "R" {
        return Some(false);
    }

    None
}