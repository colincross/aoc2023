use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Dir {
    row: isize,
    col: isize,
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

struct Grid {
    grid: Vec<Vec<char>>,
    start: Pos,
    rows: usize,
    cols: usize,
}

#[derive(Default)]
struct Memo {
    seen: HashSet<(Pos, usize)>,
}

impl Grid {
    fn from(data: &str) -> Self {
        let grid = data
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(row, row_data)|
                row_data
                    .iter()
                    .enumerate()
                    .find_map(|(col, &c)|
                        if c == 'S' { Some(Pos { row, col }) } else { None }))
            .unwrap();
        let rows = grid.len();
        let cols = grid[0].len();
        Self { grid, start, rows, cols }
    }

    fn walk(&self, pos: &Pos, depth: usize, memo: &mut Memo) -> usize {
        let mut count = 0;
        let seen = !memo.seen.insert((pos.clone(), depth));
        if seen {
            return 0;
        }

        if depth == 0 {
            return 1;
        }

        for dir in &Dir::ALL {
            if let Some(new_pos) = self.add(pos, dir) {
                if !self.rock(&new_pos) {
                    count += self.walk(&new_pos, depth - 1, memo);
                }
            }
        }

        count
    }


    fn rock(&self, pos: &Pos) -> bool {
        self.grid[pos.row][pos.col] == '#'
    }
    fn add(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        let row = pos.row.checked_add_signed(dir.row)?;
        let col = pos.col.checked_add_signed(dir.col)?;
        if row < self.rows && col < self.cols {
            Some(Pos { row, col })
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let steps = args[2].parse::<usize>().unwrap();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::from(&data);

    let mut memo = Memo::default();

    println!("{}", grid.walk(&grid.start, steps, &mut memo));
}
