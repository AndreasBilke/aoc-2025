use std::collections::HashSet;
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
    let map = Map::from(lines);

    map.reachable_rolls()
}

pub struct Map {
    rolls: HashSet<(i64, i64)>
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut rolls: HashSet<(i64, i64)> = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, e) in line.chars().enumerate() {
                let roll_exists = e == '@';
                if roll_exists {
                rolls.insert((row as i64, column as i64));
                }
            }
        }

        Map { rolls }
    }

    pub fn reachable_rolls(&self) -> usize {
        let neighbour_offsets = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        self.rolls.iter().filter(|&roll| {
            let num_neighbours = neighbour_offsets.iter().filter(|&n_o| {
                let neighbour_pos = (roll.0 + n_o.0, roll.1 + n_o.1);

                self.rolls.contains(&neighbour_pos)
            }).count();

            num_neighbours < 4
        }).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 13);
    }
}
