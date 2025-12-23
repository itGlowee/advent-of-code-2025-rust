use crate::Problem;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ItemType {
    Empty,
    RollOfPaper,
}
pub struct Day04 {
    width: u32,
    height: u32,
    grid: Vec<ItemType>,
}

impl std::fmt::Debug for Day04 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::with_capacity((self.height * self.width) as usize);
        let mut iter = self.grid.iter();
        while let Some(item) = iter.next() {
            map.push(match item {
                ItemType::Empty => '.',
                ItemType::RollOfPaper => '@',
            });
            if map.len() % self.width as usize == 0 {
                map.push('\n');
            }
        }
        f.write_str(&map)
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),  // top-left, top, top-right
    (0, -1),           (0, 1),   // left, right
    (1, -1),  (1, 0),  (1, 1),   // bottom-left, bottom, bottom-right
];

impl Problem for Day04 {
    fn day(&self) -> u8 { 4 }
    fn from_input(input: String) -> Self where Self: Sized {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut grid = vec![];
        for line in input.lines() {
            height += 1;
            if width == 0 {
                width = line.len() as u32;
            }
            grid.extend(line.chars().map(|c| match c {
                '.' => ItemType::Empty,
                '@' => ItemType::RollOfPaper,
                c => panic!("Day4 input has unexpected character: {c}"),
            }));
        }
        let result = Day04 { width, height, grid };
        result
    }
    fn solve(&self) -> Option<String> {
        let mut result: u32 = 0;
        for (index, item) in self.grid.iter().enumerate() {
            let index: u32 = index as u32;
            if let ItemType::Empty = item {
                continue;
            }

            let row = (index / self.width) as i32;
            let col = (index % self.width) as i32;
            let neighbors = DIRECTIONS.iter()
                .filter_map(|(dc, dr)| {
                    let new_row = row + dr;
                    let new_col = col + dc;
                    if new_row >= 0 && new_row < self.height as i32
                        && new_col >= 0 && new_col < self.width as i32 {
                            Some((new_row as u32 * self.width + new_col as u32) as usize)
                    } else {
                        None
                    }
                })
                .filter(|&idx| self.grid[idx] == ItemType::RollOfPaper)
                .count();

            if neighbors < 4 {
                result += 1;
            }
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        let mut grid = self.grid.clone();
        let mut result: u32 = 0;
        let mut to_remove = vec![];
        let mut removed_loop = 1;
        while removed_loop > 0 {
            removed_loop = 0;
            for (index, item) in grid.iter().enumerate() {
                let index: u32 = index as u32;
                if let ItemType::Empty = item {
                    continue;
                }

                let row = (index / self.width) as i32;
                let col = (index % self.width) as i32;
                let neighbors: Vec<usize> = DIRECTIONS.iter()
                    .filter_map(|(dc, dr)| {
                        let new_row = row + dr;
                        let new_col = col + dc;
                        if new_row >= 0 && new_row < self.height as i32
                            && new_col >= 0 && new_col < self.width as i32 {
                                Some((new_row as u32 * self.width + new_col as u32) as usize)
                        } else {
                            None
                        }
                    })
                    .filter(|&idx| grid[idx] == ItemType::RollOfPaper)
                    .collect();

                if neighbors.len() < 4 {
                    result += 1;
                    removed_loop += 1;
                    to_remove.push(index);
                }
            }
            for index in to_remove.iter() {
                grid[*index as usize] = ItemType::Empty;
            }
        }
        Some(result.to_string())
    }
}
