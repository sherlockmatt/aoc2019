use failure::{Error, ResultExt};

pub fn download(puzzle_number : usize, session : &String) -> Result<String, Error> {
    let url_to_get = format!("https://adventofcode.com/2019/day/{}/input", puzzle_number);
    let client = reqwest::Client::new();
    return Ok(client.get(&url_to_get)
        .header("cookie", format!("session={}", session))
        .send()?
        .text()
        .with_context(|_| format!("Could not download from URL {}", url_to_get))?);
}
