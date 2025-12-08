use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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
    let result = process(&lines, 1000);
    
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

pub fn process(lines: &Vec<String>, max_steps: usize) -> usize {
    let junctions1: Vec<Junction> = lines.iter().map(|l| Junction::from(l)).collect();
    let junctions2 = junctions1.clone();

    // distances are in reverse order. Shortest one is the last item etc
    let mut distances = compute_distances(&junctions1, &junctions2);

    let mut junction_to_circuit: HashMap<Junction, u32> = HashMap::new();
    let mut circuit_to_junctions: HashMap<u32, HashSet<Junction>> = HashMap::new();
    junctions1.iter().enumerate().for_each(|(id, j)| {
        junction_to_circuit.insert(j.clone(), id as u32);
        let jhs: HashSet<Junction> = HashSet::from_iter(vec![j.clone()]);
        circuit_to_junctions.insert(id as u32, jhs);
    });

    // now take the first max_steps connections and join if needed
    for _ in 0..max_steps {
        let pair = distances.pop().expect("Number of pairs should be longer than max_steps");
        let j1 = pair.0;
        let j2 = pair.1;

        let c1 = junction_to_circuit.get(&j1).unwrap().clone();
        let c2 = junction_to_circuit.get(&j2).unwrap().clone();

        if c1.eq(&c2) {
            continue;
        }

        // both junctions are not in the same circuit
        // take all junctions from c2 and put them into c1
        // replace ids for all junctions of c2

        let junctions_c2 = circuit_to_junctions.get(&c2).unwrap().clone();
        let junctions_c1 = circuit_to_junctions.get_mut(&c1).unwrap();

        junctions_c2.iter().for_each(|j| {
            junctions_c1.insert(j.clone());
            junction_to_circuit.insert(j.clone(), c1);
        });
        circuit_to_junctions.remove(&c2);
    }

    let result = circuit_to_junctions.iter()
        .map(|(_, j)| j.len())
        .sorted().rev()
        .take(3)
        .product();

    result
}

fn compute_distances(junctions1: &Vec<Junction>, junctions2: &Vec<Junction>) -> Vec<(Junction, Junction, f64)> {
    let distances: Vec<(Junction, Junction, f64)> = junctions1.iter().cartesian_product(junctions2)
        .filter(|(j1, j2)| !j1.eq(j2))
        .map(|(j1, j2)| {
            (j1.clone(), j2.clone(), j1.distance(&j2))
        }).sorted_by(|j1_pair, j2_pair| {
            let d1 = j1_pair.2;
            let d2 = j2_pair.2;

            if d1 < d2 {
                Ordering::Less
            } else if d1 > d2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .rev().collect();

    let mut cleaned_distances: Vec<(Junction, Junction, f64)> = vec![];
    // skip every second element because the same pair was computes twice
    for (index, element) in distances.iter().enumerate() {
        if index % 2 == 0 {
            cleaned_distances.push(element.clone());
        }
    }

    cleaned_distances
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Junction {
    x: i64,
    y: i64,
    z: i64
}

impl Junction {
    pub fn from(input: &String) -> Self {
        let mut s = input.split(",");
        let x = s.next().unwrap().parse::<i64>().unwrap();
        let y = s.next().unwrap().parse::<i64>().unwrap();
        let z = s.next().unwrap().parse::<i64>().unwrap();

        Junction { x, y, z }
    }

    pub fn distance(&self, other: &Junction) -> f64 {
        let p1 = ((self.x - other.x) as f64).powi(2);
        let p2 = ((self.y - other.y) as f64).powi(2);
        let p3 = ((self.z - other.z) as f64).powi(2);

        (p1 + p2 + p3).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")), 10);

        assert_eq!(result, 40);
    }
}
