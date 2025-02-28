use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Line {
    line: Vec<char>,
}

impl Line {
    fn new(line: Vec<char>) -> Self {
        Self { line }
    }

    fn find_next_rolling_rock(line: &[char], index: usize) -> Option<usize> {
        let next = (index + 1..line.len())
            .skip_while(|&i| line[i] == '.')
            .next();

        match next {
            Some(i) => match line[i] {
                'O' => Some(i),
                '#' => None,
                _ => panic!(),
            }
            None => None,
        }
    }
    fn tilt_toward_zero(&self) -> Self {
        let mut line = self.line.clone();
        let mut index = 0;
        while index < line.len() {
            if line[index] == '.' {
                let i = Line::find_next_rolling_rock(&line, index);
                if i.is_some() {
                    line[index] = 'O';
                    line[i.unwrap()] = '.';
                }
            }
            index += 1;
        }

        Self { line }
    }

    fn load(&self) -> usize {
        self.line
            .iter()
            .enumerate()
            .map(|(i, &c)| match c == 'O' {
                true => self.line.len() - i,
                false => 0,
            })
            .sum::<usize>()
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Dish {
    lines: Vec<Line>,
}

impl Dish {
    fn new(s: &str) -> Self {
        let rows = s
            .lines()
            .collect::<Vec<_>>();
        let lines =
            (0..rows[0].len())
                .map(|col| (0..rows.len())
                    .map(|row| rows[row].as_bytes()[col] as char)
                    .collect::<Vec<_>>())
                .map(Line::new)
                .collect::<Vec<_>>();

        Self { lines }
    }

    fn rotate(&self) -> Self {
        let rows = self.lines.len();
        let cols = self.lines[0].line.len();
        let mut lines =
            (0..cols)
                .map(|col| (0..rows)
                    .map(|row| self.lines[row].line[col])
                    .collect::<Vec<_>>())
                .map(Line::new)
                .collect::<Vec<_>>();
        lines.reverse();

        Self { lines }
    }

    fn tilt_north(&self) -> Self {
        let north_lines = self.lines
            .iter()
            .map(|line| line.tilt_toward_zero())
            .collect::<Vec<_>>();

        Self { lines: north_lines }
    }

    fn spin_cycle(&self) -> Self {
        self
            .tilt_north()
            .rotate()
            .tilt_north()
            .rotate()
            .tilt_north()
            .rotate()
            .tilt_north()
            .rotate()
    }

    fn load(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.load())
            .sum::<usize>()
    }

    fn to_string(&self) -> String {
        self
            .rotate()
            .lines
            .iter()
            .rev()
            .map(|line| line.line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut dish = Dish::new(&data);

    let mut seen: HashMap<Dish, usize> = HashMap::new();
    let mut count = 0;

    loop {
        let entry = seen.entry(dish.clone());
        match entry {
            Entry::Occupied(_entry) => break,
            Entry::Vacant(entry) => entry.insert(count),
        };
        dish = dish.spin_cycle();
        count += 1;
    }

    const CYCLES: usize = 1000000000;
    let loop_len = count - seen.get(&dish).unwrap();
    let remain = (CYCLES - count) % loop_len;

    for _i in 0..remain {
        dish = dish.spin_cycle();
    }
    println!("{}", dish.load());
}

//         V<-
//       * * *
//       0 1 2
//         3 4