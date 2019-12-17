use crate::intcode::IntcodeMachine;
use std::collections::HashMap;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    let directions = vec![(1i64, (0, 1)), (2i64, (0, -1)), (3i64, (-1, 0)), (4i64, (1, 0))];
    let mut room_map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut robot_pos = (0, 0);
    room_map.insert((0, 0), 1);
    for (_, (dx, dy)) in &directions { room_map.insert((*dx, *dy), -1); }
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut state = program.clone();
    let mut pos = 0;
    let mut base = 0;
    let mut is_halted = false;
    let mut steps = Vec::new();
    let mut oxygen_pos = (0, 0);
    let mut oxygen_distance = 0;

    while !is_halted && room_map.iter().any(|(_, i)| *i == -1) { // Continue until there are no unexplored sections
        let did_backtrack;
        let unknown_directions: Vec<&(i64, (i64, i64))> = directions.iter().filter(|(_, (dx, dy))|
            *room_map.entry((robot_pos.0 + *dx, robot_pos.1 + *dy)).or_insert(-1) == -1
        ).collect();

        let (command, dir_to_move) = if unknown_directions.len() == 0 {
            // There aren't any unexplored directions, so step backwards one
            let cmd = opposite_dir(steps.pop().unwrap());
            did_backtrack = true;
            directions.iter().find(|(c,_)| *c == cmd).unwrap()
        } else {
            did_backtrack = false;
            unknown_directions[0]
        };
        inputs.push(*command);
        {
            let mut machine = IntcodeMachine::new(&mut state, &mut inputs, &mut outputs, pos, base);
            machine.execute_until_next_op_is_input();
            pos = machine.get_pos();
            base = machine.get_base();
            is_halted = machine.is_halted();
        }
        let output = outputs.pop().unwrap();
        let pos_of_output = (robot_pos.0 + dir_to_move.0, robot_pos.1 + dir_to_move.1);
        room_map.insert(pos_of_output, output); // Make sure we record this before moving, as we don't always move
        if output != 0 {
            robot_pos = pos_of_output;
            if !did_backtrack { // Don't record backtracking steps
                steps.push(*command);
            }

            if output == 2 {
                // Found the oxygen, record the stats
                oxygen_pos = robot_pos;
                oxygen_distance = steps.len();
            } // ... but carry on exploring, we need to explore the whole area for part 2
        }

        // Set neighbouring positions to unknown if they aren't already known
        directions.iter().for_each(|(_, (dx, dy))| { room_map.entry((robot_pos.0 + *dx, robot_pos.1 + *dy)).or_insert(-1); });
    }

//    print_room(&room_map, &robot_pos);
    answers.push(format!("{}", oxygen_distance));

    // Do depth first search from the oxygen position to find the longest path
    let mut longest_path = 0;
    let mut explore_path = Vec::new();
    let mut visited = Vec::new();
    explore_path.push(oxygen_pos);

    while explore_path.len() > 0 {
        let current_pos = explore_path[explore_path.len() - 1];
        visited.push(current_pos);
        let mut avail_pos = Vec::new();
        for (_, (dx, dy)) in &directions {
            let compare_pos = (current_pos.0 + *dx, current_pos.1 + *dy);
            if !visited.contains(&compare_pos) && *room_map.get(&compare_pos).unwrap() == 1 {
                avail_pos.push(compare_pos)
            }
        };
        if avail_pos.len() == 0 {
            explore_path.pop(); // Backtrack
        } else {
            // Only record one move, so that we always have a single path from the oxygen to our current point
            explore_path.push(avail_pos[0]);
            longest_path = longest_path.max(explore_path.len());
        }
    }

    // The oxygen is already filled, but is present on our path, so subtract one
    answers.push(format!("{}", longest_path - 1));

    answers
}

fn opposite_dir(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        d => panic!("Unknown direction `{}`", d)
    }
}

fn _print_room(room_map: &HashMap<(i64, i64), i64>, robot_pos: &(i64, i64)) {
    for y in *room_map.iter().map(|((_, y), _)|y).min().unwrap()..=*room_map.iter().map(|((_, y),_)|y).max().unwrap() {
        for x in *room_map.iter().map(|((x,_),_)|x).min().unwrap()..=*room_map.iter().map(|((x,_),_)|x).max().unwrap() {
            if x == 0 && y == 0 {
                print!("S");
            } else if x == robot_pos.0 && y == robot_pos.1 {
                print!("R");
            } else {
                print!("{}", match room_map.get(&(x, y)).unwrap_or(&-2) {
                    0 => '#',
                    1 => '.',
                    2 => 'O',
                    -1 => '?',
                    _ => ' '
                });
            }
        }
        println!();
    }
}
