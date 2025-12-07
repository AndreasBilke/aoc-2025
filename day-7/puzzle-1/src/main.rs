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

    map.count_beam_splits()
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

    pub fn count_beam_splits(&self) -> usize {
        let mut beam_splits = 0;

        let mut active_beam_heads: HashSet<(i64, i64)> = HashSet::new();
        active_beam_heads.insert((
            self.beam_start.0 + 1,
            self.beam_start.1
        ));

        loop {
            if active_beam_heads.len() == 0 {
                break;
            }
            let current_beams = active_beam_heads.clone();
            current_beams.iter().for_each(|bh| {
                let evolve_result = self.evolve_beam(bh.clone());
                active_beam_heads.remove(bh);

                match evolve_result {
                    BeamPosition::None => {}, // nothing to do
                    BeamPosition::One(n_bh) => {
                        active_beam_heads.insert(n_bh);
                    },
                    BeamPosition::Split(n_bh1, n_bh2) => {
                        beam_splits = beam_splits + 1;
                        active_beam_heads.insert(n_bh1);
                        active_beam_heads.insert(n_bh2);
                    }
                }
            });
        }

        beam_splits
    }

    fn evolve_beam(&self, beam_pos: (i64, i64)) -> BeamPosition {
        if beam_pos.0 >= self.map_size.0 || beam_pos.1 >= self.map_size.1 {
            return BeamPosition::None;
        }

        let next_pos = (
            beam_pos.0 + 1,
            beam_pos.1
        );
        if !self.splitters.contains(&next_pos) {
            // no split, simply move forward
            return BeamPosition::One(next_pos);
        }

        // split happened
        let b1 = (
            next_pos.0,
            next_pos.1 - 1
        );
        let b2 = (
            next_pos.0,
            next_pos.1 + 1
        );

        BeamPosition::Split(b1, b2)
    }
}

#[derive(Debug)]
pub enum BeamPosition {
    None,
    One((i64, i64)),
    Split((i64, i64), (i64, i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 21);
    }
}
