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
    let mut column_data: Vec<Vec<i64>> = vec![];
    // prepare column vectors
    let num_columns = lines.first().unwrap().split_whitespace().count();
    for _ in 0..num_columns {
        column_data.push(vec![]);
    }

    lines[..lines.len() -1 ].iter().for_each(|line| {
        let columns = line.split_whitespace();
        for (column_index, data) in columns.enumerate() {
            let column_vec = column_data.get_mut(column_index).unwrap();
            column_vec.push(data.parse::<i64>().unwrap());
        }
    });

    let mut math_problems: Vec<MathProblem> = vec![];
    // get operand from last line and create final math problem
    for (column_index, o) in lines[lines.len() - 1].split_whitespace().enumerate() {
        let column_vec = column_data.get(column_index).unwrap();

        math_problems.push(MathProblem::from(column_vec, o));
    }

    let sum: i64 = math_problems.iter().map(|mp| mp.value()).sum();
    sum as usize
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
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 4277556);
    }
}
