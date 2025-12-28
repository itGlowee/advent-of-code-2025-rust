use std::fmt::{Debug, Display};

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
            let mut buttons = buttons.clone();
            let mut solved: Option<u32> = None;
            while solved.is_none() {
                let mut new_buttons = Vec::new();
                let mut new_button = Button { complexity: 0, wires: Vec::new(), pushed: false };
                for i in 0..buttons.len() {
                    if solved.is_some() {
                        break;
                    }
                    let button1 = &mut buttons[i].clone();
                    for ii in (i + 1)..buttons.len() {
                        let button2 = &mut buttons[ii].clone();

                        if goal.0.iter().enumerate().all(|(i, on)| {
                                button1.wires.contains(&(i as u32)) == *on
                        }) {
                            button1.pushed = true;
                            buttons[i] = button1.clone();
                            solved = Some(button1.complexity);
                            println!("Solved with {button1:?}");
                            break;
                        } else if goal.0.iter().enumerate().all(|(i, on)| {
                                button2.wires.contains(&(i as u32)) == *on
                        }) {
                            button2.pushed = true;
                            buttons[ii] = button2.clone();
                            solved = Some(button2.complexity);
                            println!("Solved with {button2:?}");
                            break;
                        }

                        new_button.wires.clear();
                        new_button.complexity = button1.complexity + button2.complexity;
                        for wire in button1.wires.iter().chain(button2.wires.iter()) {
                            if let Some(pos) = new_button.wires.iter().position(|n| n == wire) {
                                new_button.wires.remove(pos);
                            }
                            else {
                                new_button.wires.push(*wire);
                            }
                        }
                        if new_button.wires.is_empty() {
                            continue;
                        }
                        if goal.0.iter().enumerate().all(|(i, on)| {
                                new_button.wires.contains(&(i as u32)) == *on
                        }) {
                            button1.pushed = true;
                            buttons[i] = button1.clone();
                            button2.pushed = true;
                            buttons[ii] = button2.clone();
                            solved = Some(new_button.complexity);
                            println!("Solved with {new_button:?}");
                            // new_buttons.push(new_button.clone());
                            break;
                        }
                        // new_buttons.push(new_button.clone());
                    }
                }
                buttons.append(&mut new_buttons);
            }
            result += solved.unwrap();
            let new_machine = Machine {
                buttons,
                goal: goal.clone(),
            };
            println!("{new_machine}");
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}
