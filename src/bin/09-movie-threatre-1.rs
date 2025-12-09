use std::{fmt, fs};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;

fn run() -> Result<String, String> {
    let red_tiles = parse_input()?;
    let red_tiles_pairs = pair_tiles(&red_tiles);
    let area = red_tiles_pairs.iter().map(|t| t.area()).max().unwrap();
    return Ok(format!("{}", area));
}

fn parse_input() -> Result<Vec<Point>, String> {
    let input = read_file()?;
    let data: Vec<Point> = input.lines().map(|l| {
        let coords: Vec<usize> = l.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        Point { x: coords[0], y: coords[1] }
    }).collect();
    
    return Ok(data);
}

fn pair_tiles(tiles: &Vec<Point>) -> Vec<TilePair> {
    let mut result:Vec<TilePair> = vec![];
    for i in 0..tiles.len()-1 {
        for j in i+1..tiles.len() {
            result.push(
                TilePair {
                    first: tiles[i].clone(),
                    second: tiles[j].clone()
                });
        }
    }

    result
}

struct TilePair {
    first: Point,
    second: Point
}

impl TilePair {
    fn area(&self) -> usize {
        (((self.first.x as i64 - self.second.x as i64).abs() + 1) * 
        ((self.first.y as i64 - self.second.y as i64).abs() + 1)) as usize
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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