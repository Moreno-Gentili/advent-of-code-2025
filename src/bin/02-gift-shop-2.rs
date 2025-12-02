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
        for size in get_length(range.min)..=get_length(range.max) {
            for n in 1..=99999_u64 {
                let length = get_length(n);
                if size % length != 0 || size == length {
                    continue;
                }

                let repeated_n = repeat(n, (size / length) as usize);
                if repeated_n >= range.min && repeated_n <= range.max {
                  results.insert(repeated_n);
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
  let mut result = n;
  let size = n.ilog10() as usize + 1;
  let multiplier = 10_u64.pow(size as u32);
  for _ in 1..iterations {
    result *= multiplier;
    result += n
  }

  return result;
}

fn parse_input() -> Result<Vec<Range>, String> {
    match fs::read_to_string(format!(
        "./src/bin/{}.txt",
        env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or("")
    )) {
        Ok(message) => Ok(message
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
            .collect()),
        Err(err) => return Err(format!("Could not read input {}", err)),
    }
}

struct Range {
    min: u64,
    max: u64,
}