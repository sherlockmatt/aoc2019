use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<i64> = input.trim().chars().map(|c| c.to_digit(10).expect("Not a number found in input") as i64).collect();

    answers.push(format!("{}", fft(&inputs).iter().take(8).join("")));

    let offset = inputs.iter().take(7).join("").parse::<usize>().unwrap();
    if offset < inputs.len() * 10_000 / 2 {
        answers.push(String::from("The offset in the input is smaller than the assumed limit for this solution, no answer"));
    } else {
        // If the offset is over halfway, the problem reduces itself to summing a triangle, so we can shortcut a lot of the work
        // All inputs I've seen have the offset very close to the end, hence I assume this trick was intended
        // The trick is to ignore the first lot, up to the offset, then reverse it
        // Then you can run a cumulative sum % 10 over that list, for each phase
        // Then just reverse it again and take the first 8 digits for the answer
        let mut input_list: Vec<i64> = inputs.iter().rev().cycle().take(inputs.len()*10_000 - offset).map(|i| *i).collect();
        for _ in 0..100 {
            input_list = input_list.iter().scan(0, |state, i| { *state = (*state + *i) % 10; Some(*state)}).collect();
        }

        answers.push(format!("{}", input_list.iter().rev().take(8).join("")));
    }

    answers
}

fn fft(sequence: &Vec<i64>) -> Vec<i64> {
    let mut input_list = sequence.clone();
    for _ in 0..100 {
        let mut output_list = Vec::new();
        for i in 0..input_list.len() {
            let mut pattern: Vec<i64> = Vec::new();
            for val in &[0, 1, 0, -1] {
                for _ in 0..=i {
                    pattern.push(*val);
                }
            }
            output_list.push(
                input_list.iter().zip(
                    pattern.iter().cycle().skip(1)
                ).map(|(v, p)|
                    *v * *p
                ).sum::<i64>().abs() % 10
            );
        }
        input_list = output_list;
    }
    input_list
}
