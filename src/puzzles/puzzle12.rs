use itertools::Itertools;
use failure::_core::cmp::Ordering;
use num::integer::lcm;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let original_pos: Vec<(i64, i64, i64)> = input.trim().lines().map(
            |moon| moon.trim_end_matches('>').split(',').map(|x| x.split('=').last().unwrap().parse::<i64>().unwrap_or_else(|n| panic!("Invalid number found in input `{}`", n))).next_tuple().unwrap()
        ).collect();

    let mut pos: Vec<(i64, i64, i64)> = original_pos.clone();
    let mut vel: Vec<(i64, i64, i64)> = (0..pos.len()).map(|_| (0, 0, 0)).collect();
    for _ in 0..1000 {
        for i in 0..pos.len() {
            for j in 0..pos.len() {
                if i != j {
                    vel[i] = (
                        vel[i].0 + gravity(pos[i].0, pos[j].0),
                        vel[i].1 + gravity(pos[i].1, pos[j].1),
                        vel[i].2 + gravity(pos[i].2, pos[j].2)
                    );
                }
            }
        }
        pos = pos.iter().zip(vel.iter()).map(|((x, y, z), (dx, dy, dz))| (x + dx, y + dy, z + dz)).collect();
    }

    answers.push(format!("{}", pos.iter().zip(vel.iter()).map(|((x, y, z), (dx, dy, dz))| (x.abs() + y.abs() + z.abs()) * (dx.abs() + dy.abs() + dz.abs())).sum::<i64>()));

    // Figure out the cycle in each dimension (which is possible because they're independent)
    // then use the lowest common multiple to see when they all intersect again
    let pos_by_dimension: Vec<Vec<i64>> = vec![
        original_pos.iter().map(|(x, _, _)| *x).collect(),
        original_pos.iter().map(|(_, y, _)| *y).collect(),
        original_pos.iter().map(|(_, _, z)| *z).collect()
    ];
    let mut cycle_periods: Vec<i64> = vec![];
    for d in pos_by_dimension {
        let mut is_looping = true;
        let mut d_pos = d.clone();
        let mut vel: Vec<i64>  = d_pos.iter().map(|_| 0).collect();
        let mut step = 0;
        while is_looping {
            for i in 0..d_pos.len() {
                for j in 0..d_pos.len() {
                    if i != j {
                        vel[i] += gravity(d_pos[i], d_pos[j]);
                    }
                }
            }
            d_pos = d_pos.iter().zip(vel.iter()).map(|(p, v)| p + v).collect();
            step += 1;
            is_looping = !(d_pos == d && vel.iter().all(|v| *v == 0));
        }
        cycle_periods.push(step);
    }

    answers.push(format!("{}", cycle_periods.iter().fold(1i64, |a, b| lcm(a, *b))));

    answers
}

fn gravity(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0
    }
}


// Below are some of the experiments I did to attempt to calculate the cycle period directly from the initial position
// This works easily for 2 objects, but I didn't have enough time to generalise this to 4 objects
//
// answers.push(format!("{:?}", (0..=11).map(|n| (n, inverse_triangle_number(n))).collect::<Vec<(i64, (i64, i64))>>()));
//
// answers.push(format!("{:?}", (0..=22).map(|n| {
//     let (itn, r) = inverse_triangle_number(n/2);
//     (n, 4 * (itn + 1) + if r == 0 && n % 2 == 0 { -2 } else { 0 })
// }).collect::<Vec<(i64, i64)>>()));
//
// answers.push(format!("{}", (0..3).cartesian_product((0..pos.len()).cartesian_product(0..pos.len()).filter(|(a, b)| a != b).unique_by(|(a, b)| (a+1) * (b+1))).fold(1,
//     |acc, (d, (a, b))| {
//         println!("acc={} posa={:?} posb={:?} d={}", acc, pos[a], pos[b], d);
//         let n = ([pos[a].0, pos[a].1, pos[a].2][d] - [pos[b].0, pos[b].1, pos[b].2][d]).abs();
//         let (itm, r) = inverse_triangle_number(n/2);
//         println!("n={} n/2={} itm={} r={} final={}", n, n/2, itm, r, 4 * (itm + 1) + if r == 0 && n % 2 == 0 { -2 } else { 0 });
//         lcm(acc, 4 * (itm + 1) + if r == 0 && n % 2 == 0 { -2 } else { 0 })
//     }
// )));
//
// fn inverse_triangle_number(mut n: i64) -> (i64, i64) {
//     let mut d = 1;
//     while n >= d {
//         n -= d;
//         d += 1;
//     }
//     (d - 1, n)
// }
