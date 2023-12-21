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

    const ALL: [Dir; 4] = [Self::LEFT, Self::RIGHT, Self::UP, Self::DOWN];
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

    fn plot_count(&self) -> usize {
        // for row in &self.grid {
        //     println!("{} {}", row.iter().collect::<String>(), row.iter().filter(|&&c| c != '#').count());
        // }
        //
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c != '#').count())
            .sum()
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

    let region_plot_count = grid.plot_count();
    println!("region plot count: {}", region_plot_count);

    assert!(grid.rows == grid.cols);

    // The first region is fillable and has an odd step count
    assert!(steps % 2 == 1);


    // Steps remaning after reaching the center of any edge of the first region
    let mut remaining_steps = steps - (grid.rows - 1) / 2;
    assert!(remaining_steps % grid.rows == 0);
    let region_steps = remaining_steps / grid.rows;
    assert!(region_steps % 2 == 0);
    println!("region steps: {}", region_steps);

    // Stepping into the next region
    remaining_steps -= 1;


    // Count all regions reachable with 131*2-1 steps remaining.
    let even_filled_region = (region_steps - 2) * (region_steps - 2);
    let odd_filled_region = (region_steps - 2 + 1) * (region_steps - 2 + 1);
    println!("filled regions: {} {}", even_filled_region, odd_filled_region);

    let even_region_plot_count = grid.walk(&grid.start, grid.rows * 2, &mut Memo::default());
    let odd_region_plot_count = grid.walk(&grid.start, grid.rows * 2 - 1, &mut Memo::default());
    println!("filled region plot count: {} {}", even_region_plot_count, odd_region_plot_count);

    remaining_steps -= (region_steps - 2) * grid.rows;
    assert!(remaining_steps == grid.rows * 2 - 1);

    println!("remaining_steps: {}", remaining_steps);

    // Count partial regions

    // Cardinal regions are reachable by the center edge.
    let north = grid.walk(
        &Pos { row: grid.rows - 1, col: grid.start.col },
        remaining_steps, &mut Memo::default());

    let south = grid.walk(
        &Pos { row: 0, col: grid.start.col },
        remaining_steps, &mut Memo::default());

    let east = grid.walk(
        &Pos { row: grid.start.row, col: 0 },
        remaining_steps, &mut Memo::default());

    let west = grid.walk(
        &Pos { row: grid.start.row, col: grid.cols - 1 },
        remaining_steps, &mut Memo::default());

    println!("{} {} {} {}", north, south, east, west);


    // Cardinal regions are reachable by the center edge.
    let north2 = grid.walk(
        &Pos { row: grid.rows - 1, col: grid.start.col },
        remaining_steps - grid.rows, &mut Memo::default());

    let south2 = grid.walk(
        &Pos { row: 0, col: grid.start.col },
        remaining_steps - grid.rows, &mut Memo::default());

    let east2 = grid.walk(
        &Pos { row: grid.start.row, col: 0 },
        remaining_steps - grid.rows, &mut Memo::default());

    let west2 = grid.walk(
        &Pos { row: grid.start.row, col: grid.cols - 1 },
        remaining_steps - grid.rows, &mut Memo::default());

    println!("{} {} {} {}", north2, south2, east2, west2);
    // Diagonal regions are reachable by the corner.
    // It takes another 66 steps to get there.
    remaining_steps -= (grid.rows - 1) / 2 + 1;
    println!("remaining_steps: {} {}", remaining_steps, remaining_steps - grid.rows);

    let northeast = grid.walk(
        &Pos { row: grid.rows - 1, col: 0 },
        remaining_steps, &mut Memo::default());

    let northwest = grid.walk(
        &Pos { row: grid.rows - 1, col: grid.cols - 1 },
        remaining_steps, &mut Memo::default());

    let southeast = grid.walk(
        &Pos { row: 0, col: 0 },
        remaining_steps, &mut Memo::default());

    let southwest = grid.walk(
        &Pos { row: 0, col: grid.cols - 1 },
        remaining_steps, &mut Memo::default());

    println!("{} {} {} {}", northeast, southeast, northwest, southwest);

    let northeast2 = grid.walk(
        &Pos { row: grid.rows - 1, col: 0 },
        remaining_steps - grid.rows, &mut Memo::default());

    let northwest2 = grid.walk(
        &Pos { row: grid.rows - 1, col: grid.cols - 1 },
        remaining_steps - grid.rows, &mut Memo::default());

    let southeast2 = grid.walk(
        &Pos { row: 0, col: 0 },
        remaining_steps - grid.rows, &mut Memo::default());

    let southwest2 = grid.walk(
        &Pos { row: 0, col: grid.cols - 1 },
        remaining_steps - grid.rows, &mut Memo::default());

    println!("{} {} {} {}", northeast2, southeast2, northwest2, southwest2);

    println!("{} {}",
             4 * odd_region_plot_count - (northeast + southeast + northwest + southwest),
             northeast2 + southeast2 + northwest2 + southwest2);

    println!("{}",
             odd_filled_region * odd_region_plot_count
                 + even_filled_region * even_region_plot_count
                 + north + south + east + west // 4 O
                 + north2 + south2 + east2 + west2 // 4 partial E
                 + 4 * (region_steps - 2) * even_region_plot_count
                 + (region_steps - 1) * (northeast + southeast + northwest + southwest) // partial O
                 + region_steps * (northeast2 + southeast2 + northwest2 + southwest2)); // partial E;
}
