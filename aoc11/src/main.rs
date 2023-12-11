use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid: Vec<Vec<char>> = data
            .lines()
            .map(|s| s
                .chars()
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        Self { grid, rows, cols }
    }

    fn at(&self, row: usize, col: usize) -> char {
        self.grid[row][col]
    }
    fn col(&self, col: usize) -> Vec<char> {
        (0..self.rows)
            .map(|row| self.at(row, col))
            .collect::<Vec<_>>()
    }

    fn expand(&self) -> Self {
        let empty_rows: Vec<usize> = self.grid
            .iter()
            .enumerate()
            .filter(|(i, row)| !row.contains(&'#'))
            .map(|(i, row)| i)
            .collect::<Vec<_>>();

        let empty_cols: Vec<usize> = (0..self.cols)
            .map(|col| (col, self.col(col)))
            .filter(|(i, col)| !col.contains(&'#'))
            .map(|(i, col)| i)
            .collect::<Vec<_>>();
        dbg!(&empty_rows, &empty_cols);

        let mut grid = self.grid.clone();
        let mut rows = self.rows;
        let mut cols = self.cols;

        for (i, row) in empty_rows.iter().enumerate() {
            grid.insert(row + i, vec!['.'; cols]);
            rows += 1
        }

        for (i, col) in empty_cols.iter().enumerate() {
            for row in 0..rows {
                grid[row].insert(col + i, '.');
            }
            cols += 1
        }

        Self { grid, rows, cols }
    }


    fn galaxies(&self) -> Vec<Pos> {
        self.grid
            .iter()
            .enumerate()
            .map(|(row, row_contents)| (row, row_contents
                .iter()
                .enumerate()
                .filter(|(col, col_contents)| **col_contents == '#')
                .map(|(col, col_contents)| col)
                .collect::<Vec<usize>>()))
            .map(|(row, cols)| cols
                .iter()
                .map(|col| Pos { row: row, col: *col })
                .collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>()
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(&data).expand();

    let galaxies = grid.galaxies();
    let mut sum = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let a = &galaxies[i];
            let b = &galaxies[j];
            let dist = (a.row as i64 - b.row as i64).abs() +
                (a.col as i64 - b.col as i64).abs();
            sum += dist as usize;
        }
    }
    println!("{}", sum);
}
