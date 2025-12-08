use std::collections::{HashMap, HashSet};
use std::{fmt, fs};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Sub;
use std::rc::Rc;

fn run() -> Result<String, String> {
    let boxes = parse_input()?;
    let mut circuit_set: HashSet<Rc<Vec<Point>>> = HashSet::new();
    let mut circuit_map: HashMap<Point, Rc<Vec<Point>>> = create_circuits(&mut circuit_set, &boxes);
    let paired_boxes = pair_boxes(&boxes);
    let sorted_paired_boxes = sort_boxes(paired_boxes);
    let connect_amount_of_pairs = 1000;
    
    for i in 0..sorted_paired_boxes.len().min(connect_amount_of_pairs) {
        let pair = &sorted_paired_boxes[i];
        let first_circuit = circuit_map.get(&pair.first).unwrap();
        let second_circuit = circuit_map.get(&pair.second).unwrap();
        
        if Rc::ptr_eq(first_circuit, second_circuit) {
            continue;
        }

        let merged_circuit = Rc::new(merge_circuits(first_circuit, second_circuit));
        for i in 0..merged_circuit.len() {
            let point = &merged_circuit[i];
            let circuit = circuit_map.get_mut(point).unwrap();
            circuit_set.remove(circuit);
            *circuit = Rc::clone(&merged_circuit);
        }

        circuit_set.insert(merged_circuit);
    }

    let mut circuit_sizes: Vec<usize> = circuit_set.iter().map(|c| c.len()).collect();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    let take_results = 3;
    let mut result = 1;
    for i in 0..circuit_sizes.len().min(take_results) {
        result *=  circuit_sizes[i];
    }
    
    return Ok(format!("{}", result));
}

fn create_circuits(circuits_set: &mut HashSet<Rc<Vec<Point>>>, boxes: &Vec<Point>) -> HashMap<Point, Rc<Vec<Point>>> {
    let mut circuits: HashMap<Point, Rc<Vec<Point>>> = HashMap::new();
    for i in 0..boxes.len() {
        let circuit: Rc<Vec<Point>> = Rc::new(vec![boxes[i].clone()]);
        circuits_set.insert(Rc::clone(&circuit));
        circuits.insert(boxes[i].clone(), Rc::clone(&circuit));
    }

    circuits
}

fn merge_circuits(first_circuit: &Rc<Vec<Point>>, second_circuit: &Rc<Vec<Point>>) -> Vec<Point> {
    let mut merged_circuit: Vec<Point> = vec![];
    for i in 0..first_circuit.len() {
        merged_circuit.push(first_circuit[i].clone());
    }

    for i in 0..second_circuit.len() {
        merged_circuit.push(second_circuit[i].clone());
    }

    merged_circuit
}

fn pair_boxes(boxes: &Vec<Point>) -> Vec<BoxPair> {
    let mut result:Vec<BoxPair> = vec![];
    for i in 0..boxes.len()-1 {
        for j in i+1..boxes.len() {
            result.push(
                BoxPair {
                    first: boxes[i].clone(),
                    second: boxes[j].clone()
                });
        }
    }

    result
}

fn sort_boxes(mut boxes: Vec<BoxPair>) -> Vec<BoxPair> {
    boxes.sort_by(|a, b| a.distance().cmp(&b.distance()));
    boxes
}

fn parse_input() -> Result<Vec<Point>, String> {
    let input = read_file()?;
    let data: Vec<Point> = input.lines().map(|l| {
        let coords: Vec<usize> = l.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        Point { x: coords[0], y: coords[1], z: coords[2] }
    }).collect();
    
    return Ok(data);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct BoxPair {
    first: Point,
    second: Point
}

impl BoxPair {
    fn distance(&self) -> usize {
        ((self.first.x as i64 - self.second.x as i64).pow(2) + 
        (self.first.y as i64 - self.second.y as i64).pow(2) +
        (self.first.z as i64 - self.second.z as i64).pow(2)).isqrt() as usize
    }
}

impl fmt::Display for BoxPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} - {})", self.first, self.second)
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