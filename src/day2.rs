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
        let mut result: u64 = 0;
        for range in &self.ranges {
            let mut prev = String::with_capacity(20);
            for num in range.start..=range.end {
                prev.clear();
                let num_string = num.to_string();
                for c in num_string.chars() {
                    prev.push(c);
                    let parts = num_string.len() / prev.len();
                    if parts * prev.len() != num_string.len() {
                        continue;
                    }
                    let mut matches = 0;
                    for i in 0..parts {
                        if num_string[i * prev.len()..i * prev.len() + prev.len()] != prev {
                            break;
                        }
                        else {
                            matches += 1;
                        }
                    }
                    if matches >= 2 && matches == parts {
                        result += num;
                        break;
                    }
                }
            }
        }
        Some(result.to_string())
    }
}