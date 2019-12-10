use std::collections::HashMap;
use failure::_core::convert::TryFrom;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut data: HashMap<(u16, u16), bool> = HashMap::new();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            data.insert((j as u16, i as u16), c == '#');
        }
    }

    // For each location that has an asteroid, loop over all other locations with asteroids
    let (best_location, mut best_map) = data.iter().filter(|(_, &b)|
        b
    ).map(|((x1, y1), _)|
        (
            (*x1, *y1),
            data.iter()
                .filter(|((x2, y2), &b)| b && (x1 != x2 || y1 != y2))
                // Put each other asteroid into a bin based on its direction
                .fold(HashMap::new(), |mut a, ((x2, y2), _)| {
                    a.entry(direction_to(*x1, *y1, *x2, *y2)).or_insert(Vec::new()).push((*x2, *y2));
                    a
                })
        )
    // Find the asteroid with the most bins (i.e. the most unique directions it can see an asteroid in)
    ).max_by_key(|((_, _), asteroids)| asteroids.len()).unwrap();

    // Print the number of bins (asteroids it can see)
    answers.push(format!("{}", best_map.len()));

    // Sort each vector by distance from our chosen point
    // Sort the bins by the angle from our chosen point
    // Cycle it so .next() will auto-repeat
    // Skip to the first point that's straight up, since the sort puts directly right first
    let mut asteroids = best_map.iter_mut().map(|(k, v)| (k, v.iter().sorted_by(|pos_a, pos_b| distance_to(best_location, **pos_b).partial_cmp(&distance_to(best_location, **pos_a)).unwrap()).collect::<Vec<&(u16, u16)>>())).sorted_by(|((dx1, dy1), _), ((dx2, dy2), _)| f32::try_from(*dy1).unwrap().atan2(f32::try_from(*dx1).unwrap()).partial_cmp(&f32::try_from(*dy2).unwrap().atan2(f32::try_from(*dx2).unwrap())).unwrap()).cycle().skip_while(|((dx, dy), _)| dx < &0 || dy >= &0);
    let mut last_destroyed = (&0,&0);
    let mut num_destroyed = 0;
    // Destroy (pop) asteroids from the next bin, until we've destroyed 200
    while num_destroyed < 200 {
        match asteroids.next().unwrap().1.pop() {
            None => continue,
            Some((x, y)) => {
                num_destroyed += 1;
                last_destroyed = (x, y);
            }
        }
    }
    answers.push(format!("{}{:02}", last_destroyed.0, last_destroyed.1));

    answers
}

fn distance_to(pos_a: (u16, u16), pos_b: (u16, u16)) -> f32 {
    (f32::try_from(pos_b.0).unwrap() - f32::try_from(pos_a.0).unwrap()).hypot(f32::try_from(pos_b.1).unwrap() - f32::try_from(pos_a.1).unwrap())
}

fn direction_to(x1: u16, y1: u16, x2: u16, y2: u16) -> (i16, i16) {
    let dx = i16::try_from(x2).unwrap() - i16::try_from(x1).unwrap();
    let dy = i16::try_from(y2).unwrap() - i16::try_from(y1).unwrap();
    let g = gcd(dx, dy);
    (dx / g, dy / g)
}

fn gcd(a: i16, b: i16) -> i16 {
    match (a.abs(), b.abs()) {
        (0, n) => n,
        (n, 0) => n,
        (mut x, mut y) => {
            while y != 0 {
                let t = y;
                y = x % y;
                x = t;
            }
            x
        }
    }
}
