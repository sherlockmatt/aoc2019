use failure::{Error, ResultExt};
use failure::_core::str::FromStr;

pub fn download(puzzle_number: usize, session: &String) -> Result<String, Error> {
    let url_to_get = format!("https://adventofcode.com/2019/day/{}/input", puzzle_number);
    let client = reqwest::Client::new();
    return Ok(client.get(&url_to_get)
        .header("cookie", format!("session={}", session))
        .send()?
        .text()
        .with_context(|_| format!("Could not download from URL {}", url_to_get))?);
}

pub fn parse_range(input: String) -> Result<Vec<usize>, Error> {
    let mut ret: Vec<usize> = Vec::new();
    for s in input.split(',') {
        let split: Vec<String> = s.split('-').map(String::from).collect();
        if split.len() == 1 {
            ret.push(usize::from_str(&split[0]).with_context(|_| format!("Could not parse int from range value `{}`", split[0]))?);
        } else if split.len() == 2 {
            let start: usize = usize::from_str(&split[0]).with_context(|_| format!("Could not parse int from range value `{}`", split[0]))?;
            let end: usize = usize::from_str(&split[1]).with_context(|_| format!("Could not parse int from range value `{}`", split[1]))?;
            ret.extend(start..=end);
        } else {
            return Err(failure::err_msg(format!("Invalid range spec `{}`", s)));
        }
    }
    Ok(ret)
}
