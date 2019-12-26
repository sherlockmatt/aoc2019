use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    {
        let mut state: u32 = 0;
        input.trim().lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    state += 2_u32.pow((y * 5 + x) as u32);
                }
            })
        });
        let mut previously_seen_states = HashSet::new();

        while !previously_seen_states.contains(&state) {
            previously_seen_states.insert(state);
            let mut next_state = 0;
            for i in 0..25 {
                let mut bug_count: u32 = 0;
                let x = i % 5;
                let y = i / 5;
                for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if 0 <= x + dx && x + dx < 5 && 0 <= y + dy && y + dy < 5 {
                        bug_count += (state >> ((y + dy) * 5 + x + dx) as u32) % 2_u32;
                    }
                }
                if ((state >> i) % 2 == 1 && bug_count == 1) || ((state >> i) % 2 == 0 && (bug_count == 1 || bug_count == 2)) {
                    next_state += 2_u32.pow(i as u32);
                }
            }
            state = next_state;
        }

        answers.push(format!("{}", state));
    }

    {
        let mut bug_map = HashSet::new();
        input.trim().lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    bug_map.insert((x as i32, y as i32, 0));
                }
            })
        });

        for _ in 0..200 {
            let mut new_bug_map = HashSet::new();
            let mut checked = HashSet::new();
            let mut to_check = HashSet::new();
            for (x, y, d) in &bug_map {
                checked.insert((*x, *y, *d));
                let mut bug_count = 0;
                for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if *x + dx == -1 {
                        if bug_map.contains(&(1, 2, *d - 1)) {
                            bug_count += 1;
                        }
                        to_check.insert((1, 2, *d - 1));
                    } else if *x + dx == 2 && *y + dy == 2 {
                        if *dx == 0 {
                            for inner_x in 0..5 {
                                if bug_map.contains(&(inner_x, (*y / 2) * 4, *d + 1)) {
                                    bug_count += 1;
                                }
                                to_check.insert((inner_x, (*y / 2) * 4, *d + 1));
                            }
                        } else {
                            for inner_y in 0..5 {
                                if bug_map.contains(&((*x / 2) * 4, inner_y, *d + 1)) {
                                    bug_count += 1;
                                }
                                to_check.insert(((*x / 2) * 4, inner_y, *d + 1));
                            }
                        }
                    } else if *x + dx == 5 {
                        if bug_map.contains(&(3, 2, *d - 1)) {
                            bug_count += 1;
                        }
                        to_check.insert((3, 2, *d - 1));
                    } else if *y + dy == -1 {
                        if bug_map.contains(&(2, 1, *d - 1)) {
                            bug_count += 1;
                        }
                        to_check.insert((2, 1, *d - 1));
                    } else if *y + dy == 5 {
                        if bug_map.contains(&(2, 3, *d - 1)) {
                            bug_count += 1;
                        }
                        to_check.insert((2, 3, *d - 1));
                    } else {
                        if bug_map.contains(&(*x + dx, *y + dy, *d)) {
                            bug_count += 1;
                        }
                        to_check.insert((*x + dx, *y + dy, *d));
                    }
                }
                if bug_count == 1 {
                    new_bug_map.insert((*x, *y, *d));
                }
            }

            for (x, y, d) in to_check.difference(&checked) {
                let mut bug_count = 0;
                for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if *x + dx == -1 {
                        if bug_map.contains(&(1, 2, *d - 1)) {
                            bug_count += 1;
                        }
                    } else if *x + dx == 2 && *y + dy == 2 {
                        if *dx == 0 {
                            for inner_x in 0..5 {
                                if bug_map.contains(&(inner_x, (*y/2)*4, *d + 1)) {
                                    bug_count += 1;
                                }
                            }
                        } else {
                            for inner_y in 0..5 {
                                if bug_map.contains(&((*x / 2) * 4, inner_y, *d + 1)) {
                                    bug_count += 1;
                                }
                            }
                        }
                    } else if *x + dx == 5 {
                        if bug_map.contains(&(3, 2, *d - 1)) {
                            bug_count += 1;
                        }
                    } else if *y + dy == -1 {
                        if bug_map.contains(&(2, 1, *d - 1)) {
                            bug_count += 1;
                        }
                    } else if *y + dy == 5 {
                        if bug_map.contains(&(2, 3, *d - 1)) {
                            bug_count += 1;
                        }
                    } else {
                        if bug_map.contains(&(*x + dx, *y + dy, *d)) {
                            bug_count += 1;
                        }
                    }
                }
                if bug_count == 1 || bug_count == 2 {
                    new_bug_map.insert((*x, *y, *d));
                }
            }
            bug_map = new_bug_map;
        }

        answers.push(format!("{}", bug_map.len()));
    }

    answers
}
