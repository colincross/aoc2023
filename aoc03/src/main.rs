use regex::Regex;
use std::env;
use std::fs::read_to_string;

#[derive(Default, Debug, Clone)]
struct PartNumber<'a> {
    row: usize,
    col: usize,
    s: &'a str,
    n: u32,
}

struct Grid<'a> {
    rows: Vec<&'a str>,
}

struct GridIterator<'a> {
    start_row: i32,
    end_row: usize,

    start_col: i32,
    end_col: usize,

    row: i32,
    col: i32,

    grid: &'a Grid<'a>,
}

impl<'a> GridIterator<'a> {
    fn new(start_row: i32, end_row: usize,
           start_col: i32, end_col: usize,
           grid: &'a Grid,
    ) -> Self {
        Self {
            start_row: start_row,
            end_row: end_row,
            start_col: start_col,
            end_col: end_col,
            grid: grid,
            row: start_row,
            col: start_col-1,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    // we will be counting with usize
    type Item = char;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if self.row < 0 {
            self.row = 0;
        }

        self.col += 1;

        if self.col > 0
            && (self.col as usize > self.end_col
                || self.col as usize >= self.grid.rows[0].len()) {
            self.row += 1;
            self.col = self.start_col;
        }

        if self.col < 0 {
            self.col = 0;
        }

        if self.row > 0
            && (self.row as usize > self.end_row
                || self.row as usize >= self.grid.rows.len()) {
            None
        } else {
            let c = self.grid.rows[self.row as usize].chars().nth(self.col as usize).unwrap();
            Some(c)
        }
    }
}


impl<'a> Grid<'a> {
    fn new(data: &'a str) -> Self {
        let rows = data.lines().collect();
        Self { rows }
    }

    fn around(&self, row: usize, col: usize, part_number_len: usize) -> GridIterator {
        GridIterator::new(
            row as i32 - 1,
            row + 1,
            col as i32 - 1,
            col + part_number_len,
            self)
    }       
}

fn find_part_numbers(row: usize, s: &str) -> Vec<PartNumber> {
    let re = Regex::new(r"(\d+)").unwrap();
        
    re
        .find_iter(s)
        .map(|m| PartNumber{
            row: row,
            col: m.start(),
            s: m.as_str().into(),
            n: m.as_str().parse().unwrap(),
        })
        .collect()
}

fn has_adjacent_symbols(p: &PartNumber, grid: &Grid) -> bool {
    let adjacent = grid
        .around(p.row, p.col, p.s.len())
        .any(|c| !c.is_ascii_digit() && c != '.');

    adjacent
}
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();
    let grid = Grid::new(data.as_str());

    println!("{:#?}",
             grid.rows
             .iter()
             .zip(0..)
             .map(|pair: (&&str, usize)| find_part_numbers(pair.1, pair.0))
             .flatten()
             .filter(|part_number| has_adjacent_symbols(part_number, &grid))
             .map(|part_number| part_number.n)
             .sum::<u32>()
            );
}
