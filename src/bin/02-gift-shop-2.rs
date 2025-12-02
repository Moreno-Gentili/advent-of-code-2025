use std::collections::HashSet;
use std::fs;

fn main() {
    match run() {
        Ok(result) => println!("{}", result),
        Err(err) => panic!("{}", err),
    }
}

fn run() -> Result<String, String> {
    let mut results = HashSet::new();
    let ranges = parse_input()?;
    for range in ranges {
        for size in range.min.to_string().len()..=range.max.to_string().len() {
            for n in 1..=99999 {
                let str_n = n.to_string();
                if size % str_n.len() != 0 || size == str_n.len() {
                    continue;
                }

                let repeated_n = str_n.repeat(size / str_n.len());
                let parsed_n = repeated_n.parse::<i64>().unwrap();
                if parsed_n >= range.min && parsed_n <= range.max {
                  results.insert(parsed_n);
                }
            }
        }
    }

    return Ok(format!("{}", results.iter().sum::<i64>()));
}

fn parse_input() -> Result<Vec<Range>, String> {
    match fs::read_to_string(format!(
        "./src/bin/{}.txt",
        env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or("")
    )) {
        Ok(message) => Ok(message
            .split(",")
            .map(|tuple| {
                let parts: Vec<i64> = tuple
                    .split("-")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                return Range {
                    min: parts[0],
                    max: parts[1],
                };
            })
            .collect()),
        Err(err) => return Err(format!("Could not read input {}", err)),
    }
}

struct Range {
    min: i64,
    max: i64,
}
