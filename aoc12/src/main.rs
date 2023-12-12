use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Row {
    springs: Vec<char>,
    broken_counts: Vec<usize>,
    total_broken: usize,
}

#[derive(Clone, Debug)]
struct Answer<'a> {
    row: &'a Row,
    gaps: Vec<usize>,
    total_gaps: usize,
}

struct Memo {
    failing_sums: Vec<Vec<Option<usize>>>,
}

impl Memo {
    fn new(gaps: usize, row_length: usize) -> Self {
        Self {
            failing_sums: vec![vec![None; row_length]; gaps + 1]
        }
    }
    fn memoize_count(&mut self, answer: &Answer, count: usize) {
        self.failing_sums[answer.gaps.len()][answer.gaps.iter().sum::<usize>()] = Some(count)
    }

    fn reuse_count(&self, answer: &Answer) -> Option<usize> {
        self.failing_sums[answer.gaps.len()][answer.gaps.iter().sum::<usize>()]
    }
}

impl<'a> Answer<'a> {
    fn empty(row: &'a Row) -> Self {
        Self {
            row,
            gaps: Default::default(),
            total_gaps: 0,
        }
    }

    fn valid(&self) -> Option<bool> {
        let mut offset = 0;
        if self.total_gaps + self.row.total_broken > self.row.springs.len() {
            return Some(false);
        }
        for (i, gap) in self.gaps.iter().enumerate() {
            if self.row.springs[offset..offset + gap]
                .iter()
                .any(|c| *c == '#') {
                return Some(false);
            }
            offset += gap;
            let filled = self.row.broken_counts[i];
            if self.row.springs[offset..offset + filled]
                .iter()
                .any(|c| *c == '.') {
                return Some(false);
            }
            offset += filled;
        }
        if self.gaps.len() == self.row.broken_counts.len() {
            if self.row.springs[offset..self.row.springs.len()]
                .iter()
                .any(|c| *c == '#') {
                return Some(false);
            }
            return Some(true);
        }
        return None;
    }

    fn add_gap(&self, gap: usize) -> Self {
        let mut gaps = self.gaps.clone();
        gaps.push(gap);
        Self {
            row: self.row,
            gaps,
            total_gaps: self.total_gaps + gap,
        }
    }
}

impl Row {
    fn new(line: &str) -> Self {
        let words = line.split_whitespace().collect::<Vec<_>>();

        let repeat = 5;

        let springs = vec![words[0]; repeat].join("?")
            .chars().collect::<Vec<_>>();

        let broken_counts = words[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .repeat(repeat);

        let total_broken = broken_counts.iter().sum::<usize>();

        Self { springs, broken_counts, total_broken }
    }


    fn recurse(&self, answer: &Answer, memo: &mut Memo) -> usize {
        let reuse = memo.reuse_count(answer);
        if reuse.is_some() {
            return reuse.unwrap();
        }

        let valid = answer.valid();
        if valid.is_some() {
            if valid.unwrap() {
                memo.memoize_count(answer, 1);
                return 1;
            } else {
                return 0;
            }
        }

        let mut count: usize = 0;
        for gap in 0..=self.springs.len() - self.total_broken - answer.total_gaps {
            if gap == 0 && answer.gaps.len() > 0 {
                continue;
            }
            count += self.recurse(&answer.add_gap(gap), memo);
        }
        memo.memoize_count(answer, count);
        return count;
    }

    fn combos(&self) -> usize {
        let mut memo = Memo::new(self.broken_counts.len(), self.springs.len());
        let combos = self.recurse(&Answer::empty(self), &mut memo);
        dbg!(combos);
        combos
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let rows = data
        .lines()
        .map(Row::new)
        .collect::<Vec<_>>();

    println!("{}", rows
        .iter()
        .map(|row| row.combos())
        .sum::<usize>());
}
