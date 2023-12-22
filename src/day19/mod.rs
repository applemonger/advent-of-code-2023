use std::collections::HashMap;

use aocd::*;
use regex::Regex;

#[derive(Debug)]
pub struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn from_str(s: &'a str) -> (&'a str, Workflow<'a>) {
        // Get id
        // Alphabetical letters preceding the first curly bracket
        let id_regex = Regex::new(r"^([a-z]+)\{").unwrap();
        let id = id_regex.captures(s).unwrap().get(1).unwrap().as_str();

        // Get rules
        // Chunk of text between curly brackets
        let rules_regex = Regex::new(r"\{(.*)\}").unwrap();
        let rules_str = rules_regex.captures(s).unwrap().get(1).unwrap().as_str();

        // List of rules can be split by commas
        let rules: Vec<Rule> = rules_str.split(',').map(Rule::from_str).collect();

        (id, Workflow { rules })
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    category: Option<char>,
    operator: Option<char>,
    threshold: Option<u32>,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Rule<'a> {
        // The rule condition, if there is one, is defined by the chunk of text before
        // the colon.
        let condition_regex = Regex::new(r"(.+):").unwrap();

        // If our regex finds a condition in the string...
        if let Some(captures) = condition_regex.captures(s) {
            // Get the characters of the condition text
            let mut condition = captures.get(1).unwrap().as_str().chars();
            // The category (x, m, a, s) is the first character
            let category = Some(condition.next().unwrap());
            // The operator (<, >) is the second character
            let operator = Some(condition.next().unwrap());
            // The threshold number is the rest of the condition text
            let threshold = Some(condition.collect::<String>().parse::<u32>().unwrap());
            // Meanwhile, the target (workflow id, Accepted, or Rejected), is the text
            // after the colon
            let target_regex = Regex::new(r":(\w+)$").unwrap();
            let target = target_regex.captures(s).unwrap().get(1).unwrap().as_str();
            Rule {
                category,
                operator,
                threshold,
                target,
            }
        } else {
            // If our regex does not find a condition in the string, then the rule simply
            // points to a target (another workflow, Accepted, or Rejected)
            Rule {
                category: None,
                operator: None,
                threshold: None,
                target: s,
            }
        }
    }

    // Rules operate on parts; either it sends the part to a target (workflow, A, R) or
    // the part moves on to the next rule (None)
    fn operate(&self, part: &Part) -> Option<&str> {
        if let Some(category) = self.category {
            let rating = *part.ratings.get(&category).unwrap();
            let threshold_accepted = if self.operator.unwrap() == '>' {
                rating > self.threshold.unwrap()
            } else {
                rating < self.threshold.unwrap()
            };
            if threshold_accepted {
                Some(self.target)
            } else {
                None
            }
        } else {
            Some(self.target)
        }
    }
}

pub struct Part {
    ratings: HashMap<char, u32>,
}

impl Part {
    fn from_str(s: &str) -> Part {
        let x_regex = Regex::new(r"x=(\d+)").unwrap();
        let x = x_regex
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let m_regex = Regex::new(r"m=(\d+)").unwrap();
        let m = m_regex
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let a_regex = Regex::new(r"a=(\d+)").unwrap();
        let a = a_regex
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let s_regex = Regex::new(r"s=(\d+)").unwrap();
        let s = s_regex
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let ratings = HashMap::from([('x', x), ('m', m), ('a', a), ('s', s)]);
        Part { ratings }
    }

    fn work(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut workflow_id = "in";
        'working: loop {
            let workflow = workflows.get(workflow_id).unwrap();
            for rule in workflow.rules.iter() {
                if let Some(target) = rule.operate(self) {
                    if target == "A" {
                        return true;
                    } else if target == "R" {
                        return false;
                    } else {
                        workflow_id = target;
                        continue 'working;
                    }
                }
            }
        }
    }

    fn total_rating(&self) -> u32 {
        self.ratings.values().sum()
    }
}

#[aocd(2023, 19)]
pub fn solution1() {
    let input_data = input!();
    let input_data: Vec<&str> = input_data.split("\n\n").collect();

    // Extract workflows
    let workflows: HashMap<&str, Workflow> =
        input_data[0].split('\n').map(Workflow::from_str).collect();

    // Extract parts
    let parts: Vec<Part> = input_data[1].split('\n').map(Part::from_str).collect();

    // Run parts through workflows
    let total: u32 = parts
        .iter()
        .filter(|p| p.work(&workflows))
        .map(|p| p.total_rating())
        .sum();

    submit!(1, total);
}

#[aocd(2023, 19)]
pub fn solution2() {}
