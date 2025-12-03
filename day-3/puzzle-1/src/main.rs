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
    let banks: Vec<Bank> = lines.iter().map(|l| Bank::from(l)).collect();

    let r: u64 = banks.iter().map(|b| b.joltage()).sum();

    r as usize
}

pub struct Bank {
    batteries: Vec<u8>
}

impl Bank {
    pub fn from(line: &str) -> Self {
        let batteries: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8 ).collect();
        Bank { batteries }
    }

    pub fn joltage(&self) -> u64 {
        let largest_tenth = self.batteries[0..self.batteries.len() - 1].iter().max().unwrap();
        let largest_pos = self.batteries[0..self.batteries.len() - 1]
            .iter().position(|e| e.eq(largest_tenth)).unwrap();
        let largest_oneth = self.batteries[largest_pos + 1..self.batteries.len()].iter().max().unwrap();

        let r = *largest_tenth as u64 * 10 + *largest_oneth as u64;

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 357);
    }
}
