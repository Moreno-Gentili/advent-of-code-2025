use std::fs;

fn main() {
  match run() {
    Ok(result) => println!("{}", result),
    Err(err) => panic!("{}", err)
  }
}

fn run() -> Result<String, String> {
    let mut sum: u64 = 0;
    let ranges = parse_input()?;
    for range in ranges {
      let number_length = get_length(range.min);
      let take_length = (number_length / 2).max(1);
      let mut segment = range.min / 10_u64.pow(number_length - take_length);
      while is_below(&range, segment)? {
        if is_in_range(&range, segment)? {
          sum += double_segment(segment);
        }

        segment += 1;
      }
    }

    return Ok(format!("{}", sum));
}

fn is_in_range(range: &Range, segment: u64) -> Result<bool, String> {
  let numeric_value = double_segment(segment);
  return match numeric_value {
    n if n >= range.min && n <= range.max => Ok(true),
    _ => Ok(false)
  }
}

fn is_below(range: &Range, segment: u64) -> Result<bool, String> {
  let numeric_value = double_segment(segment);
  return match numeric_value {
    n if n <= range.max => Ok(true),
    _ => Ok(false)
  }
}

fn get_length(n: u64) -> u32 {
  if n == 0 {
    return 0;
  }
  return (n.ilog10() as usize + 1) as u32;
}

fn double_segment(segment: u64) -> u64 {
  return (segment * 10_u64.pow(get_length(segment))) + segment;
}

fn parse_input() -> Result<Vec<Range>, String> {
    match fs::read_to_string(format!("./src/bin/{}.txt", env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or(""))) {
        Ok(message) => Ok(message.split(",")
        .map(|tuple| {
            let parts: Vec<u64> = tuple.split("-").map(|n| n.parse::<u64>().unwrap()).collect();
            return Range { min: parts[0], max: parts[1] };
        })
        .collect()),
        Err(err) => return Err(format!("Could not read input {}", err))
    }
}

struct Range {
  min: u64,
  max: u64
}