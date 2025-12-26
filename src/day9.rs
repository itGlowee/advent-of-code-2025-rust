use crate::Problem;

#[derive(Debug)]
pub struct Day09 {
    red_tiles: Vec<(u64, u64)>,
}

impl Problem for Day09 {
    fn day(&self) -> u8 { 9 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut tiles = Vec::new();
        for line in input.lines() {
        let mut iter = line.split(',').map(|s| s.parse::<u64>().unwrap()).take(2);
            tiles.push((
                iter.next().unwrap(),
                iter.next().unwrap(),
            ));
        }
        Day09 { red_tiles: tiles }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        for i in &self.red_tiles {
            for j in &self.red_tiles {
                if i.0 == j.0 && i.1 == j.1 {
                    continue;
                }
                let diff_x = i.0.abs_diff(j.0) + 1;
                let diff_y = i.1.abs_diff(j.1) + 1;
                let area = diff_x * diff_y;
                if result < area {
                    result = area;
                }
            }

        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}