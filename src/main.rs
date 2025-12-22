mod day1;
mod day2;
use day1::Day01;
use day2::Day02;


fn main() {
    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(Day01::new("inputs/day1.txt")),
        Box::new(Day02::new("inputs/day2.txt")),
    ];

    for p in problems {
        match p.solve() {
            Some(result) => println!("Solved day {}: {}", p.day(), result),
            None => println!("Couldn't solve day {}", p.day())
        }

        match p.solve2() {
            Some(result) => println!("Solved day {} extra: {}", p.day(), result),
            None => (),
        }
    }

}



pub trait Problem {
    fn day(&self) -> u8;
    fn solve(&self) -> Option<String>;
    fn solve2(&self) -> Option<String>;
    fn from_input(input: String) -> Self where Self: Sized;
    fn new(path: &str) -> Self where Self: Sized {
        let input = std::fs::read_to_string(path).expect("Failed to read input file");
        Self::from_input(input)
    }
}

