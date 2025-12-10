use std::{fs, usize};
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let machines = parse_input()?;
    let mut presses: usize = 0;

    let mut c = 0;
    for machine in machines {
        let current_indicator_lights = 0_u16;
        let current_presses = find_minimum_amount_of_presses(&machine, current_indicator_lights, 0);
        if current_presses == usize::MAX {
            panic!("Not solvable");
        }

        c += 1;
        println!("{}", c);

        presses += current_presses;
    }

    return Ok(format!("{}", presses));
}

fn find_minimum_amount_of_presses(machine: &Machine, current_indicator_lights: u16, iteration: u8) -> usize {
    if iteration >= 7 {
        return usize::MAX;
    }

    let current_distance =
        hamming_distance(machine.desired_indicator_lights, current_indicator_lights);

    // let mut best_distance = current_distance;
    let mut minimum_amount_of_presses = usize::MAX;
    for i in 0..machine.button_wirings.len() {
        let button = machine.button_wirings[i];
        let evolution_indicator_lights = current_indicator_lights ^ button;
        let evolution_distance = hamming_distance(evolution_indicator_lights, machine.desired_indicator_lights);
        // if evolution_distance < current_distance {
            // best_distance = best_distance.min(evolution_distance);
            if evolution_distance == 0 {
                return 1;
                // minimum_amount_of_presses = 1;
            } else {
                let presses = find_minimum_amount_of_presses(machine, evolution_indicator_lights, iteration + 1);
                minimum_amount_of_presses = minimum_amount_of_presses.min(presses);
            }
        //}
    }

    if minimum_amount_of_presses < usize::MAX {
        return minimum_amount_of_presses + 1;
    } else {
        return usize::MAX;
    }
}

fn hamming_distance(n1: u16, n2: u16) -> usize {
    // (n1 ^ n2).count_ones() as usize
    let mut not_matching = 0_usize;
    for i in 0..16 {
        let mask = 1 << i;
        if (n1 & mask) != (n2 & mask) {
            not_matching += 1;
        }
    }

    not_matching
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
