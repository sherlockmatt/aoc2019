use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let image_width: usize = 25;
    let image_height: usize = 6;

    let input_as_chars = input.trim().chars().collect::<Vec<char>>();

    // For each chunk of 25*6 (i.e. a layer), build a frequency hash map
    // Then take the one with the least 0s, multiply it's 1 and 2 counts together
    answers.push(format!("{}", input_as_chars.chunks_exact(image_width * image_height)
        .map(|layer| {
            let mut hs = HashMap::new();
            layer.iter().for_each(|c| *hs.entry(c).or_insert(0) += 1);
            hs
        })
        .min_by_key(|layer| *layer.get(&'0').unwrap())
        .unwrap().iter()
        .fold(1, |a, (&&c, f)| if c == '1' || c == '2' { a * *f } else { a })
    ));

    // For each pixel position get an iterator of each layer's pixel at that position (using .skip(i).step_by(25*6))
    // Then fold that iterator such that we propagate the first non-2 we found, replaced by a black or white pixel
    answers.push(format!("\n{}", (0..(image_width * image_height)).map(
        |i|
            input_as_chars.iter().skip(i).step_by(image_width * image_height).fold(
                '2',
                |a, &c| if a == '2' {
                    match c {
                        '0' => '░', // "Black" pixel
                        '1' => '█', // White pixel
                        '2' => '2', // Transparent pixel
                        other => panic!("Unknown pixel value `{}`", other)
                    }
                } else { a } // If we already have a non-transparent pixel, propagate it
            )
        ).collect::<Vec<char>>().chunks_exact(image_width).map(
            |line|
                line.iter().join("")
        ).join("\n")
    ));

    answers
}
