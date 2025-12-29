use std::fmt::Debug;

use crate::Problem;

const WIDTH: u32 = 3;

#[derive(Debug)]
struct Region {
    presents: Vec<u32>,
    width: u32,
    height: u32,
}
pub struct Day12 {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

struct Shape(Vec<bool>);

impl Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("\n");
        for (i, &b) in self.0.iter().enumerate() {
            result.push(match b {
                true => '#',
                false => '.',
            });
            if (i + 1) as u32 % WIDTH == 0 {
                result.push('\n');
            }
        }
        result.push('\n');
        f.write_str(&result)
    }
}


impl Problem for Day12 {
    fn day(&self) -> u8 {
        12
    }
    fn from_input(input: String) -> Self where Self: Sized {
        let shapes = input.lines().take_while(|line| {
            !line.contains('x')
        }).collect::<String>();
        let shapes = shapes.replace(char::is_numeric, "");
        let shapes = shapes.split(':').filter(|str| !str.is_empty());
        let shapes: Vec<Shape> = shapes.map(|str| {
            Shape(str.chars().map(|c| c == '#').collect())
        }).collect();
        let regions  = input.lines().skip_while(|line| !line.contains('x'));
        let regions = regions.map(|line| {
            println!("{line}");
            let mut chars = line.chars().into_iter();
            Region {
                width: chars.by_ref().take_while(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap(),
                height: chars.by_ref().take_while(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap(),
                presents: chars
                    .skip(2)
                    .collect::<String>()
                    .split(' ')
                    .filter(|str| !str.is_empty())
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect(),
            }
        }).collect::<Vec<Region>>();

        Day12 {
            shapes: shapes,
            regions: regions,
        }
    }
    fn solve(&self) -> Option<String> {
        None
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}