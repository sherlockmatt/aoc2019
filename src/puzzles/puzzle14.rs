use std::collections::HashMap;
use itertools::Itertools;
use failure::_core::cmp::Ordering;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    // Store a map of each chemical to the reaction that produces it, in the form (quant_produced, regeants)
    let mut reactions: HashMap<String, (i64, Vec<(i64, String)>)> = HashMap::new();
    for line in input.trim().lines() {
        let (reagents, product) = line.split(" => ").next_tuple().unwrap();
        let (p_quant, p_name) = product.split(' ').next_tuple().unwrap();

        reactions.insert(
            String::from(p_name),
            (
                p_quant.parse::<i64>().expect("Not a number found in input"),
                reagents.split(", ").map(|s| {
                    let (r_quant, r_name) = s.split(' ').next_tuple().unwrap();
                    (
                        r_quant.parse::<i64>().expect("Not a number found in input"),
                        String::from(r_name)
                    )
                }).collect()
            )
        );
    }

    let ore_quant = create_fuel(1, &reactions);
    answers.push(format!("{}", ore_quant));

    // Estimate how much fuel we can make by assuming each uses the same amount of ore (it won't because of wastages)
    let target_ore = 1_000_000_000_000;
    let rough_guess = target_ore / ore_quant;
    let mut lower_bound = (rough_guess * 10) / 12; // Lower bound starts at 83.3% of guess
    let mut upper_bound = (rough_guess * 12) / 10; // Upper bound starts at 120% of guess

    // If the bounds are 1 apart, then one must be below the target and the other must be above
    while upper_bound - lower_bound > 1 {
        let middle_fuel = (lower_bound + upper_bound) / 2;
        let middle_ore = create_fuel(middle_fuel, &reactions);

        // Check which bound is closer to the target, move it to the current guess
        match middle_ore.cmp(&target_ore) {
            Ordering::Less => lower_bound = middle_fuel,
            Ordering::Greater => upper_bound = middle_fuel,
            Ordering::Equal => {
                lower_bound = middle_fuel;
                upper_bound = middle_fuel;
            }
        }
    }

    // Either we hit the target exactly, so both bounds are equal, or they straddle it
    // If the bounds are equal it doesn't matter which we pick, but
    // if they aren't we want the lower one because we can't use more ore than we have
    answers.push(format!("{}", lower_bound));

    answers
}

fn create_fuel(quantity: i64, reactions: &HashMap<String, (i64, Vec<(i64, String)>)>) -> i64 {
    let mut ore_quant = 0;
    let mut waste: HashMap<String, i64> = HashMap::new();
    let mut requests: Vec<(i64, String)> = vec![(quantity, String::from("FUEL"))];
    while let Some((required_quant, required_product)) = requests.pop() {
        if required_product == "ORE" {
            ore_quant += required_quant;
        } else {
            let (produced_quant, reagents) = reactions.get(&required_product).unwrap();
            let available_waste = waste.entry(required_product).or_insert(0);
            let new_waste = (*produced_quant - (required_quant - *available_waste) % *produced_quant) % *produced_quant;
            let reactions_needed = (required_quant - *available_waste + new_waste) / *produced_quant;
            *available_waste = new_waste;
            for (reagent_quant, reagent_name) in reagents {
                match requests.iter().position(|(_, n)| n == reagent_name) {
                    Some(i) => requests[i] = (requests[i].0 + reactions_needed * *reagent_quant, reagent_name.clone()),
                    None => requests.insert(0, (reactions_needed * *reagent_quant, reagent_name.clone()))
                }
            }
        }
    }
    ore_quant
}
