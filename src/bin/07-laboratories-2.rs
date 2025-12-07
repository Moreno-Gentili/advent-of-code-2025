use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;

fn run() -> Result<String, String> {
    let manifold = parse_input()?;
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(manifold.start.x, 1);
    for y in manifold.start.y..manifold.height {
        let mut splitted_timelines: HashMap<usize, u64> = HashMap::new();
        for timeline in timelines {
            if manifold.splitters.contains(&Point { x: timeline.0, y }) {
                add_beam_timeline(&mut splitted_timelines, timeline, -1);
                add_beam_timeline(&mut splitted_timelines, timeline, 1);
            } else {
                add_beam_timeline(&mut splitted_timelines, timeline, 0);
            }
        }

        timelines = splitted_timelines;
    }
    
    let count: u64 = timelines.iter().map(|t| t.1).sum();
    return Ok(format!("{}", count));
}

fn add_beam_timeline(timelines: &mut HashMap<usize, u64>, timeline: (usize, u64), x_delta: i32) {
    let key: usize = ((timeline.0 as i32) + x_delta) as usize;
    if let Some(value) = timelines.get_mut(&key) {
        *value += timeline.1;
    } else {
        timelines.insert(key, timeline.1);
    }
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