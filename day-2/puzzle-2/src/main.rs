use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = read_file(input);
    let result = process(&lines);
    
    println!("Result is {}", result);
}

pub fn read_file(file_name: &String) -> Vec<String> {
    let lines = fs::read_to_string(file_name)
        .expect("Could not read file");

    let lines: Vec<String> = lines
        .trim()
        .split('\n')
        .map(String::from)
        .collect();
    
    lines
}

pub fn process(lines: &Vec<String>) -> usize {
    let first_line = lines.iter().nth(0).unwrap(); // input has online one line
    let ranges: Vec<Range> = first_line.split(",")
        .map(|range| {
            Range::from(range)
        }).collect();

    let sum_invalid_ids = ranges.iter().map(|range|
        range.invalid_ids()
    ).sum();

    sum_invalid_ids
}

pub struct Range {
    start: i64,
    end: i64
}

impl Range {
    pub fn from(r_str: &str) -> Self {
        let parts: Vec<&str> = r_str.split("-").collect();
        if parts.len() != 2 {
            panic!("Unexpected number of numbers in range")
        }
        let s: i64 = parts.iter().nth(0).unwrap().parse().unwrap();
        let e: i64 = parts.iter().nth(1).unwrap().parse().unwrap();

        Range { start: s, end: e}
    }

    pub fn invalid_ids(&self) -> usize {
        let sum: i64 = (self.start..=self.end).filter(|id| {
            let id = id.to_string();
            for slice_size in 2..=id.len() {
                if Range::is_invalid(id.as_str(), slice_size) {
                    return true;
                }
            }

            false
        }).sum();

        sum as usize
    }

    fn is_invalid(id: &str, num_slices: usize) -> bool {
        if id.len() % num_slices != 0 {
            return false; // if length of id is not multiplicable of num_slices it cannot be a invalid pattern
        }
        let mut slices: Vec<&str> = vec![];

        let slice_length = id.len() / num_slices;
        for slice in 0..num_slices {
            let slice_start = slice * slice_length;
            let slice_end = (slice + 1) * slice_length;

            let sub_str = id.get(slice_start..slice_end).unwrap();

            slices.push(sub_str);
        }
        let first_element = slices.first().unwrap();
        slices.iter().all(|e| {
            first_element.eq(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 4174379265);
    }
}
