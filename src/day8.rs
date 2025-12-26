use crate::Problem;


const PAIRS: u32 = 10;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct JunctionBox<'a> {
    pos: Position,
    connections: Vec<&'a JunctionBox<'a>>,
}

impl Position {
    fn distance(self, rhs: Position) -> f32 {
        let x = (self.x as f32 - rhs.x as f32).powi(2);
        let y = (self.y as f32 - rhs.y as f32).powi(2);
        let z = (self.z as f32 - rhs.z as f32).powi(2);
        (x + y + z).sqrt()
    }
}


impl std::ops::Sub<Position> for Position {
    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
    type Output = Position;
}


#[derive(Debug, Clone)]
pub struct Day08 {
    boxes: Vec<Position>,
}

impl Problem for Day08 {
    fn day(&self) -> u8 { 8 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut positions = Vec::new();
        for line in input.lines() {
            let num_strings: Vec<u32> = line.split(',').map(|str| str.parse::<u32>().unwrap()).collect();
            assert!(num_strings.len() == 3);
            positions.push(Position {
                x: *num_strings.get(0).unwrap(),
                y: *num_strings.get(1).unwrap(),
                z: *num_strings.get(2).unwrap(),
            });
        }
        Day08 { boxes: positions }
    }
    fn solve(&self) -> Option<String> {
        // println!("{:?}", self);
        let mut min_distance = f32::MAX;
        for _ in 0..PAIRS {
            for pos1 in &self.boxes {
                let mut last_pos2: Option<&Position> = None;
                for pos2 in &self.boxes {
                    if *pos1 == *pos2 {
                        continue;
                    }

                    let d = pos1.distance(*pos2);
                    if d < min_distance {
                        last_pos2 = Some(pos2);
                        min_distance = d;
                    }
                }

                // connect
                match last_pos2 {
                    Some(last_pos2) => {
                        // println!("Smallest distance found between {:?} and {:?}", pos1, last_pos2);
                    }
                    None => (),
                }
            }
            min_distance = f32::MAX;
        }
        None
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}