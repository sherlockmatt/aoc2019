use failure::_core::str::FromStr;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    // Split the input by lines, then map them all to numbers
    let inputs: Vec<u32> = input.split_whitespace().map(|s| u32::from_str(s).unwrap()).collect();

    // Part 1
    answers.push(format!("{}", inputs.clone().into_iter().map(calc_fuel).sum::<u32>()));

    // Part 2
    answers.push(format!("{}", inputs.clone().into_iter().map(recurse_fuel).sum::<u32>()));

    return answers;
}

fn calc_fuel(input: u32) -> u32 {
    // This works because u32 division auto-floors itself
    return if input < 6 { 0 } else { (input / 3) - 2 };
}

fn recurse_fuel(input: u32) -> u32 {
    let new_fuel: u32 = calc_fuel(input);
    if new_fuel == 0 {
        return 0;
    } else {
        return new_fuel + recurse_fuel(new_fuel);
    }
}
