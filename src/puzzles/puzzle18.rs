use std::collections::{HashMap, HashSet, BinaryHeap};
use failure::_core::cmp::Ordering;
use crate::utils::{Pos, astar};

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut room_map: HashMap<Pos, char> = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, line)|
        line.chars().enumerate().for_each(|(x, c)| {
            room_map.insert(Pos::new(x, y), c);
        })
    );

    let initial_start_pos = room_map.iter().find(|(_, c)| **c == '@').unwrap().0.clone();
    room_map.insert(initial_start_pos, '1');

    answers.push(format!("{}", shortest_route(&room_map)));

    room_map.insert(Pos::new(initial_start_pos.x - 1, initial_start_pos.y - 1), '1');
    room_map.insert(Pos::new(initial_start_pos.x, initial_start_pos.y - 1), '#');
    room_map.insert(Pos::new(initial_start_pos.x + 1, initial_start_pos.y - 1), '2');
    room_map.insert(Pos::new(initial_start_pos.x - 1, initial_start_pos.y), '#');
    room_map.insert(initial_start_pos, '#');
    room_map.insert(Pos::new(initial_start_pos.x + 1, initial_start_pos.y), '#');
    room_map.insert(Pos::new(initial_start_pos.x - 1, initial_start_pos.y + 1), '3');
    room_map.insert(Pos::new(initial_start_pos.x, initial_start_pos.y + 1), '#');
    room_map.insert(Pos::new(initial_start_pos.x + 1, initial_start_pos.y + 1), '4');

    answers.push(format!("{}", shortest_route(&room_map)));

    answers
}

fn shortest_route(room_map: &HashMap<Pos, char>) -> usize {
    let start_positions: HashMap<char, Pos> = room_map.iter().filter(|(_, c)| (*c).is_numeric()).map(|(p, c)| (*c, *p)).collect();
    let keys: HashMap<char, Pos> = room_map.iter().filter(|(_, c)| (*c).is_lowercase()).map(|(p, c)| (*c, *p)).collect();

    let mut is_blocked_by: HashMap<char, HashSet<char>> = HashMap::new();
    for (_, &start_pos) in &start_positions {
        let mut explore_path = Vec::new();
        let mut visited = HashSet::new();
        let mut objects_passed = HashSet::new();
        explore_path.push(start_pos);

        while explore_path.len() > 0 {
            let current_pos = explore_path[explore_path.len() - 1];
            visited.insert(current_pos);
            let obj = room_map.get(&current_pos).unwrap_or_else(|| panic!("Nothing found at pos {:?}", current_pos));
            if (*obj).is_lowercase() {
                let block_entry = is_blocked_by.entry(*obj).or_insert(HashSet::new());
                objects_passed.iter().for_each(|c: &char| {
                    if c.is_uppercase() {
                        block_entry.insert(*c);
                    }
                });
            }
            if obj.is_alphabetic() {
                objects_passed.insert(*obj);
            }
            let mut avail_pos = Vec::new();
            for compare_pos in vec![
                Pos::new(current_pos.x + 1, current_pos.y),
                Pos::new(current_pos.x, current_pos.y + 1),
                Pos::new(current_pos.x - 1, current_pos.y),
                Pos::new(current_pos.x, current_pos.y - 1),
            ] {
                if !visited.contains(&compare_pos) && *room_map.get(&compare_pos).unwrap_or_else(|| panic!("Nothing found at pos {:?}", current_pos)) != '#' {
                    avail_pos.push(compare_pos);
                }
            };
            if avail_pos.len() == 0 {
                explore_path.pop(); // Backtrack
                objects_passed.remove(obj);
            } else {
                // Only record one move, so that we always have a single path from the start to our current point
                explore_path.push(avail_pos[0]);
            }
        }
    }

    let mut distances = HashMap::new();
    for (key_from, from_pos) in keys.iter().chain(start_positions.iter()) {
        for (key_to, to_pos) in keys.iter() {
            if key_from != key_to {
                distances.insert((*key_from, *key_to), astar(&room_map, *from_pos, *to_pos));
            }
        }
    }

    let mut routes = BinaryHeap::new();
    routes.push(Route::new(0, start_positions.keys().map(|c| *c).collect(), keys.keys().map(|i| *i).collect::<HashSet<char>>(), HashSet::new()));

    loop {
        let r = routes.pop().unwrap_or_else(|| panic!("No routes!"));
        if r.keys_to_get.len() == 0 {
            return r.distance_travelled;
        } else {
            is_blocked_by.iter().filter(|(c, b)|
                c.is_lowercase() && r.keys_to_get.contains(*c) && b.iter().all(|door| r.keys_acquired.contains(&door.to_lowercase().next().unwrap()))
            ).for_each(|(c, _)| {
                let mut next_keys_to_get = r.keys_to_get.clone();
                next_keys_to_get.remove(c);
                let mut next_keys_acquired = r.keys_acquired.clone();
                next_keys_acquired.insert(*c);
                for (i, current_character) in r.current_characters.iter().enumerate() {
                    if let Some(next_distance) = distances.get(&(*current_character, *c)).unwrap_or_else(|| panic!("No distance found for {}->{}", current_character, c)) {
                        let mut next_current_characters = r.current_characters.clone();
                        next_current_characters[i] = *c;
                        if !routes.iter().any(|other|
                            next_current_characters == other.current_characters && other.keys_to_get.is_subset(&next_keys_to_get) && other.distance_travelled <= next_distance + r.distance_travelled
                        ) {
                            routes.push(Route::new(next_distance + r.distance_travelled, next_current_characters, next_keys_to_get.clone(), next_keys_acquired.clone()));
                        }
                    }
                }
            });
        }
    }
}

#[derive(Eq, Debug)]
struct Route {
    distance_travelled: usize,
    current_characters: Vec<char>,
    keys_to_get: HashSet<char>,
    keys_acquired: HashSet<char>
}

impl Route {
    fn new(distance_travelled: usize, current_characters: Vec<char>, keys_to_get: HashSet<char>, keys_acquired: HashSet<char>) -> Route {
        Route { distance_travelled, current_characters, keys_to_get, keys_acquired }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.distance_travelled == other.distance_travelled && self.current_characters == other.current_characters
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance_travelled.cmp(&self.distance_travelled) // Deliberately do this the other way round to get an ascending sort
        .then(self.keys_acquired.len().cmp(&other.keys_acquired.len()))
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
