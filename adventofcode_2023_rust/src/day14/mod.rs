pub fn exec(lines: Vec<String>) {
    println!("[day14][part1] = {}", part1(lines.clone()));
    println!("[day14][part2] = {}", part2(lines));
}

const EMPTY_SPACE: char = '.';
const ROUNDED_ROCK: char = 'O';
const CUBE_SHAPED_ROCK: char = '#';

#[derive(Debug, Clone)]
struct Grid {
    orientation: GridOrientation,
    width: usize,
    height: usize,
    items: Vec<Vec<GridItemKind>>,
}

impl From<Vec<String>> for Grid {
    fn from(lines: Vec<String>) -> Self {
        let width = lines[0].chars().count();
        let height = lines.len();
        let mut items = Vec::with_capacity(height);
        for line in lines.into_iter() {
            let mut items_row = Vec::with_capacity(width);
            for char in line.chars() {
                let kind = GridItemKind::from(char);
                items_row.push(kind);
            }
            items.push(items_row);
        }
        Grid {
            orientation: Default::default(),
            width,
            height,
            items,
        }
    }
}

impl From<Grid> for Vec<String> {
    fn from(grid: Grid) -> Self {
        let width = grid.width;
        let height = grid.height;
        let mut lines = Vec::with_capacity(height);
        for y in 0..height {
            let mut chars = String::with_capacity(width);
            for x in 0..width {
                chars.push(char::from(grid.items[y][x]));
            }
            lines.push(chars);
        }
        lines
    }
}

impl Grid {
    fn get_rounded_rocks_load(&self) -> u64 {
        let mut result = 0u64;
        for (y, line) in self.items.iter().enumerate() {
            for item in line.iter() {
                if matches!(item, GridItemKind::RoundedRock) {
                    result += (self.height - y) as u64;
                }
            }
        }
        result
    }

    fn get_rounded_rocks_positions(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.items[y][x], GridItemKind::RoundedRock) {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn move_rocks_until_items_are_blocked(&mut self) {
        let (x_step, y_step) = match self.orientation {
            GridOrientation::North => (0isize, -1isize),
            _ => todo!("Implement moving rocks to other directions"),
        };
        let mut non_blocked_rounded_rocks_positions = self.get_rounded_rocks_positions();

        loop {
            if non_blocked_rounded_rocks_positions.is_empty() {
                break;
            }

            // loop through rounded rocks and move them
            let mut rounded_rocks_to_block_indexes =
                Vec::with_capacity(non_blocked_rounded_rocks_positions.len());
            for (index, (x, y)) in non_blocked_rounded_rocks_positions.iter_mut().enumerate() {
                let new_x = x_step + *x as isize;
                let new_y = y_step + *y as isize;
                if new_x < 0
                    || new_x >= self.width as isize
                    || new_y < 0
                    || new_y >= self.height as isize
                {
                    rounded_rocks_to_block_indexes.push(index);
                    continue;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;
                match self.items[new_y][new_x] {
                    GridItemKind::EmptySpace => {
                        self.items[new_y][new_x] = GridItemKind::RoundedRock;
                        self.items[*y][*x] = GridItemKind::EmptySpace;
                        *x = new_x;
                        *y = new_y;
                    }
                    _ => {
                        rounded_rocks_to_block_indexes.push(index);
                    }
                }
            }

            // mark rounded rocks as blocked
            for index in rounded_rocks_to_block_indexes.into_iter().rev() {
                non_blocked_rounded_rocks_positions.remove(index);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum GridOrientation {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum GridItemKind {
    #[default]
    EmptySpace,
    CubeShapedRock,
    RoundedRock,
}

impl From<char> for GridItemKind {
    fn from(value: char) -> Self {
        match value {
            EMPTY_SPACE => GridItemKind::EmptySpace,
            ROUNDED_ROCK => GridItemKind::RoundedRock,
            CUBE_SHAPED_ROCK => GridItemKind::CubeShapedRock,
            _ => panic!("Unsupported grid item"),
        }
    }
}

impl From<GridItemKind> for char {
    fn from(value: GridItemKind) -> Self {
        match value {
            GridItemKind::EmptySpace => EMPTY_SPACE,
            GridItemKind::RoundedRock => ROUNDED_ROCK,
            GridItemKind::CubeShapedRock => CUBE_SHAPED_ROCK,
        }
    }
}

fn part1(lines: Vec<String>) -> String {
    let mut grid = Grid::from(lines);
    grid.move_rocks_until_items_are_blocked();
    grid.get_rounded_rocks_load().to_string()
}

fn part2(_lines: Vec<String>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            r"O....#....",
            r"O.OO#....#",
            r".....##...",
            r"OO.#O....O",
            r".O.....O#.",
            r"O.#..O.#.#",
            r"..O..#O..O",
            r".......O..",
            r"#....###..",
            r"#OO..#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "136");
    }

    #[test]
    fn grid_move_rocks_until_items_are_blocked_to_north_success() {
        let lines = vec![
            r"O....#....",
            r"O.OO#....#",
            r".....##...",
            r"OO.#O....O",
            r".O.....O#.",
            r"O.#..O.#.#",
            r"..O..#O..O",
            r".......O..",
            r"#....###..",
            r"#OO..#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let expected = vec![
            r"OOOO.#.O..",
            r"OO..#....#",
            r"OO..O##..O",
            r"O..#.OO...",
            r"........#.",
            r"..#....#.#",
            r"..O..#.O.O",
            r"..O.......",
            r"#....###..",
            r"#....#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let mut grid = Grid::from(lines);
        assert_eq!(grid.orientation, GridOrientation::North);

        grid.move_rocks_until_items_are_blocked();
        assert_eq!(Vec::<String>::from(grid), expected);
    }

    #[test]
    fn grid_get_rounded_rocks_load_success() {
        let lines = vec![
            r"OOOO.#.O..",
            r"OO..#....#",
            r"OO..O##..O",
            r"O..#.OO...",
            r"........#.",
            r"..#....#.#",
            r"..O..#.O.O",
            r"..O.......",
            r"#....###..",
            r"#....#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let grid = Grid::from(lines);

        assert_eq!(grid.get_rounded_rocks_load(), 136);
    }

    #[test]
    fn part2_example() {}
}
