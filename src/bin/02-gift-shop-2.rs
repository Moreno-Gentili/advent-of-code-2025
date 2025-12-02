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
            for n in 1..=99999_i64 {
                let length = n.ilog10() as usize + 1;
                if size % length != 0 || size == length {
                    continue;
                }

                let repeated_n = repeat(n, size / length);
                if repeated_n >= range.min && repeated_n <= range.max {
                  results.insert(repeated_n);
                }
            }
        }
    }

    return Ok(format!("{}", results.iter().sum::<i64>()));
}

fn repeat(n: i64, iterations: usize) -> i64 {
  let mut result = n;
  let size = n.ilog10() as usize + 1;
  let multiplier = 10_i64.pow(size as u32);
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