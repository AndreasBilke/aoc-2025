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

pub fn process(lines: &Vec<String>) -> i32 {
    let mut current_pos = 50;
    let mut zero_hits_counter = 0;

    lines.iter().for_each(|line| {
        let movement = Movement::from(line);
        let (new_current_pos, zero_hits) = move_dial(current_pos, movement);
        println!("{:?} -- {:?}", new_current_pos, zero_hits);

        current_pos = new_current_pos;
        zero_hits_counter = zero_hits_counter + zero_hits;
    });

    zero_hits_counter
}

fn move_dial(current_pos: i32, m: Movement) -> (i32, i32) {
    let mut zero_hits = m.amount / 100; // number of full turns
    let amount = m.amount % 100; // ignore full turn around

    if m.direction == Direction::Right {
        let n = (current_pos + amount) % 100;
        if n < current_pos {
            zero_hits = zero_hits + 1;
        }
        (n, zero_hits)
    } else {
        let t_pos = current_pos - amount;
        let n = if t_pos < 0 {
            100 + t_pos
        } else {
            t_pos
        };

        if current_pos != 0 && n > current_pos { // ignore situations where we started at zero. Otherwise, we count 0 twice
            zero_hits = zero_hits + 1;
        }
        if n == 0 { // dial hit 0 directly, no wrap around
            zero_hits = zero_hits + 1;
        }

        (n, zero_hits)
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

        assert_eq!(result, 6);
    }
}
