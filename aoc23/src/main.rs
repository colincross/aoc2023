use std::env;
use std::fs::read_to_string;

use pathfinding::prelude::dfs_reach;

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
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    path_count: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid = data
            .lines()
            .map(|s| s
                .chars()
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        let path_count = grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c != '#').count())
            .sum();
        Self { grid, rows, cols, path_count }
    }

    fn at(&self, pos: &Pos) -> char {
        self.grid[pos.row][pos.col]
    }

    fn successors(&self, pos_history: &[Pos]) -> Vec<Vec<Pos>> {
        let mut successors = Vec::<Vec<Pos>>::new();
        let pos = pos_history.last().expect("last");

        let forced_dir = match self.at(&pos) {
            '>' => Some(Dir::RIGHT),
            '<' => Some(Dir::LEFT),
            '^' => Some(Dir::UP),
            'v' => Some(Dir::DOWN),
            '.' => None,
            _ => panic!(),
        };

        for dir in Dir::ALL {
            if let Some(forced_dir) = &forced_dir {
                if forced_dir != &dir {
                    continue;
                }
            }

            if let Some(new_pos) = self.next_pos(&pos, &dir) {
                if pos_history.contains(&new_pos) {
                    continue;
                }
                if self.at(&new_pos) != '#' {
                    let mut new_pos_history = pos_history.to_vec();
                    new_pos_history.push(new_pos);
                    successors.push(new_pos_history);
                }
            }
        }

        successors
    }

    fn find_min_value(&self) -> usize {
        let start = vec![Pos { row: 0, col: 1 }];
        /*let result = astar(&start,
                           |p| self.successors(p),
                           |p| self.distance_to_goal(p),
                           |p| p.last().expect("last").row == self.rows - 1)
            .unwrap();*/
        let paths = dfs_reach(start,
                              |p| self.successors(p))
            .filter(|pos_history| pos_history.last().unwrap().row == self.rows - 1);
        let (path, len) = paths
            .map(|path| (path.clone(), path.len()))
            .max_by(|(_, len1), (_, len2)| len1.cmp(len2))
            .unwrap();
        // for (i, path) in paths.enumerate() {
        //     println!("{}", i);
        //     self.print_path(&path);
        // }

        self.print_path(&path);
        len - 1

        //paths.map(|path| path.len()).max().unwrap()
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

    fn distance_to_goal(&self, pos_history: &[Pos]) -> isize {
        let d = (0 as isize).checked_sub_unsigned(
            self.path_count + pos_history.len())
            .unwrap();
        println!("{} {}", pos_history.len(), d);
        d
    }

    fn print_path(&self, path: &[Pos]) {
        let mut grid = vec![vec!['.'; self.cols]; self.rows];
        let len = path.len();
        for (i, pos) in path.iter().enumerate() {
            grid[pos.row][pos.col] = if i == 0 {
                'S'
            } else if i == len - 1 {
                'F'
            } else {
                'O'
            };
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
