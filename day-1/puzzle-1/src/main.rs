use std::cmp::PartialEq;
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
    let mut current_pos = 50;
    let mut zero_pos_counter = 0;

    lines.iter().for_each(|line| {
        let movement = Movement::from(line);
        current_pos = move_dial(current_pos, movement);
        if current_pos == 0 {
            zero_pos_counter = zero_pos_counter + 1;
        }
    });

    zero_pos_counter
}

fn move_dial(c: i32, m: Movement) -> i32 {
    let amount = m.amount % 100; // ignore full turn around

    if m.direction == Direction::Right {
        (c + amount) % 100
    } else {
        let n = c - amount;
        if n < 0 {
            100 + n
        } else {
            n
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    amount: i32
}

impl Movement {
    pub fn from(line: &String) -> Self {
        let direction = match line.chars().nth(0).unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction")
        };

        let amount: i32 = line.get(1..).unwrap().parse().unwrap();

        Movement { direction, amount }
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
