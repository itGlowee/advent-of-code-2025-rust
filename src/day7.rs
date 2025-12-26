use std::{fmt::Debug, ops::Deref};

use crate::Problem;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Entity {
    Spawn,
    Empty,
    Splitter,
    Beam,
}
#[derive(Clone)]
pub struct Day07 {
    map: Vec<Vec<Entity>>,
    width: u32,
}

fn print_map(map: &Vec<Vec<Entity>>) {
    let result: Vec<String> = (&map).iter().map(|row| {
        row.iter().map(|e| {
            match e {
                Entity::Spawn => 'S',
                Entity::Empty => '.',
                Entity::Splitter => '^',
                Entity::Beam => '|',
            }
        }).collect()
    }).collect();
    println!("{}\n", &result.join("\n"));
}


impl Problem for Day07 {
    fn day(&self) -> u8 {
        7
    }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut map: Vec<Vec<Entity>> = Vec::new();
        let mut width: u32 = 0;
        for line in input.lines() {
            width = line.len() as u32;
            let row: Vec<Entity> = line.chars().map(|c| {
                match c {
                    '.' => Entity::Empty,
                    '^' => Entity::Splitter,
                    '|' => Entity::Beam,
                    'S' => {

                        Entity::Spawn
                    },
                    c => panic!("Bad input {c}"),
                }
            }).collect();
            map.push(row);
        }
        assert!(width > 0);
        Day07 {
            map,
            width,
        }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        let mut prev_row: Option<Vec<Entity>> = None;
        let mut new_map: Vec<Vec<Entity>> = Vec::new();
        for row in &self.map {
            let mut new_row = row.clone();
            if let Some(prev_row) = prev_row {
                for i in 0..row.len() {
                    let prev_ent = prev_row.get(i).unwrap();
                    let cur_ent = row.get(i).unwrap();
                    if *prev_ent == Entity::Spawn || *prev_ent == Entity::Beam {
                        if *cur_ent == Entity::Empty {
                            new_row[i] = Entity::Beam;
                        }
                    }
                    if *prev_ent == Entity::Beam {
                        if *cur_ent == Entity::Splitter {
                            new_row[i - 1] = Entity::Beam;
                            new_row[i + 1] = Entity::Beam;
                            result += 1;
                        }
                    }
                }
            }
            prev_row = Some(new_row.clone());
            new_map.push(new_row);
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        // count unique paths WIP
        // let mut map = self.map.clone();
        // map.reverse();
        // let map = map;
        // print_map(&map.clone());
        // let mut result = 0;
        // let mut prev_row: Option<Vec<Entity>> = None;
        // let mut new_map: Vec<Vec<Entity>> = Vec::new();
        // for row in &map {
        //     let mut new_row = row.clone();
        //     if let Some(prev_row) = prev_row {
        //         for i in 0..row.len() {
        //             let prev_ent = prev_row.get(i).unwrap();
        //             let cur_ent = row.get(i).unwrap();
        //             if *prev_ent == Entity::Beam {
        //                 if *cur_ent == Entity::Empty {
        //                     new_row[i] = Entity::Beam;
        //                 }
        //             }
        //             if *prev_ent == Entity::Beam {
        //                 if *cur_ent == Entity::Splitter {
        //                     new_row[i - 1] = Entity::Beam;
        //                     new_row[i + 1] = Entity::Beam;
        //                     result += 1;
        //                 }
        //             }
        //         }
        //     }
        //     prev_row = Some(new_row.clone());
        //     new_map.push(new_row);
        // }
        // print_map(&new_map);
        // Some(result.to_string())
        None
    }
}
