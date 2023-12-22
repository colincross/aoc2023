use std::{env, ops};
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    z: usize,
    y: usize,
    x: usize,
}

impl Pos {
    fn from(s: &str) -> Self {
        let mut xyz = s.splitn(3, ",");
        let x = xyz.next().unwrap().parse().unwrap();
        let y = xyz.next().unwrap().parse().unwrap();
        let z = xyz.next().unwrap().parse().unwrap();

        Self { x, y, z }
    }

    fn magnitude(&self) -> usize {
        self.z + self.y + self.x
    }

    fn unit_vector(&self) -> Self {
        let x = self.x.clamp(0, 1);
        let y = self.y.clamp(0, 1);
        let z = self.z.clamp(0, 1);
        Self { x, y, z }
    }

    fn fall(&self, distance: usize) -> Self {
        let x = self.x;
        let y = self.y;
        let z = self.z - distance;
        Self { x, y, z }
    }
}

impl ops::Sub<Self> for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self { x, y, z }
    }
}

impl ops::Add<Self> for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

impl ops::Mul<usize> for Pos {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self { x, y, z }
    }
}


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Brick {
    start: Pos,
    end: Pos,
    len: usize,
    dir: Pos,
}

impl Brick {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once("~").unwrap();
        let start = Pos::from(start);
        let end = Pos::from(end);

        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        assert!(start <= end);

        let len = (end - start).magnitude() + 1;
        let dir = (end - start).unit_vector();
        Self { start, end, len, dir }
    }

    fn positions(&self) -> impl Iterator<Item=Pos> + '_ {
        (0..self.len).map(|i| self.start + self.dir * i)
    }

    fn distance_above(&self, other: &Self) -> Option<usize> {
        let mut min_z = usize::MAX;
        for self_pos in self.positions() {
            for other_pos in other.positions() {
                if self_pos.x == other_pos.x && self_pos.y == other_pos.y {
                    if min_z > self_pos.z - other_pos.z {
                        min_z = self_pos.z - other_pos.z;
                    }
                }
            }
        }
        if min_z < usize::MAX {
            Some(min_z)
        } else {
            None
        }
    }

    fn fall(&self, distance: usize) -> Self {
        let start = self.start.fall(distance);
        let end = self.end.fall(distance);
        let len = self.len;
        let dir = self.dir;
        Self { start, end, len, dir }
    }
}

fn fall(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let brick = &bricks[i];
        let mut fall_distance = usize::MAX;
        for other_brick in &bricks[0..i] {
            if let Some(distance) = brick.distance_above(other_brick) {
                if fall_distance > distance - 1 {
                    fall_distance = distance - 1;
                }
            }
        }
        fall_distance = fall_distance.clamp(0, brick.start.z - 1);
        bricks[i] = brick.fall(fall_distance);
    }
}

fn supports(bricks: &Vec<Brick>) -> Vec<(Vec<usize>)> {
    bricks
        .iter()
        .enumerate()
        .map(|(i, brick)| {
            let mut supports = Vec::<_>::new();
            for j in 0..i {
                let other_brick = &bricks[j];
                if let Some(distance) = brick.distance_above(other_brick) {
                    if distance == 1 {
                        supports.push(j);
                    }
                }
            }
            supports
        })
        .collect::<Vec<_>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut bricks = data
        .lines()
        .map(Brick::from)
        .collect::<Vec<_>>();

    // Sorts by lowest Z first
    bricks.sort();

    fall(&mut bricks);

    bricks.sort();


    let sup = &supports(&bricks);


    let mut cannot_disintigrate = sup
        .iter()
        .filter(|sup| sup.len() == 1)
        .flatten()
        .collect::<Vec<&usize>>();

    cannot_disintigrate.sort();
    cannot_disintigrate.dedup();

    println!("{}", bricks.len() - cannot_disintigrate.len());
}
