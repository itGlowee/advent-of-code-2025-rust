use std::ptr::{null, null_mut};

use crate::Problem;

pub struct Day03 {
    banks: Vec<Vec<u8>>
}

impl Problem for Day03 {
    fn day(&self) -> u8 { 3 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut banks: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            banks.push(line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .map(|num| num as u8)
                .collect()
            );
        }
        Day03 { banks: banks }
    }
    fn solve(&self) -> Option<String> {
        let mut result: usize = 0;
        for bank in &self.banks {
            let mut highest: usize = 0;
            let mut first: usize = 0;
            let mut second: usize = 0;
            for (i, num) in bank.iter().enumerate() {
                for (ii, num2) in bank.iter().enumerate() {
                    if i >= ii {
                        continue;
                    }
                    let total: usize = (*num as usize) * 10 + (*num2 as usize);
                    if total > highest {
                        highest = total;
                        first = i;
                        second = ii;
                    }
                }
            }
            result += highest;
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}