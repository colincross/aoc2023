use std::env;
use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Clone)]
struct Path {
    pos_list: Vec<Pos>,
    dir_list: Vec<Dir>,
}

impl Path {
    fn valid(&self) -> bool {
        if self.pos_list[0..self.pos_list.len() - 1].contains(self.pos_list.last().unwrap()) {
            return false;
        }
        if self.dir_list
            .iter()
            .tuple_windows()
            .any(|(&a, &b, &c)| a == b && a == c) {
            return false;
        }

        return true;
    }

    fn dir_list_from_pos_list(pos_list: &Vec<Pos>) -> Vec<Dir> {
        pos_list
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| Dir {
                row: b.row as i32 - a.row as i32,
                col: b.col as i32 - a.col as i32,
            })
            .collect()
    }

    fn extend(&self, pos: Pos, dir: Dir) -> Self {
        let mut pos_list = self.pos_list.clone();
        let mut dir_list = self.dir_list.clone();
        pos_list.push(pos);
        dir_list.push(dir);
        Self { pos_list, dir_list }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Dir {
    row: i32,
    col: i32,
}

impl Dir {
    const LEFT: Dir = Dir { row: 0, col: -1 };
    const RIGHT: Dir = Dir { row: 0, col: 1 };
    const UP: Dir = Dir { row: -1, col: 0 };
    const DOWN: Dir = Dir { row: 1, col: 0 };

    const ALL: [Dir; 4] = [Self::LEFT, Self::RIGHT, Self::UP, Self::DOWN];

    fn opposite(&self) -> Self {
        Self { row: -self.row, col: -self.col }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
struct Pos {
    row: usize,
    col: usize,
}

struct Grid {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid = data
            .lines()
            .map(|s| s
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        Self { grid, rows, cols }
    }

    fn at(&self, pos: &Pos) -> u8 {
        self.grid[pos.row][pos.col]
    }

    fn naive_path(&self) -> Path {
        let mut pos_list = Vec::<Pos>::new();
        for row in 0..self.rows {
            pos_list.push(Pos { row: row, col: row });
            if row < self.cols - 1 {
                pos_list.push(Pos { row: row, col: row + 1 });
            }
        }
        let dir_list = Path::dir_list_from_pos_list(&pos_list);
        Path { pos_list, dir_list }
    }

    fn path_value(&self, pos_list: &Vec<Pos>) -> usize {
        pos_list
            .iter()
            .map(|pos| self.at(pos) as usize)
            .sum::<usize>()
    }

    fn find_min_value(&self) -> usize {
        let starting_path = Path {
            pos_list: vec![Pos { row: 0, col: 0 }],
            dir_list: vec![Dir::RIGHT],
        };

        let naive_path = self.naive_path();
        let naive_path_value = self.path_value(&naive_path.pos_list);
        let mut memo = Memo::new(naive_path, naive_path_value);

        self.recurse(&starting_path, &mut memo);
        memo.min_value
    }

    fn recurse(&self, path: &Path, memo: &mut Memo) {
        self.print_path(&path);
        let path_value = self.path_value(&path.pos_list);
        if path_value >= memo.min_value {
            return;
        }
        let last_pos = path.pos_list.last().unwrap();
        if last_pos.row == self.rows - 1 && last_pos.col == self.cols - 1 {
            if path_value < memo.min_value {
                memo.min_value = path_value;
                memo.min_path = path.clone();
            }
            return;
        }

        for dir in Dir::ALL {
            if dir == path.dir_list.last().unwrap().opposite() {
                continue;
            }

            let pos = self.next_pos(last_pos, &dir);
            if pos.is_some() {
                let new_path = path.extend(pos.unwrap(), dir);
                if new_path.valid() {
                    self.recurse(&new_path, memo);
                }
            }
        }
    }

    fn next_pos(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        let row = (pos.row as i32) + dir.row;
        let col = (pos.col as i32) + dir.col;
        if row < 0 || col < 0
            || row as usize >= self.rows
            || col as usize >= self.cols {
            None
        } else {
            Some(Pos { row: row as usize, col: col as usize })
        }
    }

    fn print_path(&self, path: &Path) {
        let mut grid = vec![vec!['.'; self.cols]; self.rows];
        for pos in path.pos_list.iter() {
            grid[pos.row][pos.col] = '#';
        }

        println!("\n{}", grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n"));
    }
}

struct Memo {
    min_value: usize,
    min_path: Path,
}

impl Memo {
    fn new(min_path: Path, min_value: usize) -> Self {
        Self { min_value, min_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(&data);

    println!("{}", grid.find_min_value());
}

// Start from the end.
// For each shell:
//  For each cell in the shell:
//   For each incoming direction list:
//    Determine an upper bound from the naive path stepping one shell in,
//     minus the best path from the whole inner shell.
//    Exhaustively recurse through the paths starting from that cell/incoming
//     direction list.  Stop if haven't hit the inner shell and have reached the
//     upper bound.
//    Memoize the smallest result.
// Can parallelize each shell if the memo min length is atomic.
