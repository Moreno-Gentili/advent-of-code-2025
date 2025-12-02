use std::collections::HashSet;
use std::fs;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let mut results = HashSet::new();
    let ranges = parse_input()?;
    for range in ranges {
        let min_length = get_length(range.min);
        let max_length = get_length(range.max);
        for current_length in min_length..=max_length {
            for unit_size in 1..=(max_length / 2) {
                if current_length % unit_size == 0 {
                    let repetitions = (current_length / unit_size) as usize;
                    if repetitions > 1 {
                        for n in 10_u64.pow(unit_size - 1)..=repeat(9, unit_size as usize) {
                            let repeated_n = repeat(n, repetitions);
                            if repeated_n >= range.min && repeated_n <= range.max {
                                results.insert(repeated_n);
                            }
                        }
                    }
                }
            }
        }
    }

    return Ok(format!("{}", results.iter().sum::<u64>()));
}

fn get_length(n: u64) -> u32 {
    return (n.ilog10() as usize + 1) as u32;
}

fn repeat(n: u64, iterations: usize) -> u64 {
    let mut result = 0;
    let multiplier = 10_u64.pow(get_length(n));
    for _ in 0..iterations {
        result = result * multiplier + n;
    }

    return result;
}

fn parse_input() -> Result<Vec<Range>, String> {
    let input = read_file()?;
    return Ok(input
        .split(",")
        .map(|tuple| {
            let parts: Vec<u64> = tuple
                .split("-")
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            return Range {
                min: parts[0],
                max: parts[1],
            };
        })
        .collect());
}

struct Range {
    min: u64,
    max: u64,
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
