use std::env;
use std::fs::read_to_string;

use eqsolver::multivariable::MultiVarNewton;
use eqsolver::nalgebra::*;
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

    fn p(&self, xyz: usize) -> f64 {
        let p = match xyz {
            0 => self.pos.x,
            1 => self.pos.y,
            2 => self.pos.z,
            _ => panic!(),
        };
        p as f64
    }

    fn v(&self, xyz: usize) -> f64 {
        let v = match xyz {
            0 => self.velocity.x,
            1 => self.velocity.y,
            2 => self.velocity.z,
            _ => panic!(),
        };
        v as f64
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
    println!("Part 1: {}", count);

    for (i, stone) in stones.iter().enumerate().take(3) {
        fn print(i: usize, pvar: char, pos: i64, vvar: char, velocity: i64, tvar: char) {
            println!("{} + ({}) * {} = {} + {} * {}", pos, velocity, tvar, pvar, vvar, tvar);
        }
        print(i, 'x', stone.pos.x, 'a', stone.velocity.x, ('t' as usize + i) as u8 as char);
        print(i, 'y', stone.pos.y, 'b', stone.velocity.y, ('t' as usize + i) as u8 as char);
        print(i, 'z', stone.pos.z, 'c', stone.velocity.z, ('t' as usize + i) as u8 as char);
    }

    // The equations for the intersection of a hailstone with the rock are of the form:
    // Xn + Vn * tn = x + vx * tn
    // where :
    //  Xn is the initial position of hailstone n (a known constant)
    //  Vn is the initial velocity of hailstone n (a known constant)
    //  tn is the time the hailstone and rock will intersect
    //  x is the initial position of the rock
    //  vx is the initial velocity of the rock
    // This can be repeated for y and z, and for additional hailstones.
    // It can be rewritten as:
    //  (vx - Vn) * tn + x - Xn = 0
    //
    // We also need derivatives:
    //  d/dx = 1
    //  d/dvx = tn
    //  d/dtn = (vx - Vn)

    // Variables to solve for:
    // v[0] = x, v[1] = y, v[2] = z,
    // v[3] = vx, v[4] = vy, v[5] = vz,
    // v[6] = t0, v[7] = t1, v[8] = t2

    pub type Vector9<T> = Matrix<T, U9, U1, ArrayStorage<T, 9, 1>>;
    pub type RowVector9<T> = Matrix<T, U1, U9, ArrayStorage<T, 1, 9>>;
    pub type Matrix9<T> = Matrix<T, U9, U9, ArrayStorage<T, 9, 9>>;

    let f = |v: Vector9<f64>, xyz: usize, n: usize| {
        (v[3 + xyz] - stones[n].v(xyz)) * v[6 + n] + v[xyz] - stones[n].p(xyz)
    };

    let j = |v: Vector9<f64>, xyz: usize, n: usize| {
        let mut j = RowVector9::<f64>::from_element(0.0);
        // d/dx = -1
        j[xyz] = 1.0;
        // d/dvx = -tn
        j[3 + xyz] = v[6 + n];
        // d/dtn = (Vn - vx)
        j[6 + n] = v[3 + xyz] - stones[n].v(xyz);
        j
    };

    let F = |v: Vector9<f64>| {
        let data = [[
            f(v, 0, 0),
            f(v, 1, 0),
            f(v, 2, 0),
            f(v, 0, 1),
            f(v, 1, 1),
            f(v, 2, 1),
            f(v, 0, 2),
            f(v, 1, 2),
            f(v, 2, 2),
        ]];
        Vector9::from_data(ArrayStorage(data))
    };

    let J = |v: Vector9<f64>| Matrix9::from_rows(&[
        j(v, 0, 0),
        j(v, 1, 0),
        j(v, 2, 0),
        j(v, 0, 1),
        j(v, 1, 1),
        j(v, 2, 1),
        j(v, 0, 2),
        j(v, 1, 2),
        j(v, 2, 2),
    ]);

    let start_data = Vector9::from_data(ArrayStorage([[
        stones[3].p(0),
        stones[3].p(1),
        stones[3].p(2),
        stones[3].v(0),
        stones[3].v(1),
        stones[3].v(2),
        start,
        end,
        (start + end) / 2.,
    ]]));


    let solution = MultiVarNewton::new(F, J)
        .with_itermax(10000000)
        .with_tol(0.1)
        .solve(start_data);
    let solution = solution.unwrap();


    println!("Part 2: {}", solution[0] + solution[1] + solution[2]);
}
