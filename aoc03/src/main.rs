use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Default, Debug, Clone, Copy)]
struct PartNumber<'a> {
    row: usize,
    col: usize,
    s: &'a str,
    n: u32,
}

struct Grid<'a> {
    rows: Vec<&'a str>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    row: usize,
    col: usize,
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
    type Item = (Point, char);

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
            let point = Point{ row: self.row as usize, col: self.col as usize };
            Some((point, c))
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

fn adjacent_gears(p: &PartNumber, grid: &Grid) -> Vec<Point> {
    grid
        .around(p.row, p.col, p.s.len())
        .filter(|pair: &(Point, char)| pair.1 == '*')
        .map(|pair: (Point, char)| pair.0)
        .collect()
}
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();
    let grid = Grid::new(data.as_str());

    let grid_dict =
             grid.rows
             .iter()
             .zip(0..)
             .map(|(s, row): (&&str, usize)| find_part_numbers(row, s))
             .flatten()
             .map(|part_number| (part_number, adjacent_gears(&part_number, &grid)))
             .fold(HashMap::new(),
                   |mut dict: HashMap<Point, Vec<PartNumber>>, (part_number, gears): (PartNumber, Vec<Point>)| {
                       gears.iter().for_each(|gear| {
                           match dict.entry(*gear) {
                               Entry::Vacant(e) => { e.insert(vec![part_number]); },
                               Entry::Occupied(mut e) => { e.get_mut().push(part_number); }
                           }
                       });
                       dict
                   });

    println!("{}",
        grid_dict
            .iter()
            .filter(|(_gear, part_numbers): &(&Point, &Vec<PartNumber>)| part_numbers.len() == 2)
            .map(|(_gear, part_numbers): (&Point, &Vec<PartNumber>)| part_numbers[0].n * part_numbers[1].n)
            .sum::<u32>());
    }
