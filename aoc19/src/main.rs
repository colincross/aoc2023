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

#[derive(Clone)]
struct RatingRange {
    ranges: Vec<(u32, u32)>,
}

#[derive(Clone)]
struct PartRatingRanges {
    ratings: HashMap<String, RatingRange>,
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

    fn split_recurse(rules: &[Rule], part_rating_ranges: &PartRatingRanges)
                     -> Vec<(PartRatingRanges, String)> {
        let mut next = Vec::<(PartRatingRanges, String)>::new();

        for (next_ranges, target) in rules[0].split(part_rating_ranges) {
            if let Some(x) = target {
                next.push((next_ranges, x.to_string()));
            } else {
                next.append(&mut Self::split_recurse(&rules[1..], &next_ranges));
            }
        }
        next
    }

    fn split(&self, part_rating_ranges: &PartRatingRanges)
             -> Vec<(PartRatingRanges, String)> {
        Self::split_recurse(&self.rules, part_rating_ranges)
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

    fn split(&self, part_rating_ranges: &PartRatingRanges)
             -> Vec<(PartRatingRanges, Option<&str>)> {
        if let Some(condition) = &self.condition {
            let (true_part_rating_ranges, false_part_rating_ranges) =
                condition.split(part_rating_ranges);
            vec![(true_part_rating_ranges, Some(&self.target)),
                 (false_part_rating_ranges, None)]
        } else {
            vec![(part_rating_ranges.clone(), Some(&self.target))]
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

    fn split(&self, part_rating_ranges: &PartRatingRanges) -> (PartRatingRanges, PartRatingRanges) {
        let part_ranges = &part_rating_ranges.ratings[&self.rating];
        let (true_ranges, false_ranges) = part_ranges
            .split(self.operator, self.value);

        let mut true_part_rating_ranges = part_rating_ranges.clone();
        (&mut true_part_rating_ranges.ratings).insert(self.rating.clone(), true_ranges);
        let mut false_part_rating_ranges = part_rating_ranges.clone();
        (&mut false_part_rating_ranges.ratings).insert(self.rating.clone(), false_ranges);

        (true_part_rating_ranges, false_part_rating_ranges)
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

    fn accepted(&self, rating_ranges: &PartRatingRanges) -> usize {
        let mut count = 0;
        let mut work_list = Vec::<(PartRatingRanges, &Workflow)>::new();
        work_list.push((PartRatingRanges::default(), &self.workflows["in"]));
        while let Some((range, workflow)) = work_list.pop() {
            for (next_range, target) in workflow.split(&range) {
                if target == "A" {
                    count += next_range.size();
                } else if target != "R" {
                    work_list.push((next_range, &self.workflows[&target]));
                }
            }
        }
        count
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

impl RatingRange {
    fn split(&self, operator: char, value: u32) -> (Self, Self) {
        let mut true_ranges = Vec::<(u32, u32)>::new();
        let mut false_ranges = Vec::<(u32, u32)>::new();
        if operator == '<' {
            for range in &self.ranges {
                if range.1 < value {
                    true_ranges.push(*range);
                } else if range.0 < value {
                    true_ranges.push((range.0, value - 1));
                    false_ranges.push((value, range.1));
                } else {
                    false_ranges.push(*range);
                }
            }
        } else {
            for range in &self.ranges {
                if range.0 > value {
                    true_ranges.push(*range);
                } else if range.1 > value {
                    true_ranges.push((value + 1, range.1));
                    false_ranges.push((range.0, value));
                } else {
                    false_ranges.push(*range);
                }
            }
        }
        (Self { ranges: true_ranges }, Self { ranges: false_ranges })
    }

    fn default() -> Self {
        Self { ranges: vec![(1, 4000)] }
    }
}

impl PartRatingRanges {
    fn default() -> Self {
        Self {
            ratings: HashMap::<String, RatingRange>::from(
                [
                    (String::from("x"), RatingRange::default()),
                    (String::from("m"), RatingRange::default()),
                    (String::from("a"), RatingRange::default()),
                    (String::from("s"), RatingRange::default()),
                ])
        }
    }

    fn size(&self) -> usize {
        self.ratings
            .values()
            .map(|ranges| ranges.ranges
                .iter()
                .map(|range| (range.1 - range.0 + 1) as usize)
                .sum::<usize>())
            .product()
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

    let rating_ranges = PartRatingRanges::default();

    println!("{}", workflows.accepted(&rating_ranges));
}
