use crate::Problem;
use std::ops::Range;

pub struct Day02 {
    ranges: Vec<Range<u64>>,
}

impl Problem for Day02 {
    fn day(&self) -> u8 { 2 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut ranges: Vec<Range<u64>> = vec![];
        for (_, range_text) in input.split(',').enumerate() {
            let mut range_iter = range_text.split('-');

            if let (Some(first), Some(second)) = (range_iter.next(), range_iter.next()) {
                ranges.push(Range {
                    start: first.parse::<u64>().unwrap_or_else(|_| {
                        eprintln!("Error: failed to parse '{}' as u64", first);
                        0
                    }),
                    end: second.parse::<u64>().unwrap_or_else(|_| {
                        eprintln!("Error: failed to parse '{}' as u64", second);
                        0
                    }),
                });
            } else {
                eprintln!("Error: failed to parse range '{}'", range_text);
            }
        }
        Day02 { ranges: ranges }
    }
    fn solve(&self) -> Option<String> {
        let mut result: u64 = 0;
        for range in &self.ranges {
            for num in range.start..range.end {
                let num_string: String = num.to_string();
                if num_string.len() % 2 != 0 {
                    continue;
                }
                let (first, second) = num_string.split_at(num_string.len() / 2);
                if first == second {
                    result += num;
                }
            }
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}