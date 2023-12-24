use std::env;
use std::fs::read_to_string;

use itertools::Itertools;

struct XYZ {
    x: i64,
    y: i64,
    z: i64,
}

impl XYZ {
    fn from(s: &str) -> Self {
        let (x, y, z) = s
            .split(", ")
            .map(|n| n.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y, z }
    }

    fn xy_slope(&self) -> f64 {
        (self.y as f64) / (self.x as f64)
    }

    fn xy_equals(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Stone {
    pos: XYZ,
    velocity: XYZ,
}

impl Stone {
    fn from(s: &str) -> Self {
        let (pos, velocity) = s
            .split(" @ ")
            .map(XYZ::from)
            .collect_tuple()
            .unwrap();
        Self { pos, velocity }
    }

    fn xy_intersect(a: &Stone, b: &Stone) -> Option<(f64, f64)> {
        if a.velocity.xy_equals(&b.velocity) {
            return None;
        }

        let slope_a = a.velocity.xy_slope();
        let slope_b = b.velocity.xy_slope();
        let ax0 = a.pos.x as f64;
        let ay0 = a.pos.y as f64;
        let bx0 = b.pos.x as f64;
        let by0 = b.pos.y as f64;

        const EPSILON: f64 = 1e-12;
        if (slope_a - slope_b).abs() < EPSILON {
            return None;
        }

        // y = y0 + (x - x0) * slope_a;
        // ay0 + (x - ax0) * slope_a = by0 + (x - bx0) * slope_b
        // x = ((slope_a*ax0 - ay0) - (slope_b*bx0 - by0)) / (slope_a - slope_b)

        let x = ((slope_a * ax0 - ay0) - (slope_b * bx0 - by0)) / (slope_a - slope_b);
        let ya = ay0 + (x - ax0) * slope_a;
        let yb = by0 + (x - bx0) * slope_b;
        let ta = (x - ax0) / (a.velocity.x as f64);
        let tb = (x - bx0) / (b.velocity.x as f64);
        if ta < 0.0 || tb < 0.0 {
            return None;
        }
        //assert!((ya - yb).abs() < EPSILON);

        Some((x, ya))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let start = args[2].parse().unwrap();
    let end = args[3].parse().unwrap();

    let stones = data
        .lines()
        .map(Stone::from)
        .collect::<Vec<_>>();

    let mut count = 0;
    for (i, a) in stones.iter().enumerate() {
        for (j, b) in stones[0..i].iter().enumerate() {
            // println!("{}, {}, {} @ {}, {}, {}", a.pos.x, a.pos.y, a.pos.z, a.velocity.x, a.velocity.y, a.velocity.z);
            // println!("{}, {}, {} @ {}, {}, {}", b.pos.x, b.pos.y, b.pos.z, b.velocity.x, b.velocity.y, b.velocity.z);
            if let Some((x, y)) = Stone::xy_intersect(a, b) {
                // println!("x={}, y={}", x, y);
                if x >= start && x <= end && y >= start && y <= end {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
