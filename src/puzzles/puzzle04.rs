fn ascending_range(start: &u32, end: &u32) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = vec![];

    let mut current: Vec<u8> = format!("{}", start).chars().map(
        |n| n.to_digit(10).expect("Not a number") as u8
    ).collect();
    // State's first element is a copy flag: once we see a decrease in the number, just start copying the same thing
    // State's second element is the current number to compare/copy
    // This will do something like [3, 6, 7, 4, 5, 9] -> [3, 6, 7, 7, 7, 7]
    // Copy mode is entered when the 4 is seen (first decrease), then we copy the 7 across
    // This skips to the first increasing number above start
    current = current.iter().scan((false, 0u8), |state, x|
        match *state {
            // We're in copy mode, so just return whatever is in state's second element
            (true, n) => Some(n),
            // We aren't in copy mode yet, and the new number is higher than the current, so update current
            (false, n) if n <= *x => {
                *state = (false, *x);
                Some(*x)
            },
            // We aren't in copy mode yet, and the new number is lower than the current, so update current and enable copy mode
            (false, n) => {
                *state = (true, n);
                Some(n)
            }
        }
    // Add the characters in reverse, i.e. 1234 -> [4, 3, 2, 1]
    // This allows for much easier incrementing and folding back to a number
    ).collect::<Vec<u8>>().iter().rev().map(|&i| i).collect();

    while (0u32..).zip(current.iter()).fold(0, |a, (e, x)| a + 10u32.pow(e as u32) * (*x as u32)) <= *end {
        ret.push(current.iter().rev().map(|&i| i).collect());
        let inc_pos = current.iter().position(|n| *n < 9);
        match inc_pos {
            None => {
                // All the digits are 9, roll them all over to 1 and append a 1
                current = current.iter().map(|_| 1u8).collect();
                current.push(1u8);
            },
            Some(p) => {
                // p is the position of the first non-9, so the value we want to roll to is 1 more than what's at that pos
                // This goes from [9, 9, 4, 1] to [5, 5, 5, 1], skipping a huge chunk of the input space
                let rollover_val = current[p] + 1;
                for i in 0..=p {
                    current[i] = rollover_val;
                }
            }
        }
    }

    ret
}

fn has_double(n: &Vec<u8>) -> bool {
    n.windows(2).any(|i| i[0] == i[1])
}

// The puzzle says all input is length 6, but for completeness this works for all positive integers
fn has_exact_double(n: &Vec<u8>) -> bool {
    match n.len() {
        0..=1 => false,
        2 => n[0] == n[1],
        _ => {
            let win = n.windows(3);
            let first = win.clone().next().unwrap();
            let last = win.clone().last().unwrap();

            // Check the first 3 and last 3 explicitly, then check each 4 in the middle
            (first[0] == first[1] && first[1] != first[2]) ||
                (last[0] != last[1] && last[1] == last[2]) ||
                n.windows(4).any(|i|
                    i[0] != i[1] && i[1] == i[2] && i[2] != i[3]
                )
        }
    }
}

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let range_to_search: Vec<u32> = input.trim().split('-').map(|s| s.parse::<u32>().expect("Not a number")).collect();
    let start = range_to_search.first().expect("Not enough values");
    let end = range_to_search.last().expect("Not enough values");

    // Build a vec like [1, 2, 3, 4, 5, 6] for each number to check, then filter them with the criteria
    let first_pass: Vec<Vec<u8>> = ascending_range(start, end).iter().filter(
        |i| has_double(i)
    ).map(|i| i.clone()).collect();

    answers.push(format!("{}", first_pass.len()));

    // Since we're just adding a new criterion, we can reuse the output from part 1
    answers.push(format!("{}", first_pass.iter().filter(
        |i| has_exact_double(i)
    ).collect::<Vec<&Vec<u8>>>().len()));

    answers
}
