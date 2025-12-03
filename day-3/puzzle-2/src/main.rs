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
        let mut digits: Vec<u8> = vec![];
        let mut search_pos = 0;

        for n_digit in (1..=12).rev() {
            let l = self.batteries[search_pos..=self.batteries.len() - n_digit].iter().max().unwrap();
            let l_pos = self.batteries[search_pos..=self.batteries.len() - n_digit]
                .iter().position(|e| e.eq(l)).unwrap();
            digits.push(l.clone());
            search_pos = search_pos + l_pos + 1;
        }

        // compute value
        let mut r = 0;
        for (index, value) in digits.iter().enumerate() {
            let p = (12 - index - 1) as u32;
            let v = (*value) as u64 * 10u64.pow(p);

            r = r + v;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 3121910778619);
    }
}
