use std::env;
use std::fs;
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let mut lines = read_file(input);
    let result = process(&mut lines);
    
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

pub fn process(lines: &mut Vec<String>) -> usize {
    let last_line = lines.last().unwrap().clone();
    lines.pop(); // remove last line, otherwise it will spoil our parsing

    let longest_line_length = lines.iter().map(|l| l.len()).max().unwrap();
    let mut columns: Vec<String> = vec![];
    for column in 0..longest_line_length {
        columns.push(get_column_data(lines, column));
    }

    let mut numbers: Vec<Vec<i64>> = vec![];
    for s in columns.split(|d| d.trim().is_empty()) {
        let column_numbers: Vec<i64> = s.iter()
            .map(|n| n.trim().parse::<i64>().unwrap()).collect();
        numbers.push(column_numbers);
    }
    let operands: Vec<&str> = last_line.split_whitespace().collect();

    let solution: i64 = zip(numbers, operands)
        .map(|(n, o)| MathProblem::from(&n, o))
        .map(|mp| mp.value())
        .sum();

    solution as usize
}

fn get_column_data(lines: &Vec<String>, column: usize) -> String {
    let mut column_string: String = String::new();
    lines.iter().for_each(|line| {
        let d = line.get(column..=column).unwrap();
        column_string.push_str(d);
    });

    column_string
}

pub enum Operand {
    Plus,
    Multiply
}

impl Operand {
    pub fn from(o: &str) -> Self {
        match o {
            "+" => Operand::Plus,
            "*" => Operand::Multiply,
            _ => panic!("Unknown operand {:?}", o)
        }
    }
}

pub struct MathProblem {
    numbers: Vec<i64>,
    operand: Operand
}

impl MathProblem {
    pub fn from(numbers: &Vec<i64>, operand: &str) -> Self {
        let n = numbers.clone();
        let o = Operand::from(operand);

        MathProblem { numbers: n, operand: o }
    }

    pub fn value(&self) -> i64 {
        match self.operand {
            Operand::Plus => self.numbers.iter().fold(0, |acc, n| acc + n),
            Operand::Multiply => self.numbers.iter().fold(1, |acc, n| acc * n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&mut read_file(&String::from("../test-input")));

        assert_eq!(result, 3263827);
    }
}
