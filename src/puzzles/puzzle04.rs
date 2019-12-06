fn is_not_decreasing(n: &Vec<u32>) -> bool {
    n.iter().fold(0, |a, c|
        match a {
            10 => 10,
            _ => if *c >= a { *c } else { 10 }
        },
    ) < 10
}

fn has_double(n: &Vec<u32>) -> bool {
    n.windows(2).any(|i| i[0] == i[1])
}

// The puzzle says all input is length 6, but for completeness this works for all positive integers
fn has_exact_double(n: &Vec<u32>) -> bool {
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
    let first_pass: Vec<Vec<u32>> = (*start..=*end).map(
        |s| format!("{}", s).chars().map(
            |n| n.to_digit(10).expect("Not a number")
        ).collect()
    ).filter(
        |i| is_not_decreasing(i) && has_double(i)
    ).collect();

    answers.push(format!("{}", first_pass.len()));

    // Since we're just adding a new criterion, we can reuse the output from part 1
    answers.push(format!("{}", first_pass.iter().filter(
        |i| has_exact_double(i)
    ).collect::<Vec<&Vec<u32>>>().len()));

    answers
}
