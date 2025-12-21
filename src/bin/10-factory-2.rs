// Thanks to this tenthmascot's post
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// Runs in ~13s

use std::collections::{HashMap};
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, usize};

fn run() -> Result<String, String> {
    let machines = parse_input()?;
    let mut presses: usize = 0;

    for m in 0..machines.len() {
        let machine = &machines[m];
        let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();
        let least_presses = recurse(&machine.joltage_requirements, &machine.button_wirings, &mut cache);
        presses += least_presses;
    }

    return Ok(format!("{}", presses));
}

fn recurse(joltage_requirements: &Vec<usize>, button_wirings: &Vec<Vec<usize>>, cache: &mut HashMap<Vec<usize>, usize>) -> usize {
    if let Some(result) = cache.get(joltage_requirements) {
        return *result;
    }

    let odd_button_index_patterns =
        get_button_patterns_matching_light_indicator(joltage_requirements, button_wirings);
    let mut least_presses = usize::MAX;
    for i in 0..odd_button_index_patterns.len() {
        let pattern = &odd_button_index_patterns[i];
        let mut residual_joltage_requirements = joltage_requirements.clone();
        let subtract_result = subtract_buttons(&mut residual_joltage_requirements, &button_wirings, pattern);
        if !subtract_result {
            continue;
        }

        let pattern_presses = match residual_joltage_requirements.iter().sum::<usize>() {
            0 => 0,
            _ => {
                halve_requirements(&mut residual_joltage_requirements);
                recurse(&residual_joltage_requirements, button_wirings, cache)
            }
        };

        if pattern_presses != usize::MAX {
            let total_pattern_presses = (pattern_presses * 2) + pattern.len();
            least_presses = least_presses.min(total_pattern_presses);
        }
    }

    cache.insert(joltage_requirements.to_vec(), least_presses);
    least_presses
}

fn subtract_buttons(
    residual_joltage_requirements: &mut Vec<usize>,
    button_wirings: &Vec<Vec<usize>>,
    pattern: &Vec<usize>,
) -> bool {
    for i in pattern {
        let button = &button_wirings[*i];
        for j in button {
            if residual_joltage_requirements[*j] > 0 {
                residual_joltage_requirements[*j] -= 1
            } else {
                return false;
            }
        }
    }

    return true;
}

fn halve_requirements(residual_joltage_requirements: &mut Vec<usize>) {
    for i in 0..residual_joltage_requirements.len() {
        if residual_joltage_requirements[i] % 2 != 0 {
            panic!("Requirement is not even");
        }

        residual_joltage_requirements[i] /= 2;
    }
}

fn get_button_patterns_matching_light_indicator(
    joltage_requirements: &Vec<usize>,
    button_wirings: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let indicators = joltage_requirements.len();
    let button_numerical_representations =
        get_button_numerical_representations(button_wirings, indicators);

    let equivalent_light_indicator =
        get_equivalent_light_indicator(joltage_requirements, indicators);

    get_button_combinations(
        &button_numerical_representations,
        equivalent_light_indicator,
    )
}

fn get_equivalent_light_indicator(joltage_requirement: &Vec<usize>, indicators: usize) -> u16 {
    joltage_requirement
        .iter()
        .enumerate()
        .fold(0_u16, |s, (i, j)| match j {
            n if n % 2 == 1 => s + (1 << indicators - 1 - i),
            _ => s,
        })
}

fn get_button_numerical_representations(buttons: &Vec<Vec<usize>>, indicators: usize) -> Vec<u16> {
    return buttons
        .iter()
        .map(|b| b.iter().fold(0_u16, |s, n| s + (1 << indicators - 1 - n)))
        .collect::<Vec<u16>>();
}

fn get_button_combinations(
    button_wirings: &Vec<u16>,
    desired_indicator_lights: u16,
) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = vec![];

    for i in 0..2_usize.pow(button_wirings.len() as u32) {
        let mut current = 0;
        let mut buttons: Vec<usize> = vec![];
        for j in 0..button_wirings.len() {
            let button = button_wirings[j];
            if (i & (1 << j)).count_ones() == 1 {
                current ^= button;
                buttons.push(j);
            }
        }

        if current == desired_indicator_lights {
            combinations.push(buttons);
        }
    }

    combinations
}

fn parse_input() -> Result<Vec<Machine>, String> {
    let input = read_file()?;
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<&str>>();

            let joltage_requirements = parts[parts.len() - 1]
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let button_wirings = parts[1..parts.len() - 1]
                .iter()
                .map(|w| {
                    w.strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            Machine {
                button_wirings,
                joltage_requirements,
            }
        })
        .collect();

    return Ok(machines);
}

struct Machine {
    button_wirings: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
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
