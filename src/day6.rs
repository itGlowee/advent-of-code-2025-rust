use crate::Problem;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct Day06 {
    num_grid: Vec<Vec<u32>>,
    ops: Vec<Operation>,
}

impl Problem for Day06 {
    fn day(&self) -> u8 { 6 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut number_grid: Vec<Vec<u32>> = Vec::new();
        let mut ops_row: Vec<Operation> = Vec::new();
        for line in input.lines() {
            let mut iter = line.chars();
            if line.starts_with(&['*', '+']) {
                while let Some(c) = iter.next() {
                    if c == '*' {
                        ops_row.push(Operation::Mul);
                    }
                    else if c == '+' {
                        ops_row.push(Operation::Add);
                    }
                }
                assert!(ops_row.len() == number_grid[0].len());
                continue;
            }
            let mut row_nums: Vec<u32> = Vec::new();
            let mut num_str = String::with_capacity(10);
            while let Some(c) = iter.next() {
                if c.is_numeric() {
                    num_str.push(c);
                }
                else if num_str.len() > 0 {
                    row_nums.push(num_str.parse::<u32>().unwrap());
                    num_str.clear();
                }
            }
            if num_str.len() > 0 {
                row_nums.push(num_str.parse::<u32>().unwrap());
            }
            number_grid.push(row_nums);
        }
        Day06 {
            num_grid: number_grid,
            ops: ops_row,
        }
    }
    fn solve(&self) -> Option<String> {
        let mut result = 0;
        let op_iter = (&self.ops).iter().enumerate();
        for (i, op) in op_iter {
            let mut row_iter = self.num_grid.iter();
            let mut homework_result = row_iter.next().unwrap()[i] as u64;
            for row in row_iter {
                let num = row.get(i).unwrap();
                match op {
                    Operation::Add => homework_result += *num as u64,
                    Operation::Mul => homework_result *= *num as u64,
                }
            }
            result += homework_result;
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}