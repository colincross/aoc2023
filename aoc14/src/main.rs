use std::env;
use std::fs::read_to_string;

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

struct Dish {
    north_lines: Vec<Line>,
}

impl Dish {
    fn new(s: &str) -> Self {
        let rows = s
            .lines()
            .collect::<Vec<_>>();
        let north_lines =
            (0..rows[0].len())
                .map(|col| (0..rows.len())
                    .map(|row| rows[row].as_bytes()[col] as char)
                    .collect::<Vec<_>>())
                .map(Line::new)
                .collect::<Vec<_>>();

        Self { north_lines }
    }

    fn tilt_north(&self) -> Self {
        let north_lines = self.north_lines
            .iter()
            .map(|line| line.tilt_toward_zero())
            .collect::<Vec<_>>();

        Self { north_lines }
    }

    fn load(&self) -> usize {
        self.north_lines
            .iter()
            .map(|line| line.load())
            .sum::<usize>()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let dish = Dish::new(&data);
    let tilted_dish = dish.tilt_north();
    println!("{}", tilted_dish.load());
}
