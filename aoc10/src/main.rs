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

    rows: usize,
    cols: usize,
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
        let rows = grid.len();
        let cols = grid[0].len();
        Self { grid, rows, cols }
    }

    fn empty_copy(&self) -> Self {
        let grid: Vec<Vec<Tile>> = (0..self.rows)
            .map(|_row| (0..self.cols)
                .map(|_col| Tile { c: '.' })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        Self { grid, rows, cols }
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
        if pos.row < 0 || pos.row as usize >= self.rows ||
            pos.col < 0 || pos.col as usize >= self.cols {
            None
        } else {
            Some(self.grid[pos.row as usize][pos.col as usize].clone())
        }
    }

    fn set(&mut self, pos: &Pos, c: char) {
        self.grid[pos.row as usize][pos.col as usize].c = c;
    }

    fn set_all(&mut self, positions: &Vec<Pos>, c: char) {
        for pos in positions.iter() {
            self.set(pos, c);
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

    fn collect_pipe(&self, start: &Pos, dir: &Dir) -> Vec<Pos> {
        let mut pipe_positions: Vec<Pos> = Vec::new();
        pipe_positions.push(*start);
        let mut pos = start + dir;
        let mut prev_pos = *start;
        loop {
            let next_dir = self.follow_pipe(&prev_pos, &pos);
            if !next_dir.is_some() {
                panic!();
            }
            pipe_positions.push(pos);
            prev_pos = pos;
            pos = &pos + &next_dir.unwrap();
            if self.at(&pos).unwrap_or(Tile { c: '.' }).c == 'S' {
                return pipe_positions;
            }
        }
    }

    fn collect_connected(&mut self, pos: &Pos) -> Vec<Pos> {
        self.set(pos, 'C');
        let mut connected: Vec<Pos> = Vec::new();
        connected.push(*pos);
        for dir in Dir::DIRECTIONS.iter() {
            let adjacent_pos = pos + dir;
            let adjacent_tile = self.at(&adjacent_pos);
            if adjacent_tile.is_some() {
                match adjacent_tile.unwrap().c {
                    '*' => continue,
                    'C' => continue,
                    '.' => connected.append(&mut self.collect_connected(&adjacent_pos)),
                    _ => panic!(),
                }
            }
        }
        connected
    }

    fn count(&self, c: char) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter(|tile| tile.c == c)
            .count()
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|tile| tile.c).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(data.as_str());

    let start = grid.find_start();

    let start_dir = grid.find_start_dir(&start);

    let pipe_positions = grid.collect_pipe(&start, start_dir);

    let mut grid_mark = grid.empty_copy();
    for pos in pipe_positions {
        grid_mark.set(&pos, '*')
    }
    println!("{}", &grid_mark.to_string());

    for row in 0..grid_mark.rows {
        for col in 0..grid_mark.cols {
            let pos = Pos { row: row as i64, col: col as i64 };
            if grid_mark.at(&pos).unwrap().c == '.' {
                let connected = grid_mark.collect_connected(&pos);
                if connected
                    .iter()
                    .find(|pos| pos.row == 0
                        || pos.row as usize == grid_mark.rows - 1
                        || pos.col == 0
                        || pos.col as usize == grid_mark.cols - 1)
                    .is_some() {
                    grid_mark.set_all(&connected, 'O');
                } else {
                    grid_mark.set_all(&connected, 'I');
                }
            }
        }
    }
    println!("{}", &grid_mark.to_string());

    println!("{}", grid_mark.count('I'));
}
