use std::fs;

fn main() {
  match run() {
    Ok(result) => println!("{}", result),
    Err(err) => panic!("{}", err)
  }
}

fn run() -> Result<String, String> {
    let ticks = 100;
    let mut current = 50;
    let mut at_zero = 0;
    let instructions = parse_input()?;
    for instruction in instructions.iter() {
      let mut iterations: i32 = *instruction;
      let tick = iterations / iterations.abs();
      while iterations != 0 {
        current = (current + tick) % ticks;
        at_zero += match current {
          0 => 1,
          _ => 0
        };

        iterations -= tick;
      }

      /*at_zero += match current {
        0 => 1,
        _ => 0
      };*/
    }

    return Ok(format!("{}", at_zero));
}


fn parse_input() -> Result<Vec<i32>, String> {
    match fs::read_to_string(format!("./src/bin/{}.txt", env!("CARGO_BIN_NAME").split("-").nth(0).unwrap_or(""))) {
        Ok(message) => Ok(message.lines()
        .map(|line| {
            let direction = match line.starts_with('R') {
              true => 1,
              false => -1
            };
            let amount = line[1..].parse::<i32>().unwrap() * direction;
            return amount;
        })
        .collect()),
        Err(err) => return Err(format!("Could not read input {}", err))
    }
}