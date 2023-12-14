use std::cmp::min;
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn new(lines: &[&str]) -> Self {
        let rows = lines
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let cols = (0..rows[0].len())
            .map(|col| (0..rows.len())
                .map(|row| rows[row].as_bytes()[col] as char)
                .collect::<String>())
            .collect::<Vec<_>>();

        Self { rows, cols }
    }

    fn distance(a: &str, b: &str) -> usize {
        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();
        (0..bytes_a.len())
            .map(|i| (bytes_a[i] != bytes_b[i]) as usize)
            .sum::<usize>()
    }

    fn is_mirror(index: usize, list: &[String]) -> bool {
        let size = min(index + 1, list.len() - index - 1);
        let distance = (0..size)
            .map(|i| Self::distance(&list[index - i], &list[index + 1 + i]))
            .sum::<usize>();
        distance == 1
    }

    fn find_mirror_row(&self) -> Option<usize> {
        (0..(self.rows.len() - 1))
            .find(|&row| Self::is_mirror(row, &self.rows))
    }

    fn find_mirror_col(&self) -> Option<usize> {
        (0..(self.cols.len() - 1))
            .find(|&col| Self::is_mirror(col, &self.cols))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let patterns =
        data
            .lines()
            .collect::<Vec<_>>()
            .split(|&s| s == "")
            .map(Pattern::new)
            .collect::<Vec<_>>();

    println!("{}",
             patterns
                 .iter()
                 .filter_map(|pattern| pattern.find_mirror_row())
                 .map(|row| (row + 1) * 100)
                 .sum::<usize>()
                 + patterns
                 .iter()
                 .filter_map(|pattern| pattern.find_mirror_col())
                 .map(|col| col + 1)
                 .sum::<usize>(),
    );
}
