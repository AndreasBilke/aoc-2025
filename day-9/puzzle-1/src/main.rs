use std::collections::HashSet;
use std::env;
use std::fs;
use itertools::Itertools;

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
    let map = Map::from(lines);
    map.largest_rect() as usize
}

#[derive(Debug)]
pub struct Map {
    tiles: HashSet<(i64, i64)>
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let tiles: HashSet<(i64, i64)> = lines.iter().map(|l| {
            let mut parts = l.split(",");
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();

            (x, y)
        }).collect();

        Map { tiles }
    }

    pub fn largest_rect(&self) -> i64 {
        self.tiles.iter().cartesian_product(self.tiles.iter())
            .map(|(t1, t2)| {
                let x_length = (t1.0 - t2.0).abs() + 1;
                let y_length = (t1.1 - t2.1).abs() + 1;

                x_length * y_length
            })
            .sorted()
            .max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 50);
    }
}
