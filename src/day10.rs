use std::{fmt::{Debug, Display} };

use crate::Problem;

#[derive(Clone)]
struct Button {
    complexity: u32,
    wires: Vec<u32>,
    pushed: bool,
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("({})", self.wires.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
        if self.complexity > 1 {
            result += &format!("c={}", self.complexity);
        }
        f.write_str(&result)
    }
}


#[derive(Clone)]
struct Goal(Vec<bool>);

impl FromIterator<bool> for Goal {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Goal(iter.into_iter().collect())
    }
}

impl Debug for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!("[{}]", self.0.iter().map(|b| {
            match b {
                true => "#".to_string(),
                false => ".".to_string(),
            }
        }).collect::<String>());
        f.write_str(&result)
    }
}


impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?} {:?}", self.goal, self.buttons))
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // whole machine
        let result = format!("{:?} {}", self.goal, self.buttons.iter().map(|b| {
            // buttons
            let mut result = format!("\n({})", self.goal.0.iter().enumerate().map(|(i, _)| {
                // button printing
                match b.wires.contains(&(i as u32)) {
                    true => "#".to_string(),
                    false => ".".to_string(),
                }
            }).collect::<String>());
            if b.pushed {
                result = result + " *";
            }
            result
        }).collect::<String>());
            //b.wires.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
            // if b.complexity > 1 {
            //     result += &format!("c={}", b.complexity);
            // }

        f.write_str(&result)
    }
}


struct Machine {
    goal: Goal,
    buttons: Vec<Button>,
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
            let goal: Goal = line.chars().skip(1).take_while(|c| *c != ']').map(|c| match c {
                '.' => false,
                '#' => true,
                c => panic!("Invalid char {c}"),
            }).collect();
            let button_part = String::from_iter(line.chars().skip_while(|c| !c.is_whitespace()).take_while(|c| *c != '{'));
            let buttons: Vec<Button> = button_part.trim().split(' ').map(|str| {
                str.trim_matches(&['(', ')'])
                })
                .map(|str| {
                    str.split(',').map(|str| {
                        str.parse::<u32>().expect("Not parseable to u32...")
                    }).collect::<Vec<u32>>()
                })
                .map(|v| {
                    Button {
                        complexity: 1,
                        wires: v,
                        pushed: false,
                    }
                })
                .collect();
            machines.push(Machine {
                buttons: buttons,
                goal: goal,
            });
        }
        Day10 {
            machines,
        }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        for machine in &self.machines {
            let Machine { goal, buttons } = machine;
            let mut machine_solved = false;
            for complexity in 1..(machine.buttons.len() + 1) {
                let mut test: Vec<usize> = (0..complexity).collect();
                loop {
                    let mut state = goal.0.clone();
                    state.fill(false);
                    for b_index in test.iter() {
                        for wire in buttons.get(*b_index).unwrap().wires.iter() {
                            state[*wire as usize] = !state[*wire as usize];
                        }
                    }

                    if state == goal.0 {
                        machine_solved = true;
                        result += complexity;
                        break;
                    }

                    // Basically bionomial coefficient thingy...
                    let mut incremented = false;
                    for pos in (0..test.len()).rev() {
                        if test[pos] < buttons.len() - (test.len() - pos) {
                            test[pos] += 1;

                            for j in (pos + 1)..test.len() {
                                test[j] = test[j - 1] + 1;
                            }
                            incremented = true;
                            break;
                        }
                    }
                    if !incremented {
                        break;
                    }
                }
                if machine_solved {
                    break;
                }
            }
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}
