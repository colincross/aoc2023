use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Clone)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}

#[derive(Clone)]
struct Condition {
    rating: String,
    value: u32,
    operator: char,
}

struct Workflows {
    workflows: HashMap<String, Workflow>,
}

struct Part {
    ratings: HashMap<String, u32>,
}

impl Workflow {
    fn from(s: &str) -> Self {
        let curly_brace = s.find("{").expect("opening curly brace");
        let name = s[0..curly_brace].to_string();
        let rules: Vec<_> = s[curly_brace + 1..s.len() - 1]
            .split(",")
            .map(Rule::from)
            .collect();
        Self { name, rules }
    }

    fn apply(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .map(|rule| rule.apply(part))
            .find(|&target| target.is_some())
            .unwrap()
            .unwrap()
    }
}

impl Rule {
    fn from(s: &str) -> Self {
        let colon = s.find(":");
        if let Some(x) = colon {
            let condition = Some(Condition::from(&s[0..x]));
            let target = s[x + 1..].to_string();
            Self { condition, target }
        } else {
            Self { condition: None, target: s.to_string() }
        }
    }

    fn apply(&self, part: &Part) -> Option<&str> {
        if let Some(condition) = &self.condition {
            if condition.matches(part) {
                Some(&self.target)
            } else {
                None
            }
        } else {
            Some(&self.target)
        }
    }
}

impl Condition {
    fn from(s: &str) -> Self {
        let (operator_index, operator) = s
            .chars()
            .enumerate()
            .find(|&(_i, c)| c == '<' || c == '>')
            .expect("operator");
        let rating = s[0..operator_index].to_string();
        let value = s[operator_index + 1..].parse().expect("value");
        Self { rating, operator, value }
    }

    fn matches(&self, part: &Part) -> bool {
        let part_value = part.ratings[&self.rating];
        if self.operator == '<' {
            part_value < self.value
        } else {
            part_value > self.value
        }
    }
}

impl Workflows {
    fn from(workflow_list: &[Workflow]) -> Self {
        let mut workflows = HashMap::<String, Workflow>::new();

        for workflow in workflow_list {
            workflows.insert(workflow.name.clone(), workflow.clone());
        }

        Self { workflows }
    }

    fn accepted(&self, part: &Part) -> bool {
        let mut target = "in";
        while target != "A" && target != "R" {
            let workflow = &self.workflows[target];
            target = workflow.apply(part);
        }
        target == "A"
    }
}

impl Part {
    fn from(s: &str) -> Self {
        let ratings: HashMap<_, _> = s[1..s.len() - 1]
            .split(",")
            .map(|r| r.split_once("=").unwrap())
            .map(|(rating, value)| (rating.to_string(), value.parse().expect("value")))
            .collect();
        Self { ratings }
    }

    fn sum_ratings(&self) -> u32 {
        self.ratings
            .values()
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();
    let mut lines = data.lines();

    let rule_list: Vec<_> = (&mut lines)
        .take_while(|&line| line != "")
        .map(Workflow::from)
        .collect();

    let workflows = Workflows::from(&rule_list);

    println!("{}", lines
        .map(Part::from)
        .filter(|part| workflows.accepted(part))
        .map(|part| part.sum_ratings())
        .sum::<u32>());
}
