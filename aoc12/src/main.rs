use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Row {
    springs: Vec<char>,
    broken_counts: Vec<usize>,
    unknown_locations: Vec<usize>,
    missing_broken: usize,
}

#[derive(Clone, Debug)]
struct Answer<'a> {
    row: &'a Row,
    answers: Vec<char>,
    broken_count: usize,
}

impl<'a> Answer<'a> {
    fn empty(row: &'a Row) -> Self {
        Self {
            row,
            answers: Default::default(),
            broken_count: 0,
        }
    }

    fn valid(&self) -> bool {
        if self.broken_count != self.row.missing_broken {
            return false;
        }
        let mut answers = self.answers.iter();
        let springs = self.row.springs
            .iter()
            .map(|c| match c {
                '#' => '#',
                '.' => '.',
                '?' => *(answers.next().unwrap_or(&'?')),
                _ => panic!(),
            })
            .collect::<Vec<_>>();

        //let springs_str = springs.iter().collect::<String>();
        //dbg!(&springs_str);

        let mut springs_index = 0;
        let mut broken_count_index = 0;
        while springs_index < springs.len() && broken_count_index < self.row.broken_counts.len() {
            if springs[springs_index] == '#' {
                let expected_count = self.row.broken_counts[broken_count_index];
                if springs_index + expected_count > springs.len() {
                    return false;
                }
                if springs[springs_index..springs_index + expected_count]
                    .iter()
                    .any(|c| *c != '#' && *c != '?') {
                    return false;
                }
                if springs_index + expected_count < springs.len() {
                    let next_c = springs[springs_index + expected_count];
                    if next_c != '.' && next_c != '?' {
                        return false;
                    }
                }
                springs_index += expected_count + 1;
                broken_count_index += 1;
            } else {
                springs_index += 1;
            }
        }
        true
    }

    fn add_broken(&self) -> Self {
        let mut answers = self.answers.clone();
        answers.push('#');
        Self {
            row: self.row,
            answers,
            broken_count: self.broken_count + 1,
        }
    }

    fn add_not_broken(&self) -> Self {
        let mut answers = self.answers.clone();
        answers.push('.');
        Self {
            row: self.row,
            answers,
            broken_count: self.broken_count,
        }
    }
}

impl Row {
    fn new(line: &str) -> Self {
        let words = line.split_whitespace().collect::<Vec<_>>();

        let springs = words[0].chars().collect::<Vec<_>>();

        let broken_counts = words[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let total_broken = broken_counts.iter().sum::<usize>();
        let have_broken = springs.iter().filter(|c| **c == '#').count();
        let missing_broken = total_broken - have_broken;

        let unknown_locations = springs
            .iter()
            .enumerate()
            .filter(|(_i, c)| **c == '?')
            .map(|(i, _c)| i)
            .collect::<Vec<_>>();

        Self { springs, broken_counts, unknown_locations, missing_broken }
    }


    fn recurse(&self, answer: Answer) -> usize {
        if answer.answers.len() == self.unknown_locations.len() {
            return match answer.valid() {
                true => 1,
                false => 0,
            };
        }
        let mut count: usize = 0;
        if answer.broken_count < self.missing_broken {
            count += self.recurse(answer.add_broken());
        }
        count += self.recurse(answer.add_not_broken());
        return count;
    }

    fn combos(&self) -> usize {
        self.recurse(Answer::empty(self))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let rows = data
        .lines()
        .map(Row::new)
        .collect::<Vec<_>>();

    println!("{}", rows
        .iter()
        .map(|row| row.combos())
        .sum::<usize>());
}
