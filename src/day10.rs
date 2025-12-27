use std::collections::HashSet;

use crate::Problem;


#[derive(Debug)]
struct Machine {
    goal: Vec<bool>,
    buttons: Vec<Vec<u32>>,
}

#[derive(Debug)]
pub struct Day10 {
    machines: Vec<Machine>,
}

impl Problem for Day10 {
    fn day(&self) -> u8 { 10 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut machines: Vec<Machine> = Vec::new();
        for line in input.lines() {
            let goal: Vec<bool> = line.chars().skip(1).take_while(|c| *c != ']').map(|c| match c {
                '.' => false,
                '#' => true,
                c => panic!("Invalid char {c}"),
            }).collect();
            let button_part = String::from_iter(line.chars().skip_while(|c| !c.is_whitespace()).take_while(|c| *c != '{'));
            let buttons: Vec<Vec<u32>> = button_part.trim().split(' ').map(|str| {
                str.trim_matches(&['(', ')'])
                })
                .map(|str| {
                    str.split(',').map(|str| {
                        str.parse::<u32>().expect("Not parseable to u32...")
                    }).collect::<Vec<u32>>()
                })
                .collect();
            machines.push(Machine {
                buttons: buttons,
                goal: goal,
            });
        }
        println!("{machines:?}");
        Day10 {
            machines,
        }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        for machine in &self.machines {
            let Machine { goal, buttons } = machine;
            let mut buttons = buttons.clone();
            let complexity = 1;
            // let have_answer = buttons.contains(goal);
            // while
            for i in 0..buttons.len() {
                let button1 = &buttons[i];
                for ii in (i + 1)..buttons.len() {
                    let button2 = &buttons[ii];
                    let mut new_button: Vec<u32> = Vec::new();
                    for wire in button1.iter().chain(button2.iter()) {
                        if let Some(pos) = new_button.iter().position(|n| n == wire) {
                            new_button.remove(pos);
                        }
                        else {
                            new_button.push(*wire);
                        }
                    }
                    if new_button.is_empty() {
                        continue;
                    }
                    println!("{button1:?} and {button2:?} = {new_button:?}");
                }
            }


        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}
