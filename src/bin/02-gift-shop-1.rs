use std::fs;

fn main() {
  match run() {
    Ok(result) => println!("{}", result),
    Err(err) => panic!("{}", err)
  }
}

fn run() -> Result<String, String> {
    let mut sum: i64 = 0;
    let ranges = parse_input()?;
    for range in ranges {
      let min = range.min.to_string();
      let max_length = (min.len()/2).max(1); // TODO: non usare parse
      let mut segment = *(&min[0..max_length].parse::<i64>().unwrap());
      while is_below(&range, segment)? {
        if is_in_range(&range, segment)? {
          sum += double_segment(segment);
        }

        segment += 1;
      }
    }

    return Ok(format!("{}", sum));
}

fn is_in_range(range: &Range, segment: i64) -> Result<bool, String> {
  let numeric_value = double_segment(segment);
  return match numeric_value {
    n if n >= range.min && n <= range.max => Ok(true),
    _ => Ok(false)
  }
}

fn is_below(range: &Range, segment: i64) -> Result<bool, String> {
  let numeric_value = double_segment(segment);
  return match numeric_value {
    n if n <= range.max => Ok(true),
    _ => Ok(false)
  }
}

fn double_segment(segment: i64) -> i64 {
  return (segment * 10_i64.pow(segment.to_string().len() as u32)) + segment;
}

fn parse_input() -> Result<Vec<Range>, String> {
    match fs::read_to_string(format!("./src/bin/{}.txt", env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or(""))) {
        Ok(message) => Ok(message.split(",")
        .map(|tuple| {
            let parts: Vec<i64> = tuple.split("-").map(|n| n.parse::<i64>().unwrap()).collect();
            return Range { min: parts[0], max: parts[1] };
        })
        .collect()),
        Err(err) => return Err(format!("Could not read input {}", err))
    }
}

struct Range {
  min: i64,
  max: i64
}

// 18595663903