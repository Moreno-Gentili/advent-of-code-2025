use std::fs;
use std::ops::{Range, Sub};
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let input = parse_input()?;
    let fresh = input.ids
        .iter()
        .filter(|id| input.ranges.iter()
            .any(|r| r.contains(id)))
        .count();
    
    return Ok(format!("{}", fresh));
}

fn parse_input() -> Result<Input, String> {
    let input = read_file()?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<Range<i64>> = parts[0].lines().map(|l| {
        let range: Vec<i64> = l.split("-").map(|n| n.parse::<i64>().unwrap()).collect();
        range[0]..range[1]+1
    }).collect();

    let ids = parts[1].lines().map(|n| n.parse::<i64>().unwrap()).collect();
    return Ok(Input { ranges, ids });
}

struct Input {
    ranges: Vec<Range<i64>>,
    ids: Vec<i64>
}

///////////

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    match run() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => panic!("{}", err),
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("It took: {}ms", end.sub(start).as_millis());
}

fn read_file() -> Result<String, String> {
    return match fs::read_to_string(format!(
        "./src/bin/{}.txt",
        env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or("")
    )) {
        Ok(message) => Ok(message),
        Err(err) => return Err(format!("Could not read input {}", err)),
    };
}