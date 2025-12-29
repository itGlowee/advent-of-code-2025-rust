use std::str::FromStr;
use std::collections::HashMap;

use crate::Problem;



pub struct Day11 {
    devices: HashMap<String, Vec<String>>,
}

impl Problem for Day11 {
    fn day(&self) -> u8 {
        11
    }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut devices = HashMap::new();
        for line in input.lines() {
            devices.insert(
                line.chars().take_while(|c| *c != ':').collect(),
                Vec::from_iter(line.chars().skip_while(|c| !(*c).is_whitespace()).collect::<String>().trim().split(' ').map(|str| String::from_str(str).unwrap())),
            );
        }
        println!("{devices:?}");
        Day11 {
            devices: devices,
        }
    }
    fn solve(&self) -> Option<String> {
        let mut memo = HashMap::new(); 
        let result: u32 = count_paths("you", "out", &self.devices.clone(), &mut memo);
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}

fn count_paths(start: &str, target: &str, graph: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, u32>) -> u32 {
    if start == target {
        return 1;
    }
    if let Some(&cached) = memo.get(start) {
        return cached;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(start) {
        for next in neighbors {
            total += count_paths(next, target, graph, memo);
        }
    }
    memo.insert(start.to_string(), total);
    total
}