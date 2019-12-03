use std::collections::HashSet;
use failure::_core::iter::FromIterator;
use failure::_core::convert::TryInto;
use failure::_core::fmt;
use std::ops::BitAnd;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y + other.y).abs()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Wire {
    points: Vec<Point>
}

impl Wire {
    fn from_string(input: &String) -> Wire {
        let steps: Vec<String> = input.split(",").map(|i| i.to_string()).collect();

        // Store a list of points, each 1 unit from its neighbours
        let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];

        for step in &steps {
            let (direction, distance) = step.split_at(1);
            let distance = distance.parse::<i32>().expect("Not a number");
            for _ in 0..distance {
                let mut next = points.last().expect("No points!").clone();

                match direction {
                    "L" => next.x -= 1,
                    "R" => next.x += 1,
                    "U" => next.y += 1,
                    "D" => next.y -= 1,
                    _ => panic!("Unknown direction found `{}`", direction),
                }

                points.push(next);
            }
        }

        Wire { points }
    }

    // Since each point is 1 distance, just count the points until we find it
    fn distance_to_point(&self, point: &Point) -> i32 {
        self.points.iter().position(|p| p == point).expect("Point not in wire").try_into().unwrap()
    }

    fn points_as_set(&self) -> HashSet<&Point> {
        HashSet::from_iter(&self.points)
    }
}

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let o = Point { x: 0, y: 0 };

    let wires: Vec<Wire> = input.lines().map(|s| s.to_string()).map(|w| Wire::from_string(&w)).collect();

    let mut intersections = wires.iter().skip(1).fold(
        wires[0].points_as_set(),
        |a, w| a.bitand(&w.points_as_set())
    );
    // The origin is not a valid intersection, so just get rid of it
    intersections.remove(&o);

    answers.push(format!("{}",
                         intersections.iter().map(
                             |p| o.manhattan_distance(p)
                         ).min().expect("No intersections")
    ));

    answers.push(format!("{}",
                         intersections.iter().map(
                             |p| wires.iter().map(
                                 |w| w.distance_to_point(p)
                             ).sum::<i32>()
                         ).min().expect("No intersections")
    ));

    answers
}
