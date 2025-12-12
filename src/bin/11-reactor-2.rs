use std::collections::HashMap;
use std::fs;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let reactor = parse_input()?;
    let paths_count = walk(reactor.start_input, reactor.fft, &reactor.connections)
        * walk(reactor.fft, reactor.dac, &reactor.connections)
        * walk(reactor.dac, reactor.end_output, &reactor.connections);

    return Ok(format!("{}", paths_count));
}

fn walk(from: u16, to: u16, connections: &HashMap<u16, Vec<u16>>) -> u64 {
    let mut paths_cache: HashMap<u16, u64> = HashMap::new();
    recurse(from, to, connections, &mut paths_cache)
}

fn recurse(
    current: u16,
    destination: u16,
    connections: &HashMap<u16, Vec<u16>>,
    paths_cache: &mut HashMap<u16, u64>,
) -> u64 {
    let outputs = connections.get(&current);
    let mut total_paths_count = 0;
    match outputs {
        None => {}
        Some(outputs) => {
            for output in outputs {
                total_paths_count += match paths_cache.contains_key(output) {
                    true => paths_cache[output],
                    false => {
                        let subtotal_paths_count = match output {
                            value if *value == destination => 1,
                            other => recurse(*other, destination, connections, paths_cache),
                        };

                        paths_cache.insert(*output, subtotal_paths_count);
                        subtotal_paths_count
                    }
                }
            }
        }
    }

    total_paths_count
}

fn parse_input() -> Result<Reactor, String> {
    let input = read_file()?;
    let mut input_map: HashMap<String, u16> = HashMap::new();
    let connections: HashMap<u16, Vec<u16>> = input
        .lines()
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<&str>>();
            let input = resolve_name(&mut input_map, String::from(parts[0]));
            let outputs = parts[1]
                .split(' ')
                .map(|o| resolve_name(&mut input_map, String::from(o)))
                .collect();
            (input, outputs)
        })
        .collect();

    return Ok(Reactor {
        start_input: resolve_name(&mut input_map, String::from("svr")),
        end_output: resolve_name(&mut input_map, String::from("out")),
        fft: resolve_name(&mut input_map, String::from("fft")),
        dac: resolve_name(&mut input_map, String::from("dac")),
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
    fft: u16,
    dac: u16,
    connections: HashMap<u16, Vec<u16>>,
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
