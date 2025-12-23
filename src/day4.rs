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
            // count neighbouring rolls of paper
            let top = index < self.width;
            let bottom = index >= self.width * (self.height - 1) ;
            let left = index % self.width  == 0;
            let right = (index + 1) % self.width  == 0;
            let tl = if top || left {
                None
            } else {
                Some(index - self.width  - 1)
            };

            let t = if top {
                None
            } else {
                Some(index - self.width )
            };

            let tr = if top || right {
                None
            } else {
                Some(index - self.width + 1)
            };

            let l = if left {
                None
            } else {
                Some(index - 1)
            };

            let r = if right {
                None
            } else {
                Some(index + 1)
            };

            let bl = if bottom || left {
                None
            } else {
                Some(index + self.width - 1)
            };

            let b = if bottom {
                None
            } else {
                Some(index + self.width)
            };

            let br = if bottom || right {
                None
            } else {
                Some(index + self.width + 1)
            };
            let mut neighbors = 0;
            for neighbour in vec![tl, t, tr, l, r, bl, b, br] {
                if let Some(neighbour) = neighbour {
                    match &self.grid[neighbour as usize] {
                        ItemType::Empty => (),
                        ItemType::RollOfPaper => neighbors += 1,
                    }
                }
            }
            if neighbors < 4 {
                result += 1;
            }
        }
        Some(result.to_string())
    }
    fn solve2(&self) -> Option<String> {
        None
    }
}
