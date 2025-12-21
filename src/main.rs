

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


struct Day01 {
    input: String,
}

impl Problem for Day01 {
    fn day(&self) -> u8 { 1 }
    fn from_input(input: String) -> Self where Self: Sized {
        Self { input }
    }

    fn solve(&self) -> Option<String> {
        Some((&self.input.len()).to_string())
    }
}