use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::env;
use std::fs::read_to_string;

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
}


#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MemoItem {
    pos: Pos,
    reachable: Vec<Pos>,
}

#[derive(Default)]
struct Memo {
    memo: HashMap<MemoItem, Option<(usize, Vec<Pos>)>>,
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
        Self { grid, rows, cols }
    }

    fn at(&self, pos: &Pos) -> char {
        self.grid[pos.row][pos.col]
    }

    fn iter(&self) -> impl Iterator<Item=Pos> + '_ {
        (0..self.rows)
            .map(move |row| (0..self.cols)
                .map(move |col| Pos { row, col }))
            .flatten()
    }

    fn junctions(&self) -> Vec<Pos> {
        self.iter()
            .filter(|pos| self.at(pos) != '#')
            .filter(|pos| self.junction_count(pos) > 2 || pos.row == 0 || pos.row == self.rows - 1)
            .collect()
    }

    fn connected(&self, pos: &Pos, targets: &[Pos]) -> Vec<(Pos, usize)> {
        self.dfs_targets(pos, pos, targets, 0)
    }

    fn reachable(&self, connections: &HashMap<Pos, Vec<(Pos, usize)>>, path: &Vec<Pos>, pos: &Pos) -> Vec<Pos> {
        let mut reachable = HashSet::new();
        self.reachable_recurse(connections, path, pos, &mut reachable);
        let mut list = reachable.into_iter().collect::<Vec<_>>();
        list.sort();
        // println!("reachable from ({},{}): {}", pos.row, pos.col,
        //          list
        //              .iter()
        //              .map(|pos| format!("({},{})", pos.row, pos.col))
        //              .collect::<Vec<_>>()
        //              .join(", "));
        list
    }

    fn reachable_recurse(&self, connections: &HashMap<Pos, Vec<(Pos, usize)>>, path: &Vec<Pos>, pos: &Pos,
                         reachable: &mut HashSet<Pos>) {
        for (connection, _len) in &connections[pos] {
            if reachable.contains(connection) || path.contains(connection) {
                continue;
            }
            reachable.insert(connection.clone());
            self.reachable_recurse(connections, path, connection, reachable)
        }
    }

    fn dfs_targets(&self, pos: &Pos, last_pos: &Pos, targets: &[Pos], len: usize) -> Vec<(Pos, usize)> {
        let mut reached = Vec::new();
        for dir in Dir::ALL {
            if let Some(new_pos) = self.next_pos(&pos, &dir) {
                if &new_pos == last_pos {
                    continue;
                }
                if self.at(&new_pos) != '#' {
                    if targets.contains(&new_pos) {
                        return vec![(new_pos, len + 1)];
                    }
                    reached.append(
                        &mut self.dfs_targets(&new_pos, pos, targets, len + 1));
                }
            }
        }
        reached
    }

    fn junction_count(&self, pos: &Pos) -> usize {
        Dir::ALL
            .iter()
            .map(|dir| self.next_pos(pos, dir))
            .flatten()
            .filter(|pos| self.at(pos) != '#')
            .count()
    }

    fn dfs(&self, connections: &HashMap<Pos, Vec<(Pos, usize)>>, path: &mut Vec<Pos>, memo: &mut Memo) -> Option<(usize, Vec<Pos>)> {
        let pos = path.last().unwrap().clone();
        if pos.row == self.rows - 1 {
            return Some((0, vec![pos.clone()]));
        }

        let memo_key = MemoItem {
            pos: pos.clone(),
            reachable: self.reachable(connections, path, &pos),
        };
        let memo_entry = memo.memo.entry(memo_key.clone());
        if let Entry::Occupied(entry) = memo_entry {
            return entry.get().clone();
        }

        let mut max_len = None;
        let mut max_path = Vec::new();
        for (connection, len) in &connections[&pos] {
            if path.contains(connection) {
                continue;
            }
            path.push(connection.clone());
            if let Some((new_len, mut new_path)) = self.dfs(connections, path, memo) {
                let new_path_len = new_len + len;
                if max_len.is_none() || max_len.unwrap() < new_path_len {
                    max_len = Some(new_path_len);
                    new_path.push(pos.clone());
                    max_path = new_path;
                }
            }
            path.pop();
        }


        return if let Some(len) = max_len {
            memo.memo.insert(memo_key, Some((len, max_path.clone())));
            Some((len, max_path))
        } else {
            memo.memo.insert(memo_key, None);
            None
        };
    }

    fn find_min_value(&self) -> usize {
        let junctions = self.junctions();
        let connections = HashMap::from_iter(
            junctions
                .iter()
                .map(|junction| (junction.clone(), self.connected(junction, &junctions)))
                .collect::<Vec<_>>());


        let mut path = Vec::with_capacity(junctions.len());
        path.push(Pos { row: 0, col: 1 });

        let (length, path) = self.dfs(&connections, &mut path, &mut Memo::default()).expect("length");

        println!("{}", path.iter().rev().map(|pos| format!("({},{})", pos.row, pos.col)).collect::<Vec<_>>().join(" -> "));
        length
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let grid = Grid::new(&data);

    println!("{}", grid.find_min_value());
}
