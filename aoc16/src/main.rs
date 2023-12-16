use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Dir {
    row: i32,
    col: i32,
}

impl Dir {
    const LEFT: Dir = Dir { row: 0, col: -1 };
    const RIGHT: Dir = Dir { row: 0, col: 1 };
    const UP: Dir = Dir { row: -1, col: 0 };
    const DOWN: Dir = Dir { row: 1, col: 0 };
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
struct Pos {
    row: usize,
    col: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid = data
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { grid }
    }

    fn at(&self, pos: &Pos) -> char {
        self.grid[pos.row][pos.col]
    }


    fn next_pos(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        let row = (pos.row as i32) + dir.row;
        let col = (pos.col as i32) + dir.col;
        if row < 0 || col < 0
            || row as usize >= self.grid.len()
            || col as usize >= self.grid[0].len() {
            None
        } else {
            Some(Pos { row: row as usize, col: col as usize })
        }
    }

    fn next_dirs(&self, pos: &Pos, dir: &Dir) -> Vec<Dir> {
        match self.at(&pos) {
            '.' => vec![*dir],
            '|' => match *dir {
                Dir::UP | Dir::DOWN => vec![*dir],
                _ => vec![Dir::UP, Dir::DOWN],
            },
            '-' => match *dir {
                Dir::LEFT | Dir::RIGHT => vec![*dir],
                _ => vec![Dir::LEFT, Dir::RIGHT],
            },
            '\\' => match *dir {
                Dir::LEFT => vec![Dir::UP],
                Dir::RIGHT => vec![Dir::DOWN],
                Dir::UP => vec![Dir::LEFT],
                Dir::DOWN => vec![Dir::RIGHT],
                _ => panic!(),
            }
            '/' => match *dir {
                Dir::LEFT => vec![Dir::DOWN],
                Dir::RIGHT => vec![Dir::UP],
                Dir::UP => vec![Dir::RIGHT],
                Dir::DOWN => vec![Dir::LEFT],
                _ => panic!(),
            }
            _ => panic!(),
        }
    }

    fn walk<F>(&self, pos: &Pos, dir: &Dir, seen: &mut HashMap<(Pos, Dir), ()>, f: &mut F) where
        F: FnMut(&Pos) {
        match seen.entry((*pos, *dir)) {
            Entry::Occupied(_) => return,
            Entry::Vacant(x) => x.insert(()),
        };
        f(pos);
        let next_dirs = self.next_dirs(pos, dir);
        for next_dir in next_dirs {
            let next_pos = self.next_pos(pos, &next_dir);
            if next_pos.is_some() {
                self.walk(&next_pos.unwrap(), &next_dir, seen, f);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(&data);

    let mut energized = vec![vec!['.'; grid.grid[0].len()]; grid.grid.len()];

    let mut seen = HashMap::<(Pos, Dir), ()>::new();
    grid.walk(&Pos::default(), &Dir::RIGHT, &mut seen,
              &mut |pos: &Pos| energized[pos.row][pos.col] = '#');

    for e in &energized {
        println!("{}", e.iter().collect::<String>());
    }

    println!("{}", energized
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum::<usize>());
}
