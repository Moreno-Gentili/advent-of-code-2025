use std::collections::{HashMap, VecDeque};
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, usize};

fn run() -> Result<String, String> {
    let reactor = parse_input()?;
    let mut paths_count: usize = 0;

    let mut queue: VecDeque<Iteration> = VecDeque::new();
    queue.push_back(Iteration {
        input: reactor.start_input
    });

    while let Some(iteration) = queue.pop_front() {
        let outputs = &reactor.connections[&iteration.input];
        for output in outputs {
            if *output == reactor.end_output {
                paths_count += 1;
            } else {
                queue.push_back(Iteration { input: *output });
            }
        }
    }

    return Ok(format!("{}", paths_count));
}

fn parse_input() -> Result<Reactor, String> {
    let input = read_file()?;
    let mut input_map: HashMap::<String, u16> = HashMap::new();
    let connections: HashMap<u16, Vec<u16>> = input
        .lines()
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<&str>>();
            let input = resolve_name(&mut input_map, String::from(parts[0]));
            let outputs = parts[1].split(' ').map(|o| resolve_name(&mut input_map, String::from(o))).collect();
            (input, outputs)
        })
        .collect();

    return Ok(Reactor {
        start_input: resolve_name(&mut input_map, String::from("you")),
        end_output: resolve_name(&mut input_map,String::from("out")),
        connections,
    });
}

fn resolve_name(map: &mut HashMap<String, u16>, name: String) -> u16 {
    if map.contains_key(&name) {
        map.get(&name).unwrap().clone()
    } else {
        let value = map.len() as u16;
        map.insert(name, value);
        value
    }
}

struct Reactor {
    start_input: u16,
    end_output: u16,
    connections: HashMap<u16, Vec<u16>>,
}

struct Iteration {
    input: u16,
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
