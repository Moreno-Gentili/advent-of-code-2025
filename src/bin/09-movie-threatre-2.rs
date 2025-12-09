use std::{fmt, fs};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;

use geo::{Covers, LineString, Polygon};

fn run() -> Result<String, String> {
    let red_tiles = parse_input()?;
    let polygon = build_polygon(&red_tiles);
    let mut red_tiles_pairs = pair_tiles(&red_tiles);
    red_tiles_pairs.sort_by(|a, b| (b.area() as i64).cmp(&((&a).area() as i64)));

    let mut best_contained_area: Option<i64> = None;
    for i in 0..red_tiles_pairs.len() {
        let pair = &red_tiles_pairs[i];
        if is_within(pair, &polygon) {
            best_contained_area = Some(pair.area() as i64);
            break;
        }
    }

    match best_contained_area {
        Some(value) => Ok(format!("{}", value)),
        None => Err(format!("Could not find area"))
    }
}

fn build_polygon(points: &Vec<Point>) -> Polygon {
    let point_tuples = points.iter().map(|p| (p.x, p.y)).collect::<Vec<(f64, f64)>>();
    let line_string = LineString::from(point_tuples);

    let polygon = Polygon::new(line_string, vec![]);
    return polygon;
}

fn is_within(pair: &TilePair, polygon: &Polygon) -> bool {
    let points: Vec<Point> = vec![
        Point { x: pair.first.x, y: pair.first.y },
        Point { x: pair.second.x, y: pair.first.y },
        Point { x: pair.second.x, y: pair.second.y },
        Point { x: pair.first.x, y: pair.second.y }
    ];
    let rect = build_polygon(&points);
    polygon.covers(&rect)
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


fn parse_input() -> Result<Vec<Point>, String> {
    let input = read_file()?;
    let data: Vec<Point> = input.lines().map(|l| {
        let coords: Vec<f64> = l.split(",").map(|c| c.parse::<f64>().unwrap()).collect();
        Point { x: coords[0], y: coords[1] }
    }).collect();
    
    return Ok(data);
}

struct TilePair {
    first: Point,
    second: Point
}

impl TilePair {
    fn area(&self) -> f64 {
        ((self.first.x - self.second.x).abs() + 1_f64) * 
        ((self.first.y - self.second.y).abs() + 1_f64)
    }
}

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64
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