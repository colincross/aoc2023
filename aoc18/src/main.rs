use std::{env, fmt};
use std::fs::read_to_string;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Dir {
    row: i32,
    col: i32,
}

impl Dir {
    const LEFT: Dir = Dir { row: 0, col: -1 };
    const RIGHT: Dir = Dir { row: 0, col: 1 };
    const UP: Dir = Dir { row: -1, col: 0 };
    const DOWN: Dir = Dir { row: 1, col: 0 };
    const NONE: Dir = Dir { row: 0, col: 0 };

    const ALL: [Dir; 4] = [Self::LEFT, Self::RIGHT, Self::UP, Self::DOWN];

    fn opposite(&self) -> Self {
        Self { row: -self.row, col: -self.col }
    }

    fn from(s: &str) -> Self {
        match s {
            "R" => Self::RIGHT,
            "L" => Self::LEFT,
            "U" => Self::UP,
            "D" => Self::DOWN,
            _ => panic!(),
        }
    }

    fn corner(a: &Dir, b: &Dir) -> char {
        if a == &Self::UP && b == &Self::RIGHT ||
            a == &Self::LEFT && b == &Self::DOWN {
            'F'
        } else if a == &Self::RIGHT && b == &Self::DOWN ||
            a == &Self::UP && b == &Self::LEFT {
            '7'
        } else if a == &Self::DOWN && b == &Self::LEFT ||
            a == &Self::RIGHT && b == &Self::UP {
            'J'
        } else if a == &Self::LEFT && b == &Self::UP ||
            a == &Self::DOWN && b == &Self::RIGHT {
            'L'
        } else {
            panic!()
        }
    }
}

struct Step {
    dir: Dir,
    num: usize,
    color: u32,
}

impl Step {
    fn from(s: &str) -> Self {
        let fields: Vec<_> = s.split_whitespace().collect();

        let dir = Dir::from(fields[0]);
        let num = fields[1].parse().expect("num");
        let color = u32::from_str_radix(&fields[2][2..8], 16).expect("hex color");

        Self { dir, num, color }
    }

    fn from_hex(&self) -> Self {
        let dir = vec![Dir::RIGHT, Dir::DOWN, Dir::LEFT, Dir::UP][(self.color & 0xF) as usize].clone();
        let num = (self.color >> 4) as usize;
        let color = self.color;
        Self { dir, num, color }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Pos {
    row: i32,
    col: i32,
}

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    row_quanta: Vec<i32>,
    col_quanta: Vec<i32>,
    row_size: Vec<usize>,
    col_size: Vec<usize>,
}

impl Grid {
    fn new(row_quanta: Vec<i32>, col_quanta: Vec<i32>) -> Self {
        let rows = row_quanta.len();
        let cols = col_quanta.len();
        let mut row_size: Vec<_> = row_quanta
            .windows(2)
            .map(|rows: &[i32]| (rows[1] - rows[0]) as usize)
            .collect();
        row_size.push(1);
        let mut col_size: Vec<_> = col_quanta
            .windows(2)
            .map(|cols: &[i32]| (cols[1] - cols[0]) as usize)
            .collect();
        col_size.push(1);

        let grid = vec![vec!['.'; cols]; rows];
        Self { rows, cols, grid, row_quanta, col_quanta, row_size, col_size }
    }

    fn at(&mut self, pos: &Pos) -> &mut char {
        let row_index = self.row_quanta.iter().position(|&x| x == pos.row).expect("row quanta not found");
        let col_index = self.col_quanta.iter().position(|&x| x == pos.col).expect("col quanta not found");
        &mut self.grid[row_index][col_index]
    }

    fn step_to_pos_list(&self, pos: &Pos, step: &Step) -> Vec<Pos> {
        match step.dir {
            Dir::LEFT => {
                let start_col = pos.col;
                let end_col = pos.col - (step.num as i32);
                let mut pos_list = self
                    .col_quanta
                    .iter()
                    .filter(|&&col| col < start_col && col >= end_col)
                    .map(|&col| Pos { row: pos.row, col: col })
                    .collect::<Vec<_>>();
                pos_list.reverse();
                pos_list
            }
            Dir::RIGHT => {
                let start_col = pos.col;
                let end_col = pos.col + (step.num as i32);
                let mut pos_list = self
                    .col_quanta
                    .iter()
                    .filter(|&&col| col > start_col && col <= end_col)
                    .map(|&col| Pos { row: pos.row, col: col })
                    .collect::<Vec<_>>();
                pos_list
            }
            Dir::UP => {
                let start_row = pos.row;
                let end_row = pos.row - (step.num as i32);
                let mut pos_list = self
                    .row_quanta
                    .iter()
                    .filter(|&&row| row < start_row && row >= end_row)
                    .map(|&row| Pos { col: pos.col, row: row })
                    .collect::<Vec<_>>();
                pos_list.reverse();
                pos_list
            }
            Dir::DOWN => {
                let start_row = pos.row;
                let end_row = pos.row + (step.num as i32);
                let mut pos_list = self
                    .row_quanta
                    .iter()
                    .filter(|&&row| row > start_row && row <= end_row)
                    .map(|&row| Pos { col: pos.col, row: row })
                    .collect::<Vec<_>>();
                pos_list
            }
            _ => panic!(),
        }
    }

    fn apply(&mut self, steps: &[Step]) {
        let mut pos = Pos::default();
        let mut dir = Dir::NONE;
        for step in steps {
            if dir != Dir::NONE {
                *self.at(&pos) = Dir::corner(&dir, &step.dir)
            }
            let pos_list = self.step_to_pos_list(&pos, step);
            for p in &pos_list {
                if step.dir == Dir::UP || step.dir == Dir::DOWN {
                    *self.at(p) = '|';
                } else {
                    *self.at(p) = '-';
                }
            }
            pos = *pos_list.last().unwrap();
            dir = step.dir.clone();
        }
        *self.at(&pos) = Dir::corner(&dir, &steps[0].dir);
    }

    fn steps_to_pos_list(steps: &[Step]) -> Vec<Pos> {
        let mut pos = Pos::default();
        steps
            .iter()
            .map(|step| {
                pos = Pos {
                    row: pos.row + step.dir.row * (step.num as i32),
                    col: pos.col + step.dir.col * (step.num as i32),
                };
                pos
            })
            .collect()
    }

    fn from(steps: &[Step]) -> Self {
        let pos_list = Self::steps_to_pos_list(steps);

        let mut row_quanta: Vec<_> = pos_list
            .iter()
            .map(|pos| vec![pos.row - 1, pos.row, pos.row + 1])
            .flatten()
            .collect();
        row_quanta.sort();
        row_quanta.dedup();

        let mut col_quanta: Vec<_> = pos_list
            .iter()
            .map(|pos| vec![pos.col - 1, pos.col, pos.col + 1])
            .flatten()
            .collect();
        col_quanta.sort();
        col_quanta.dedup();

        let mut grid = Self::new(row_quanta, col_quanta);
        grid.apply(&steps);
        grid
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for row in 0..self.grid.len() {
            let mut last_corner = '.';
            let mut inside = false;
            let mut row_count = 0;
            for col in 0..self.grid[row].len() {
                let c = self.grid[row][col];
                let is_floor = c == '.';
                if is_floor {
                    if inside {
                        row_count += self.row_size[row] * self.col_size[col];
                    }
                } else {
                    row_count += self.row_size[row] * self.col_size[col];
                    if c == '|' {
                        inside = !inside;
                    } else if c == 'J' {
                        assert!(last_corner == 'F' || last_corner == 'L');
                        if last_corner == 'F' {
                            inside = !inside;
                        }
                    } else if c == 'F' {
                        last_corner = 'F';
                    } else if c == 'L' {
                        last_corner = 'L';
                    } else if c == '7' {
                        assert!(last_corner == 'F' || last_corner == 'L');
                        if last_corner == 'L' {
                            inside = !inside;
                        }
                    }
                }
            }
            count += row_count;
            println!("{} {}", self.grid[row].iter().collect::<String>(), row_count);
        }
        count
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let steps: Vec<Step> = data
        .lines()
        .map(Step::from)
        .map(|step| step.from_hex())
        .collect();


    let mut grid = Grid::from(&steps);

    //println!("{}", &grid);
    println!("{}", grid.count());
}
