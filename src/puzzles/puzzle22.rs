pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut offset: i128 = 0;
    let mut increment: i128 = 1;

    shuffle_once(&input, &mut offset, &mut increment, 10_007);

    answers.push(format!("{}", ((2019 - offset) * mod_pow(increment, 10_005, 10_007)) % 10_007));
    
    offset = 0;
    increment = 1;
    let cards= 119_315_717_514_047;
    let repeats = 101_741_582_076_661;

    shuffle_once(&input, &mut offset, &mut increment, cards);

    let final_increment = mod_pow(increment, repeats, cards);
    let final_offset = ((offset * (1 - final_increment)) % cards * mod_pow((1 - increment) % cards, cards - 2, cards)) % cards;

    answers.push(format!("{}", (final_offset + final_increment * 2020) % cards));

    answers
}

fn shuffle_once(instructions: &String, offset: &mut i128, increment: &mut i128, deck_size: i128) {
    instructions.trim().lines().for_each(|l|
        if l.starts_with("deal with increment") {
            let n = l.split(' ').last().unwrap().parse::<i128>().unwrap();
            *increment *= mod_pow(n, deck_size - 2, deck_size);
            *increment %= deck_size;
        } else if l.starts_with("deal into new stack") {
            *increment *= -1;
            *increment %= deck_size;
            *offset += *increment;
            *offset %= deck_size;
        } else if l.starts_with("cut ") {
            let n = l.split(' ').last().unwrap().parse::<i128>().unwrap();
            *offset += *increment * n;
            *offset %= deck_size;
        } else {
            panic!("Unknown instruction `{}`", l);
        }
    );
}

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
