use std::collections::{HashMap, HashSet};
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

    let mut count_cache: HashMap<(i64, i64), usize> = HashMap::new();
    map.count_paths(map.beam_start, &mut count_cache)
}

#[derive(Debug)]
pub struct Map {
    splitters: HashSet<(i64, i64)>,
    beam_start: (i64, i64),
    map_size: (i64, i64)
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut splitters: HashSet<(i64, i64)> = HashSet::new();
        let mut beam_start: (i64, i64) = (0, 0);

        let map_size = (
            lines.len() as i64,
            lines.first().unwrap().len() as i64
        );

        for (row, line) in lines.iter().enumerate() {
            for (column, item) in line.chars().enumerate() {
                let pos = (row as i64, column as i64);
                match item {
                    'S' => { beam_start = pos; },
                    '^' => { splitters.insert(pos); },
                    '.' => {}, // known noop
                    _ => panic!("Unknown symbol: {:?}", item)
                }
            }
        }

        Map { splitters, beam_start, map_size }
    }

    pub fn count_paths(&self, beam: (i64, i64), cache: &mut HashMap<(i64, i64), usize>) -> usize {
        let next_split = self.next_split(&beam);

        match next_split {
            BeamSplit::None => 1,
            BeamSplit::Split(s1, s2) => {
                let num_s1 = if cache.contains_key(&s1) {
                    cache.get(&s1).unwrap().clone()
                } else {
                    let r = self.count_paths(s1.clone(), cache);
                    cache.insert(s1, r);

                    r
                };
                let num_s2 = if cache.contains_key(&s2) {
                    cache.get(&s2).unwrap().clone()
                } else {
                    let r = self.count_paths(s2.clone(), cache);
                    cache.insert(s2, r);

                    r
                };

                num_s1 + num_s2
            }
        }
    }

    fn next_split(&self, beam_pos: &(i64, i64)) -> BeamSplit {
        let mut search_pos = (beam_pos.0, beam_pos.1); // we start searching at current pos

        loop {
            if search_pos.0 >= self.map_size.0 || search_pos.1 >= self.map_size.1 {
                return BeamSplit::None;
            }
            if self.splitters.contains(&search_pos) {
                let b1 = (
                    search_pos.0,
                    search_pos.1 - 1
                );
                let b2 = (
                    search_pos.0,
                    search_pos.1 + 1
                );

                return BeamSplit::Split(b1, b2);
            }
            search_pos = (search_pos.0 + 1, search_pos.1);
        }
    }
}

#[derive(Debug)]
pub enum BeamSplit {
    None, // if beam is out of map
    Split((i64, i64), (i64, i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 40);
    }
}
