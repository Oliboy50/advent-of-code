pub fn exec(lines: Vec<String>) {
    println!("[day04][part1] = {}", part1(lines.clone()));
    println!("[day04][part2] = {}", part2(lines));
}

pub fn part1(lines: Vec<String>) -> String {
    let (_, number_of_rolls_that_can_be_removed) =
        remove_rolls_that_have_less_than_4_rolls_around_them(
            lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        );

    number_of_rolls_that_can_be_removed.to_string()
}

fn remove_rolls_that_have_less_than_4_rolls_around_them(
    grid: Vec<Vec<char>>,
) -> (Vec<Vec<char>>, u64) {
    let mut updated_grid = grid.clone();
    let mut how_many_rolls_were_removed = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut rolls_around = 0;

            // Check up
            if r > 0 && grid[r - 1][c] == '@' {
                rolls_around += 1;
            }
            // Check up right
            if r > 0 && c < cols - 1 && grid[r - 1][c + 1] == '@' {
                rolls_around += 1;
            }
            // Check right
            if c < cols - 1 && grid[r][c + 1] == '@' {
                rolls_around += 1;
            }
            // Check down right
            if r < rows - 1 && c < cols - 1 && grid[r + 1][c + 1] == '@' {
                rolls_around += 1;
            }
            // Check down
            if r < rows - 1 && grid[r + 1][c] == '@' {
                rolls_around += 1;
            }
            // Check down left
            if r < rows - 1 && c > 0 && grid[r + 1][c - 1] == '@' {
                rolls_around += 1;
            }
            // Check left
            if c > 0 && grid[r][c - 1] == '@' {
                rolls_around += 1;
            }
            // Check up left
            if r > 0 && c > 0 && grid[r - 1][c - 1] == '@' {
                rolls_around += 1;
            }

            if rolls_around < 4 {
                updated_grid[r][c] = '.';
                how_many_rolls_were_removed += 1;
            }
        }
    }

    (updated_grid, how_many_rolls_were_removed)
}

fn recursively_remove_rolls_that_have_less_than_4_rolls_around_them(
    acc: u64,
    grid: Vec<Vec<char>>,
) -> u64 {
    let mut result = acc;

    let (updated_grid, how_many_rolls_were_removed) =
        remove_rolls_that_have_less_than_4_rolls_around_them(grid);

    if how_many_rolls_were_removed > 0 {
        result += how_many_rolls_were_removed;
        result =
            recursively_remove_rolls_that_have_less_than_4_rolls_around_them(result, updated_grid);
    }

    result
}

pub fn part2(lines: Vec<String>) -> String {
    let number_of_rolls_removed = recursively_remove_rolls_that_have_less_than_4_rolls_around_them(
        0,
        lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect(),
    );

    number_of_rolls_removed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "13".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "43".to_string());
    }
}
