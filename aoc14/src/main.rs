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
    lines: Vec<Line>,
}

impl Dish {
    fn new(s: &str) -> Self {
        let lines = s
            .lines()
            .map(|line| Line::new(line.chars().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        Self { lines }.rotate()
    }

    fn rotate(&self) -> Self {
        let lines =
            (0..self.lines[0].line.len())
                .map(|col| (0..self.lines.len())
                    .map(|row| self.lines[row].line[col])
                    .collect::<Vec<_>>())
                .map(Line::new)
                .collect::<Vec<_>>();

        Self { lines }
    }

    fn tilt_north(&self) -> Self {
        let north_lines = self.lines
            .iter()
            .map(|line| line.tilt_toward_zero())
            .collect::<Vec<_>>();

        Self { lines: north_lines }
    }

    fn load(&self) -> usize {
        self.lines
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
