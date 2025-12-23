mod day1;
mod day2;
mod day3;
mod day4;
use day1::Day01;
use day2::Day02;
use day3::Day03;
use day4::Day04;


fn main() {
    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(Day01::new("inputs/day1.txt")),
        Box::new(Day02::new("inputs/day2.txt")),
        Box::new(Day03::new("inputs/day3example.txt")),
        Box::new(Day04::new("inputs/day4.txt")),
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

