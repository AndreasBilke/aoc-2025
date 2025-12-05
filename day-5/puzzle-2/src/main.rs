use std::env;
use std::fs;
use range_set::range_set;

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
    let mut input_split = lines.split(|line| {
        line.len() == 0
    });
    let db_ranges: Vec<String> = input_split.next().unwrap().to_vec();
    let ranges = Database::from(&db_ranges);

    let first_range = ranges.ranges.first().unwrap();
    let mut range_set = range_set!((first_range.0)..=(first_range.1));

    ranges.ranges.iter().for_each(|r| {
        let r = range_set!((r.0)..=(r.1));

        let new_r = range_set.union(&r);
        range_set = new_r;
    });

    range_set.len()
}

pub struct Database {
    ranges: Vec<(u64, u64)>
}

impl Database {
    pub fn from(lines: &Vec<String>) -> Self {
        let ranges: Vec<(u64, u64)> = lines.iter().map(|l| {
            let mut r = l.split("-");
            let s: u64 = r.next().unwrap().parse().unwrap();
            let e: u64 = r.next().unwrap().parse().unwrap();

            (s, e)
        }).collect();

        Database { ranges }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 14);
    }
}
