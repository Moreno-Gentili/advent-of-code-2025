use std::fs;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let banks = parse_input()?;

    let total_jolts: u64 = banks.iter().map(|bank| {
        let mut jolts: u64 = 0;
        let mut batteries_to_activate = 12;
        let mut current_bank = bank.clone();
        while batteries_to_activate > 0 {
            let mut best_jolts: u8 = 0;
            let mut best_jolts_position: usize = 0;
            for i in 0..current_bank.len()-batteries_to_activate+1 {
                if current_bank[i] > best_jolts {
                    best_jolts = current_bank[i];
                    best_jolts_position = i;
                }
            }

            current_bank = current_bank.get(best_jolts_position+1..).unwrap().to_vec();

            let current_jolts = best_jolts as u64 * 10_u64.pow((batteries_to_activate - 1) as u32);
            jolts += current_jolts;
            batteries_to_activate -= 1;
        }

        jolts
    }).sum();

    return Ok(format!("{}", total_jolts));
}

fn parse_input() -> Result<Vec<Vec<u8>>, String> {
    let input = read_file()?;
    let banks = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    return Ok(banks);
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
