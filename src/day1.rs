use core::panic;
use crate::Problem;


enum RotDir {
    Clockwise,
    Counterclockwise,
}
struct Rotation {
    direction: RotDir,
    distance: u32,
}

pub struct Day01 {
    rotations: Vec<Rotation>
}

impl Problem for Day01 {
    fn day(&self) -> u8 { 1 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mapper = input.lines()
            .map(|line| Rotation {
                direction: match line.as_bytes().first() {
                    Some(b'L') => RotDir::Counterclockwise,
                    Some(b'R') => RotDir::Clockwise,
                    Some(_) | None => panic!("Could not get direction from line"),
                },
                distance: line[1..].parse().unwrap()
        });
        Self {rotations: mapper.collect() }
    }

    fn solve(&self) -> Option<String> {
        let mut current_value: u32 = 50;
        let mut result: u32 = 0;
        for rotation in &self.rotations {
            match rotation.direction {
                RotDir::Clockwise => current_value = (current_value + (rotation.distance % 100)) % 100,
                RotDir::Counterclockwise => current_value = (current_value + 100 - (rotation.distance % 100)) % 100,
            }
            if current_value == 0 {
                result += 1;
            }
        }
        Some(result.to_string())
    }
}

impl Day01 {
    pub fn solve2(&self) -> Option<String> {
        let mut current_value: u32 = 50;
        let mut result: u32 = 0;
        for rotation in &self.rotations {
            result += rotation.distance / 100; // full turns
            match rotation.direction {
                RotDir::Clockwise => {
                    if (current_value + (rotation.distance % 100)) > 100 {
                        result += 1; // leftover turn
                    }
                    current_value = (current_value + (rotation.distance % 100)) % 100;
                },
                RotDir::Counterclockwise => {
                    if current_value > 0 && current_value < (rotation.distance % 100) {
                        result += 1; // leftover turn
                    }
                    current_value = (current_value + 100 - (rotation.distance % 100)) % 100;
                }
            }
            if current_value == 0 {
                result += 1;
            }
        }
        Some(result.to_string())
    }
}