use std::collections::HashSet;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;

fn run() -> Result<String, String> {
    let mut splits = 0;
    let manifold = parse_input()?;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(manifold.start.x);
    for y in manifold.start.y..manifold.height {
        let mut splitted_beams: HashSet<usize> = HashSet::new();
        for x in beams {
            if manifold.splitters.contains(&Point { x, y }) {
                splitted_beams.insert(x - 1);
                splitted_beams.insert(x + 1);
                splits += 1;
            } else {
                splitted_beams.insert(x);
            }
        }

        beams = splitted_beams;
    }
    
    return Ok(format!("{}", splits));
}

fn parse_input() -> Result<Manifold, String> {
    let input = read_file()?;
    let data: Vec<Vec<String>> = input.lines().map(|l| l.chars().map(|c| c.to_string()).collect()).collect();
    let mut start: Option<Point> = None;
    let mut splitters: HashSet<Point> = HashSet::new();
    let height = data.len();
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            match data[y][x].as_str() {
                "S" => { start = Some(Point { x, y }); },
                "^" => { splitters.insert(Point { x, y }); },
                _ => {}
            }
        }
    }

    return Ok(Manifold { start: start.unwrap(), splitters, height });
}

struct Manifold {
    start: Point,
    splitters: HashSet<Point>,
    height: usize
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
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