use std::collections::VecDeque;
use std::{fs, usize};
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let machines = parse_input()?;
    let mut presses: usize = 0;

    for machine in machines {
        let mut queue: VecDeque<Iteration> = VecDeque::new();
        queue.push_back(Iteration { presses: 0, current_indicator_lights: 0 });
        while let Some(iteration) = queue.pop_front() {
            for i in 0..machine.button_wirings.len() {
                let button = machine.button_wirings[i];
                let evolution_indicator_lights = iteration.current_indicator_lights ^ button;
                if hamming_distance(evolution_indicator_lights, machine.desired_indicator_lights) == 0 {
                    presses += iteration.presses + 1;
                    queue.clear();
                    break;
                } else {
                    queue.push_back(Iteration { presses: iteration.presses + 1, current_indicator_lights: evolution_indicator_lights });
                }
            }
        }
    }

    return Ok(format!("{}", presses));
}

fn hamming_distance(n1: u16, n2: u16) -> usize {
    (n1 ^ n2).count_ones() as usize
}

fn parse_input() -> Result<Vec<Machine>, String> {
    let input = read_file()?;
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<&str>>();
            let indicators = parts[0].len() - 2;
            let indicator_lights = parts[0]
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .chars()
                .into_iter()
                .enumerate()
                .fold(0_u16, |s, (i, j)| match j {
                    '#' => s + (1 << indicators - 1 - i),
                    _ => s,
                });

            let joltage_requirements = parts[parts.len() - 1]
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(',')
                .map(|n| n.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            let button_wirings = parts[1..parts.len() - 1]
                .iter()
                .map(|w| {
                    w.strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(',')
                        .fold(0_u16, |s, n| {
                            s + (1 << indicators - 1 - n.parse::<usize>().unwrap())
                        })
                })
                .collect::<Vec<u16>>();

            Machine {
                desired_indicator_lights: indicator_lights,
                button_wirings,
                joltage_requirements,
            }
        })
        .collect();

    return Ok(machines);
}

struct Machine {
    desired_indicator_lights: u16,
    button_wirings: Vec<u16>,
    joltage_requirements: Vec<u16>,
}

struct Iteration {
    presses: usize,
    current_indicator_lights: u16
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
