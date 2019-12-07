use std::collections::HashMap;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    // Store a map of each node and its parent
    let mut node_map: HashMap<String, Option<String>> = HashMap::new();
    // The "Centre Of Mass" orbits nothing
    node_map.insert(String::from("COM"), None);

    input.lines().map(
        |l| l.split(')')
    ).for_each(
        |i| {
            let d: Vec<String> = i.map(|s| s.to_string()).collect();
            node_map.insert(d[1].clone(), Some(d[0].clone()));
            ()
        }
    );

    // Count the depth of each node, and add them all up
    answers.push(format!("{:?}", node_map.iter().map(
        |(_, p)| {
            let mut count = 0;
            let mut n = p;
            while match n { Some(_) => true, None => false } {
                count += 1;
                n = node_map.get(n.as_ref().unwrap()).unwrap();
            }
            count
        }
    ).sum::<i32>()));

    // Build the list of parents for both YOU and SAN
    let mut you_to_com: Vec<String> = Vec::new();
    let mut san_to_com: Vec<String> = Vec::new();

    let mut current = node_map.get(&String::from("YOU")).unwrap();
    while match current { Some(_) => true, None => false } {
        you_to_com.insert(0, current.as_ref().unwrap().clone());
        current = node_map.get(current.as_ref().unwrap()).unwrap();
    }

    let mut current = node_map.get(&String::from("SAN")).unwrap();
    while match current { Some(_) => true, None => false } {
        san_to_com.insert(0, current.as_ref().unwrap().clone());
        current = node_map.get(current.as_ref().unwrap()).unwrap();
    }

    // Find the position of the first node that is different, i.e. the branching point
    let divergence = you_to_com.iter().zip(san_to_com.iter()).position(|(a, b)| a != b).unwrap();

    // The distance from YOU to SAN is the number of steps from YOU to divergence plus the number of steps from divergence to SAN
    answers.push(format!("{}", (you_to_com.len() - divergence) + (san_to_com.len() - divergence)));

    answers
}
