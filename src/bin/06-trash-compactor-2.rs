use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;

fn run() -> Result<String, String> {
    let operations = parse_input()?;
    let total: u64 = operations.iter().map(|op| match op {
        Operation::Addition(numbers) => numbers.iter().sum::<u64>(),
        Operation::Multiplication(numbers) => numbers.iter().fold(1, |tot, n| tot * n),
    }).sum();
    
    return Ok(format!("{}", total));
}

fn parse_input() -> Result<Vec<Operation>, String> {
    let input = read_file()?;
    let data: Vec<Vec<String>> = input.lines().map(|l| l.chars().map(|c| c.to_string()).collect()).collect();
    let mut operations: Vec<Operation> = vec![];
    let mut numbers: Vec<u64> = vec![];
    for x in (0..data[0].len()).rev() {
        let mut number = String::from("");
        let mut y = 0;
        while y < data.len()-1 {
            number += match data[y][x].as_str() {
                " " => "",
                digit => digit
            };

            y += 1;
        }
        
        if number != "" {
            numbers.push(number.parse::<u64>().unwrap());
        }

        match data[y][x].as_str() {
            " " => {  },
            other => {
                let operation = match other {
                    "+" => Operation::Addition(numbers),
                    "*" => Operation::Multiplication(numbers),
                    _ => panic!("Operation not supported")
                };

                operations.push(operation);
                numbers = vec![];
            }
        }
    }

    return Ok(operations);
}

enum Operation {
    Addition(Vec<u64>),
    Multiplication(Vec<u64>)
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