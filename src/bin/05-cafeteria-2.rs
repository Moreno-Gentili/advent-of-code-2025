use std::{any, fs};
use std::ops::{Range, Sub};
use std::time::{SystemTime, UNIX_EPOCH};

fn run() -> Result<String, String> {
    let mut ranges = parse_input()?;
    let mut any_range_overlapping = true;
    while any_range_overlapping {
        any_range_overlapping = false;
        let mut merged_ranges: Vec<Range<i64>> = vec![];

        for range in ranges {
            let mut overlapping_index: Option<usize> = None;
            for i in 0..merged_ranges.len() {
                if ranges_overlap(&range, &merged_ranges[i]) {
                    overlapping_index = Some(i);
                    break;
                }
            }

            match overlapping_index {
                Some(i) => {
                    any_range_overlapping = true;
                    let range_start = merged_ranges[i].start.min(range.start);
                    let range_end = merged_ranges[i].end.max(range.end);
                    merged_ranges[i] = range_start..range_end;
                },
                None => {
                    merged_ranges.push(range);
                }
            }
        }

        ranges = merged_ranges;
    }

    let fresh_ids: i64 = ranges.iter().map(|r| r.end - r.start + 1).sum();
    
    return Ok(format!("{}", fresh_ids));
}

fn ranges_overlap(first: &Range<i64>, second: &Range<i64>) -> bool {
    first.start <= second.end && first.end >= second.start 
}

fn parse_input() -> Result<Vec<Range<i64>>, String> {
    let input = read_file()?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<Range<i64>> = parts[0].lines().map(|l| {
        let range: Vec<i64> = l.split("-").map(|n| n.parse::<i64>().unwrap()).collect();
        range[0]..range[1]
    }).collect();

    Ok(ranges)
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