use std::{env, ops};
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos {
    row: i64,
    col: i64,
}

#[derive(Copy, Clone, Debug)]
struct Dir {
    row: i64,
    col: i64,
}

impl Dir {
    const NORTH: Self = Self { row: -1, col: 0 };
    const SOUTH: Self = Self { row: 1, col: 0 };
    const EAST: Self = Self { row: 0, col: 1 };
    const WEST: Self = Self { row: 0, col: -1 };
    const NONE: Self = Self { row: 0, col: 0 };

    const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::SOUTH, Self::EAST, Self::WEST];
}

impl ops::Add<&Dir> for &Pos {
    type Output = Pos;

    fn add(self, rhs: &Dir) -> Pos {
        Pos { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}


#[derive(Copy, Clone, Debug)]
struct Tile {
    c: char,
}

impl Tile {
    fn connected(&self) -> [Dir; 2] {
        match self.c {
            '|' => [Dir::NORTH, Dir::SOUTH],
            '-' => [Dir::WEST, Dir::EAST],
            'L' => [Dir::NORTH, Dir::EAST],
            'J' => [Dir::NORTH, Dir::WEST],
            '7' => [Dir::WEST, Dir::SOUTH],
            'F' => [Dir::EAST, Dir::SOUTH],
            '.' => [Dir::NONE, Dir::NONE],
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid: Vec<Vec<Tile>> = data
            .lines()
            .map(|s| s
                .chars()
                .map(|c| Tile { c })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { grid }
    }

    fn find_start(&self) -> Pos {
        let row_col = self
            .grid
            .iter()
            .map(|tiles| tiles
                .iter()
                .enumerate()
                .find(|(_col, tile)| tile.c == 'S'))
            .enumerate()
            .find(|(_row, start_col)| start_col.is_some())
            .unwrap();
        Pos { row: row_col.0 as i64, col: row_col.1.unwrap().0 as i64 }
    }

    fn find_start_dir(&self, start: &Pos) -> &Dir {
        Dir::DIRECTIONS
            .iter()
            .find(|dir| self.follow_pipe(&start, &(start + *dir)).is_some())
            .unwrap()
    }

    fn at(&self, pos: &Pos) -> Option<Tile> {
        if pos.row < 0 || pos.row as usize >= self.grid.len() ||
            pos.col < 0 || pos.col as usize >= self.grid[0].len() {
            None
        } else {
            Some(self.grid[pos.row as usize][pos.col as usize].clone())
        }
    }
    fn follow_pipe(&self, prev: &Pos, cur: &Pos) -> Option<Dir> {
        let connected = self.at(cur)?.connected();
        if cur + &connected[0] == *prev {
            Some(connected[1])
        } else if cur + &connected[1] == *prev {
            Some(connected[0])
        } else {
            None
        }
    }

    fn count_pipe(&self, start: &Pos, dir: &Dir) -> usize {
        let mut count: usize = 1;
        let mut pos = start + dir;
        let mut prev_pos = *start;
        loop {
            let next_dir = self.follow_pipe(&prev_pos, &pos);
            if !next_dir.is_some() {
                return 0;
            }
            count += 1;
            prev_pos = pos;
            pos = &pos + &next_dir.unwrap();
            if self.at(&pos).unwrap_or(Tile { c: '.' }).c == 'S' {
                return count / 2;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(data.as_str());

    let start = grid.find_start();

    let start_dir = grid.find_start_dir(&start);

    let count = grid.count_pipe(&start, start_dir);
    println!("{}", count);
}
