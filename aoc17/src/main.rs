use std::env;
use std::fs::read_to_string;

use pathfinding::prelude::astar;

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
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PosWithDirs {
    pos: Pos,
    last_dir: Dir,
    last_dir_count: usize,
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

    fn at(&self, pos: &Pos) -> usize {
        self.grid[pos.row][pos.col] as usize
    }

    fn successors(&self, pos_with_dirs: &PosWithDirs) -> Vec<(PosWithDirs, usize)> {
        let mut successors = Vec::<(PosWithDirs, usize)>::new();

        for dir in Dir::ALL {
            if dir == pos_with_dirs.last_dir.opposite() {
                continue;
            }

            if pos_with_dirs.last_dir != Dir::NONE
                && dir != pos_with_dirs.last_dir
                && pos_with_dirs.last_dir_count < 4 {
                continue;
            }

            if dir == pos_with_dirs.last_dir
                && pos_with_dirs.last_dir_count >= 10 {
                continue;
            }

            let new_pos = self.next_pos(&pos_with_dirs.pos, &dir);
            let dir_count = if dir == pos_with_dirs.last_dir {
                pos_with_dirs.last_dir_count + 1
            } else {
                1
            };
            if let Some(pos) = new_pos {
                let weight = self.at(&pos);
                let new_pos_with_dir = PosWithDirs {
                    pos: pos,
                    last_dir_count: dir_count,
                    last_dir: dir,

                };
                successors.push((new_pos_with_dir, weight));
            }
        }

        successors
    }

    fn find_min_value(&self) -> usize {
        let start = PosWithDirs::default();
        let goal = Pos {
            row: self.rows - 1,
            col: self.cols - 1,
        };
        let result = astar(&start,
                           |p| self.successors(p),
                           |p| self.distance_to_goal(&p.pos),
                           |p| p.pos == goal && p.last_dir_count >= 4)
            .unwrap();

        self.print_path(&result.0);

        result.1
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

    fn distance_to_goal(&self, pos: &Pos) -> usize {
        (pos.row.abs_diff(self.rows - 1) + pos.col.abs_diff(self.cols - 1))
            / 3
    }

    fn print_path(&self, path: &[PosWithDirs]) {
        let mut grid = vec![vec!['.'; self.cols]; self.rows];
        for pos in path.iter() {
            grid[pos.pos.row][pos.pos.col] = '#';
        }

        println!("\n{}", grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n"));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(&data);

    println!("{}", grid.find_min_value());
}
