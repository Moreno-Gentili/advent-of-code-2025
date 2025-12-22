use std::fs;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let trees = parse_input()?;
    let fitting_presents = trees.iter().filter(|xt| {
        let region_area = xt.region.width * xt.region.height;
        let presents_area: usize = xt.presents.iter().map(|p| {
            let present_area = p.shape.0.iter().map(|f| f.iter().filter(|c| **c).count() as usize).sum::<usize>();
            p.quantity * present_area
        }).sum();
        
        presents_area <= region_area // haha, why is this working? Santa has been too nice...
    }).count();

    return Ok(format!("{}", fitting_presents));
}

fn parse_input() -> Result<Vec<XmasTree>, String> {
    let input = read_file()?;
    let parts: Vec<String> = input.split("\n\n").map(|l| l.to_string()).collect();
    let mut shapes: Vec<Shape> = vec![];
    
    for _ in 0..parts.len()-1 {
        let shape = parts[1]
        .lines()
        .skip(1)
        .map(|r| 
            r.chars().map(|c| match c {
            '#' => true,
            _ => false
        }).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

        shapes.push(Shape(shape));
    }

    let trees = parts.last().unwrap().lines().map(|l| {
        let tree_parts = l.split(": ").collect::<Vec<&str>>();
        let dimensions: Vec<usize> = tree_parts[0].split("x").map(|d| d.parse::<usize>().unwrap()).collect();
        let actual_presents: Vec<Present> = tree_parts[1].split(" ").enumerate().map(|(i, q)| Present { quantity: q.parse::<usize>().unwrap(), shape: Shape(shapes[i].0.clone()) } ).collect();
        XmasTree { region: Region { width: dimensions[0], height: dimensions[1] }, presents: actual_presents }
    }).collect::<Vec<XmasTree>>();

    return Ok(trees);
}

struct XmasTree {
    region: Region,
    presents: Vec<Present>,
}

struct Region {
    width: usize,
    height: usize
}

struct Present {
    shape: Shape,
    quantity: usize
}

struct Shape(Vec<Vec<bool>>);

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
