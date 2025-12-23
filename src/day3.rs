use crate::Problem;

pub struct Day03 {
    banks: Vec<Vec<u8>>
}

impl Problem for Day03 {
    fn day(&self) -> u8 { 3 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut banks: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            banks.push(line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .map(|num| num as u8)
                .collect()
            );
        }
        Day03 { banks: banks }
    }
    fn solve(&self) -> Option<String> {
        let mut result: usize = 0;
        for bank in &self.banks {
            let mut highest: usize = 0;
            let mut first: usize = 0;
            let mut second: usize = 0;
            for (i, num) in bank.iter().enumerate() {
                for (ii, num2) in bank.iter().enumerate() {
                    if i >= ii {
                        continue;
                    }
                    let total: usize = (*num as usize) * 10 + (*num2 as usize);
                    if total > highest {
                        highest = total;
                        first = i;
                        second = ii;
                    }
                }
            }
            result += highest;
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        return None;
        let mut result: usize = 0;
        let digits = 12;
        for bank in &self.banks {
            let mut turned_on: Vec<usize> = vec![];
            for (i, _num) in bank.iter().rev().enumerate().take(digits) {
                let index = bank.len() - 1 - i;
                turned_on.push(index);
            }
            // println!("{:?}", turned_on);
            // println!("Joltage is {}", self.calc_joltage(bank, &turned_on));
            let mut highest: usize = 0;
            for (i, num) in bank.iter().rev().enumerate() {
                let joltage = self.calc_joltage(bank, &turned_on);
                if joltage > highest {
                    highest = joltage;
                }
                else {
                    // move a switch
                    // remove the lowest one.
                    let mut temp = turned_on.remove(0);
                    for ii in (0..temp).rev() {
                        if let Some(switch_index) = turned_on.iter().position(|&item| item == ii) {
                            // I'll do this later i guess this seems like a dead end
                        }
                        else {

                        }
                    }
                }
                for (ii, num2) in bank.iter().enumerate() {
                    if i >= ii {
                        continue;
                    }
                    // if total > highest {
                    //     highest = total;
                    // }
                }
            }
            result += highest;
        }
        Some(result.to_string())
    }
}

impl Day03 {
    fn calc_joltage(&self, bank: &Vec<u8>, turned_on: &Vec<usize>) -> usize {
        let mut result: usize = 0;
        let bank_len = bank.len();
        for index in turned_on {
            let significance: usize = 10usize.pow((bank_len - 1 - index) as u32);
            result += significance * (bank[*index] as usize);
        }
        result
    }
}