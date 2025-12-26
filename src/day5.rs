use crate::Problem;
use std::ops::{Deref, Range};
pub struct Day05 {
    ranges: Vec<Range<u64>>,
    wanna_know: Vec<u64>,
}


impl Problem for Day05 {
    fn day(&self) -> u8 { 5 }
    fn from_input(input: String) -> Self where Self: Sized {
        let ranges = input.lines()
            .take_while(|line| line.len() != 0 )
            .map(|line| {
                let (start, end) = line.split_at(line.find('-').unwrap());
                let end  = end.trim_start_matches('-');
                Range {
                    start: start.parse::<u64>().unwrap(),
                    end: end.parse::<u64>().unwrap() + 1,
                }
            }).collect();
        let wanna_know: Vec<u64> = input.lines().into_iter()
            .skip_while(|line| line.len() > 0)
            .skip(1)
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        Day05 {
            ranges: ranges,
            wanna_know: wanna_know
        }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        let mut ranges = self.ranges.clone();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        for num in &self.wanna_know {
            for range in &ranges {
                if range.contains(num) {
                    result += 1;
                    break;
                }
            }
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        let mut result = 0;
        let mut ranges = self.ranges.clone();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let mut prev: Range<u64> = ranges[0].clone();
        for range in ranges.iter_mut().skip(1) {
            if range.start < prev.end {
                range.start = prev.end;
                if range.start > range.end {
                    range.end = range.start; // basically empty range
                }
            }   
            prev = range.clone();
        }
        
        for range in &ranges {
            result += range.end - range.start;
        }
        Some(result.to_string())
    }
}