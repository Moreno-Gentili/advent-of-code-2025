use std::fs;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let mut input = parse_input()?;
    let adjacent_positions = [
        Point { x: -1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: -1, y: 1 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
    ];
    let mut total_removed = 0;
    let mut currently_removed = -1;
    let width = input.len();
    while currently_removed != 0 {
        currently_removed = 0;
        for x in 0..width {
            let height = input[x].len();
            for y in 0..height {
                // TODO: can I avoid cloning the row?
                let s: Vec<&Item> = input[x].iter().collect();
                match s[y] {
                    Item::RollOfPaper => {
                        let mut adjacent_rolls_of_paper = 0;
                        for pos in adjacent_positions.iter() {
                            match get_ajacent_point(x, y, pos, width, height) {
                                Some(point) => {
                                    let p = &input[point.x as usize][point.y as usize];
                                    match p {
                                    Item::RollOfPaper => {
                                        adjacent_rolls_of_paper += 1;
                                    }
                                    _ => {}
                                }
                                },
                                _ => {}
                            }
                        }

                        if adjacent_rolls_of_paper < 4 {
                            // TODO: try to use usize for x and y
                            input[x as usize][y as usize] = Item::EmptySpace;
                            currently_removed += 1;
                            total_removed += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    return Ok(format!("{}", total_removed));
}

fn get_ajacent_point(
    x: usize,
    y: usize,
    pos: &Point,
    width: usize,
    height: usize,
) -> Option<Point> {
    let adjacent_x: i32 = x as i32 + pos.x;
    let adjacent_y: i32 = y as i32 + pos.y;
    if adjacent_x < 0 || adjacent_y < 0 || adjacent_x >= width as i32 || adjacent_y >= height as i32
    {
        None
    } else {
        Some(Point {
            x: adjacent_x,
            y: adjacent_y,
        })
    }
}

fn parse_input() -> Result<Vec<Vec<Item>>, String> {
    let input = read_file()?
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => Item::RollOfPaper,
                    _ => Item::EmptySpace,
                })
                .collect()
        })
        .collect();
    return Ok(input);
}

enum Item {
    RollOfPaper,
    EmptySpace,
}

struct Point {
    x: i32,
    y: i32,
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
