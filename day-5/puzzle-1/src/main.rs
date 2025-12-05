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
    let mut input_split = lines.split(|line| {
        line.len() == 0
    });
    let db_ranges: Vec<String> = input_split.next().unwrap().to_vec();
    let ranges = Database::from(&db_ranges);

    let ids: Vec<u64> = input_split.next().unwrap()
        .iter().map(|str_id| str_id.parse::<u64>().unwrap()).collect();

    ids.iter().filter(|&&id| {
        ranges.is_fresh(id)
    }).count()
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

    pub fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|range| {
            id >= range.0 && id <= range.1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 3);
    }
}
