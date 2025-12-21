use core::panic;



fn main() {
    let problems = vec![
        Box::new(Day01::new("inputs/day1.txt")),
    ];

    for p in problems {
        match p.solve() {
            Some(result) => println!("Solved day {}: {}", p.day(), result),
            None => println!("Couldn't solve day {}", p.day())
        }
    }

}



trait Problem {
    fn day(&self) -> u8;
    fn solve(&self) -> Option<String>;
    fn from_input(input: String) -> Self where Self: Sized;
    fn new(path: &str) -> Self where Self: Sized {
        let input = std::fs::read_to_string(path).expect("Failed to read input file");
        Self::from_input(input)
    }
}


enum RotDir {
    Clockwise,
    Counterclockwise,
}
struct Rotation {
    direction: RotDir,
    distance: u32,
}

struct Day01 {
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
            println!("{}", current_value);
        }
        Some(result.to_string())
    }
}