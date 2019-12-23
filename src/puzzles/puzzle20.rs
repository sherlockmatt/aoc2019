use std::collections::{HashMap, HashSet, BinaryHeap};
use crate::utils::Pos;
use failure::_core::cmp::Reverse;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut room_map = HashMap::new();
    input.trim_matches('\n').lines().enumerate().for_each(|(y, line)|
        line.chars().enumerate().for_each(|(x, c)| { if c != ' ' { room_map.insert(Pos::new(x, y), c); } })
    );
    let max_pos = *room_map.iter().max_by_key(|(p, _)| **p).unwrap().0;

    let mut portal_map = HashMap::new();
    let mut reverse_portal_map = HashMap::new();
    room_map.iter().for_each(|(p, c)| {
        if *c == '.' {
            if room_map.get(&Pos::new(p.x, p.y - 1)).unwrap().is_alphabetic() {
                let portal_name = format!("{}{}", room_map.get(&Pos::new(p.x, p.y - 2)).unwrap(), room_map.get(&Pos::new(p.x, p.y - 1)).unwrap());
                let portal = portal_map.entry(portal_name.clone()).or_insert(Portal::new());
                if p.y == 2 {
                    portal.outer = Some(*p);
                } else {
                    portal.inner = Some(*p);
                }
                reverse_portal_map.insert(*p, portal_name);
            } else if room_map.get(&Pos::new(p.x, p.y + 1)).unwrap().is_alphabetic() {
                let portal_name = format!("{}{}", room_map.get(&Pos::new(p.x, p.y + 1)).unwrap(), room_map.get(&Pos::new(p.x, p.y + 2)).unwrap());
                let portal = portal_map.entry(portal_name.clone()).or_insert(Portal::new());
                if p.y == max_pos.y {
                    portal.outer = Some(*p);
                } else {
                    portal.inner = Some(*p);
                }
                reverse_portal_map.insert(*p, portal_name);
            } else if room_map.get(&Pos::new(p.x - 1, p.y)).unwrap().is_alphabetic() {
                let portal_name = format!("{}{}", room_map.get(&Pos::new(p.x - 2, p.y)).unwrap(), room_map.get(&Pos::new(p.x - 1, p.y)).unwrap());
                let portal = portal_map.entry(portal_name.clone()).or_insert(Portal::new());
                if p.x == 2 {
                    portal.outer = Some(*p);
                } else {
                    portal.inner = Some(*p);
                }
                reverse_portal_map.insert(*p, portal_name);
            } else if room_map.get(&Pos::new(p.x + 1, p.y)).unwrap().is_alphabetic() {
                let portal_name = format!("{}{}", room_map.get(&Pos::new(p.x + 1, p.y)).unwrap(), room_map.get(&Pos::new(p.x + 2, p.y)).unwrap());
                let portal = portal_map.entry(portal_name.clone()).or_insert(Portal::new());
                if p.x == max_pos.x {
                    portal.outer = Some(*p);
                } else {
                    portal.inner = Some(*p);
                }
                reverse_portal_map.insert(*p, portal_name);
            }
        }
    });

    let aa_pos = portal_map.get(&*"AA".to_string()).unwrap().outer.unwrap();
    let zz_pos = portal_map.get(&*"ZZ".to_string()).unwrap().outer.unwrap();

    let mut to_check = Vec::new();
    to_check.push(aa_pos);

    let mut subgraphs = HashMap::new();

    while let Some(subgraph_start) = to_check.pop() {
        let mut explore_path = Vec::new();
        let mut visited = HashSet::new();
        explore_path.push(subgraph_start);

        while explore_path.len() > 0 {
            let current_pos = explore_path[explore_path.len() - 1];
            visited.insert(current_pos);

            if let Some(portal_name) = reverse_portal_map.get(&current_pos) {
                if current_pos != subgraph_start && *portal_name != "AA".to_string() {
                    let other_end;
                    let depth_delta;
                    if *portal_name == "ZZ".to_string() {
                        other_end = zz_pos;
                        depth_delta = 0;
                    } else {
                        let portal = portal_map.get(portal_name).unwrap();
                        if current_pos == portal.inner.unwrap() {
                            other_end = portal.outer.unwrap();
                            depth_delta = 1;
                        } else {
                            other_end = portal.inner.unwrap();
                            depth_delta = -1;
                        }
                        if !subgraphs.contains_key(&other_end) {
                            to_check.push(other_end);
                        }
                    };
                    subgraphs.entry(subgraph_start).or_insert(HashSet::new()).insert((other_end, explore_path.len(), depth_delta));
                }
            }

            let mut avail_pos = Vec::new();
            for compare_pos in vec![
                Pos::new(current_pos.x + 1, current_pos.y),
                Pos::new(current_pos.x, current_pos.y + 1),
                Pos::new(current_pos.x - 1, current_pos.y),
                Pos::new(current_pos.x, current_pos.y - 1),
            ] {
                if !visited.contains(&compare_pos) && *room_map.get(&compare_pos).unwrap_or_else(|| panic!("Nothing found at pos {:?}", current_pos)) == '.' {
                    avail_pos.push(compare_pos);
                }
            };
            if avail_pos.len() == 0 {
                explore_path.pop(); // Backtrack
            } else {
                // Only record one move, so that we always have a single path from the start to our current point
                explore_path.push(avail_pos[0]);
            }
        }
    }

    // Now we have all the subgraphs, do a priority queue to traverse them
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, aa_pos)));

    while let Some(Reverse((distance, position))) = queue.pop() {
        if position == zz_pos {
            answers.push(format!("{}", distance - 1));
            break;
        }
        for next_pos in subgraphs.get(&position).unwrap() {
            queue.push(Reverse((distance + next_pos.1, next_pos.0)));
        }
    }

    // Re-do the traverse, but this time worry about the depth
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, aa_pos)));

    while let Some(Reverse((distance, depth, position))) = queue.pop() {
        if position == zz_pos && depth == 0 {
            answers.push(format!("{}", distance - 1));
            break;
        }
        for next_pos in subgraphs.get(&position).unwrap() {
            if next_pos.0 != aa_pos && !(depth != 0 && next_pos.0 == zz_pos) && depth + next_pos.2 >= 0 {
                queue.push(Reverse((distance + next_pos.1, depth + next_pos.2, next_pos.0)));
            }
        }
    }

    answers
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Portal {
    inner: Option<Pos>,
    outer: Option<Pos>
}

impl Portal {
    fn new() -> Portal {
        Portal { inner: None, outer: None }
    }
}
