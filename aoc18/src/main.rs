use std::{env, fmt};
use std::cmp::{max, min};
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
            dbg!(a, b);
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
}

#[derive(Clone, Copy, Debug, Default)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn apply(&self, step: &Step) -> Vec<Self> {
        let mut pos = *self;
        (0..step.num)
            .map(|_| {
                let row = pos.row + step.dir.row;
                let col = pos.col + step.dir.col;
                pos = Self { row, col };
                pos
            })
            .collect()
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    min_pos: Pos,
}

impl Grid {
    fn new(min_pos: &Pos, max_pos: &Pos) -> Self {
        let rows = (max_pos.row - min_pos.row + 1) as usize;
        let cols = (max_pos.col - min_pos.col + 1) as usize;
        let grid = vec![vec!['.'; cols]; rows];
        let min_pos = *min_pos;
        Self { rows, cols, grid, min_pos }
    }

    fn at(&mut self, pos: &Pos) -> &mut char {
        &mut self.grid
            [(pos.row - self.min_pos.row) as usize]
            [(pos.col - self.min_pos.col) as usize]
    }

    fn apply(&mut self, steps: &[Step]) {
        let mut pos = Pos::default();
        let mut dir = Dir::NONE;
        for step in steps {
            if dir != Dir::NONE {
                *self.at(&pos) = Dir::corner(&dir, &step.dir)
            }
            let pos_list = pos.apply(step);
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
                pos = *pos.apply(step).last().unwrap();
                pos
            })
            .collect()
    }
    fn from(steps: &[Step]) -> Self {
        let pos_list = Self::steps_to_pos_list(steps);

        let max_pos = pos_list
            .iter()
            .fold(Pos { row: 0, col: 0 },
                  |max_pos, pos| {
                      Pos {
                          row: max(pos.row, max_pos.row),
                          col: max(pos.col, max_pos.col),
                      }
                  });
        let min_pos = pos_list
            .iter()
            .fold(Pos { row: 0, col: 0 },
                  |min_pos, pos| {
                      Pos {
                          row: min(pos.row, min_pos.row),
                          col: min(pos.col, min_pos.col),
                      }
                  });

        let mut grid = Self::new(&min_pos, &max_pos);
        grid.apply(&steps);
        grid
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for row in &self.grid {
            let mut last_corner = '.';
            let mut inside = false;
            let mut row_count = 0;
            for &c in row {
                let is_floor = c == '.';
                if is_floor {
                    if inside {
                        row_count += 1;
                    }
                } else {
                    row_count += 1;
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
            println!("{} {}", row.iter().collect::<String>(), row_count);
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
        .collect();


    let mut grid = Grid::from(&steps);

    //println!("{}", &grid);
    println!("{}", grid.count());
}
