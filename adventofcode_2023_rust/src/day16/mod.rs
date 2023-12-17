use console_engine::{Color, KeyCode};
use rayon::prelude::*;

pub fn exec(lines: Vec<String>) {
    println!("[day16][part1] = {}", part1(lines.clone(), false));
    println!("[day16][part2] = {}", part2(lines));
}

const EMPTY_SPACE: char = '.';
const MIRROR_FROM_BOTTOM_LEFT_TO_TOP_RIGHT: char = '/';
const MIRROR_FROM_TOP_LEFT_TO_BOTTOM_RIGHT: char = '\\';
const VERTICAL_SPLITTER: char = '|';
const HORIZONTAL_SPLITTER: char = '-';
const ENERGIZED_ITEM: char = '#';
const NON_ENERGIZED_ITEM: char = '.';

#[derive(Debug, Clone, Copy, PartialEq)]
struct BeamHead {
    direction: BeamDirection,
    x: isize,
    y: isize,
}

impl Default for BeamHead {
    fn default() -> Self {
        BeamHead {
            direction: BeamDirection::Right,
            x: -1,
            y: 0,
        }
    }
}

impl BeamHead {
    fn as_char(&self) -> char {
        match self.direction {
            BeamDirection::Right => '>',
            BeamDirection::Down => 'v',
            BeamDirection::Left => '<',
            BeamDirection::Up => '^',
        }
    }

    fn rotate_to_90_degrees_clockwise(&mut self) {
        self.direction = match self.direction {
            BeamDirection::Right => BeamDirection::Down,
            BeamDirection::Down => BeamDirection::Left,
            BeamDirection::Left => BeamDirection::Up,
            BeamDirection::Up => BeamDirection::Right,
        };
    }

    fn rotate_to_90_degrees_counterclockwise(&mut self) {
        self.direction = match self.direction {
            BeamDirection::Right => BeamDirection::Up,
            BeamDirection::Down => BeamDirection::Right,
            BeamDirection::Left => BeamDirection::Down,
            BeamDirection::Up => BeamDirection::Left,
        };
    }

    fn handle_mirror_from_bottom_left_to_top_right(&mut self) {
        match self.direction {
            BeamDirection::Left | BeamDirection::Right => {
                self.rotate_to_90_degrees_counterclockwise();
            }
            BeamDirection::Down | BeamDirection::Up => {
                self.rotate_to_90_degrees_clockwise();
            }
        };
    }

    fn handle_mirror_from_top_left_to_bottom_right(&mut self) {
        match self.direction {
            BeamDirection::Left | BeamDirection::Right => {
                self.rotate_to_90_degrees_clockwise();
            }
            BeamDirection::Down | BeamDirection::Up => {
                self.rotate_to_90_degrees_counterclockwise();
            }
        };
    }

    fn handle_vertical_splitter(&mut self) -> Option<Self> {
        match self.direction {
            BeamDirection::Right => {
                self.direction = BeamDirection::Up;
                let mut new_beam_head = self.clone();
                new_beam_head.direction = BeamDirection::Down;
                Some(new_beam_head)
            }
            BeamDirection::Left => {
                self.direction = BeamDirection::Up;
                let mut new_beam_head = self.clone();
                new_beam_head.direction = BeamDirection::Down;
                Some(new_beam_head)
            }
            _ => None,
        }
    }

    fn handle_horizontal_splitter(&mut self) -> Option<Self> {
        match self.direction {
            BeamDirection::Down => {
                self.direction = BeamDirection::Left;
                let mut new_beam_head = self.clone();
                new_beam_head.direction = BeamDirection::Right;
                Some(new_beam_head)
            }
            BeamDirection::Up => {
                self.direction = BeamDirection::Left;
                let mut new_beam_head = self.clone();
                new_beam_head.direction = BeamDirection::Right;
                Some(new_beam_head)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GridItem {
    kind: GridItemKind,
    is_energized: bool,
}

impl GridItem {
    fn as_char(&self) -> char {
        match self.kind {
            GridItemKind::EmptySpace => EMPTY_SPACE,
            GridItemKind::MirrorFromBottomLeftToTopRight => MIRROR_FROM_BOTTOM_LEFT_TO_TOP_RIGHT,
            GridItemKind::MirrorFromTopLeftToBottomRight => MIRROR_FROM_TOP_LEFT_TO_BOTTOM_RIGHT,
            GridItemKind::VerticalSplitter => VERTICAL_SPLITTER,
            GridItemKind::HorizontalSplitter => HORIZONTAL_SPLITTER,
        }
    }

    fn as_energized_char(&self) -> char {
        if self.is_energized {
            ENERGIZED_ITEM
        } else {
            NON_ENERGIZED_ITEM
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    items: Vec<Vec<GridItem>>,
    number_of_energized_items: usize,
}

impl From<Vec<String>> for Grid {
    fn from(lines: Vec<String>) -> Self {
        let width = lines[0].chars().count();
        let height = lines.len();
        let mut items = Vec::with_capacity(height);
        for line in lines {
            let mut row = Vec::with_capacity(width);
            for c in line.chars() {
                let kind = match c {
                    EMPTY_SPACE => GridItemKind::EmptySpace,
                    MIRROR_FROM_BOTTOM_LEFT_TO_TOP_RIGHT => {
                        GridItemKind::MirrorFromBottomLeftToTopRight
                    }
                    MIRROR_FROM_TOP_LEFT_TO_BOTTOM_RIGHT => {
                        GridItemKind::MirrorFromTopLeftToBottomRight
                    }
                    VERTICAL_SPLITTER => GridItemKind::VerticalSplitter,
                    HORIZONTAL_SPLITTER => GridItemKind::HorizontalSplitter,
                    _ => panic!("Unknown grid item kind: {}", c),
                };
                row.push(GridItem {
                    kind,
                    is_energized: false,
                });
            }
            items.push(row);
        }
        Grid {
            width,
            height,
            items,
            number_of_energized_items: 0,
        }
    }
}

impl From<Grid> for Vec<String> {
    fn from(grid: Grid) -> Self {
        grid.items
            .iter()
            .map(|row| row.iter().map(|item| item.as_char()).collect())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum BeamDirection {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridItemKind {
    EmptySpace,
    MirrorFromBottomLeftToTopRight,
    MirrorFromTopLeftToBottomRight,
    VerticalSplitter,
    HorizontalSplitter,
}

fn get_number_of_energized_items_for_starting_beam_head(
    grid: Grid,
    first_beam_head: BeamHead,
) -> usize {
    let mut grid = grid;
    let mut last_number_of_energized_items = 0;
    let mut number_of_iterations_when_new_is_same_as_last_energized_items = 0;
    let mut beam_heads = Vec::with_capacity(grid.width * grid.height);
    beam_heads.push(first_beam_head);

    loop {
        // move beam heads forward by one step
        // or remove them if they go out of bounds
        let mut beam_heads_to_remove = vec![];
        for beam_head in &mut beam_heads {
            match beam_head.direction {
                BeamDirection::Right => {
                    if beam_head.x < (grid.width - 1) as isize {
                        beam_head.x += 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Down => {
                    if beam_head.y < (grid.height - 1) as isize {
                        beam_head.y += 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Left => {
                    if beam_head.x > 0 {
                        beam_head.x -= 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Up => {
                    if beam_head.y > 0 {
                        beam_head.y -= 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
            }
        }

        // remove beam heads that went out of bounds
        for beam_head_to_remove in beam_heads_to_remove {
            let position = beam_heads
                .iter()
                .position(|beam_head| *beam_head == beam_head_to_remove)
                .unwrap();
            beam_heads.remove(position);
        }

        let mut beam_heads_to_add = vec![];
        for beam_head in &mut beam_heads {
            let beam_head_x = beam_head.x as usize;
            let beam_head_y = beam_head.y as usize;
            // energize the item at the new beam head's position
            if !grid.items[beam_head_y][beam_head_x].is_energized {
                grid.items[beam_head_y][beam_head_x].is_energized = true;
                grid.number_of_energized_items += 1;
            }
            // handle mirrors and splitters
            match grid.items[beam_head_y][beam_head_x].kind {
                GridItemKind::MirrorFromBottomLeftToTopRight => {
                    beam_head.handle_mirror_from_bottom_left_to_top_right();
                }
                GridItemKind::MirrorFromTopLeftToBottomRight => {
                    beam_head.handle_mirror_from_top_left_to_bottom_right();
                }
                GridItemKind::VerticalSplitter => {
                    if let Some(new_beam) = beam_head.handle_vertical_splitter() {
                        beam_heads_to_add.push(new_beam);
                    }
                }
                GridItemKind::HorizontalSplitter => {
                    if let Some(new_beam) = beam_head.handle_horizontal_splitter() {
                        beam_heads_to_add.push(new_beam);
                    }
                }
                _ => {}
            }
        }
        // add beam heads that were created by splitters
        for beam_head_to_add in beam_heads_to_add {
            beam_heads.push(beam_head_to_add);
        }

        // no need to continue if there is no more beam heads on the grid
        // or if the number of energized items is the same as the last iterations
        if last_number_of_energized_items == grid.number_of_energized_items {
            number_of_iterations_when_new_is_same_as_last_energized_items += 1;
        }
        last_number_of_energized_items = grid.number_of_energized_items;
        if beam_heads.is_empty()
            || number_of_iterations_when_new_is_same_as_last_energized_items > 50
        {
            break;
        }
    }

    grid.number_of_energized_items
}

fn part1(lines: Vec<String>, draw_in_terminal: bool) -> String {
    let mut grid = Grid::from(lines);
    let first_beam_head = BeamHead::default();
    let mut beam_heads = vec![first_beam_head];

    if !draw_in_terminal {
        return get_number_of_energized_items_for_starting_beam_head(grid, first_beam_head)
            .to_string();
    }

    let mut last_number_of_energized_items = 0;
    let mut number_of_iterations_when_number_of_energized_items_are_identical = 0;
    let mut engine = console_engine::ConsoleEngine::init(10, 10, 4).unwrap();
    engine.resize(grid.width as u32 + 1, grid.height as u32);
    loop {
        // wait for next frame + capture inputs
        engine.wait_frame();
        // manual exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        // reset the screen
        engine.clear_screen();

        // draw the grid
        for (y, row) in grid.items.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                engine.set_pxl(
                    x as i32,
                    y as i32,
                    console_engine::pixel::pxl_fbg(item.as_char(), Color::White, Color::Black),
                );
            }
        }
        // draw beam heads
        for beam_head in &beam_heads {
            engine.set_pxl(
                beam_head.x as i32,
                beam_head.y as i32,
                console_engine::pixel::pxl_fbg(beam_head.as_char(), Color::Green, Color::Black),
            );
        }
        // draw the screen
        engine.draw();

        // move beam heads forward by one step
        // or remove them if they go out of bounds
        let mut beam_heads_to_remove = vec![];
        for beam_head in &mut beam_heads {
            match beam_head.direction {
                BeamDirection::Right => {
                    if beam_head.x < (grid.width - 1) as isize {
                        beam_head.x += 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Down => {
                    if beam_head.y < (grid.height - 1) as isize {
                        beam_head.y += 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Left => {
                    if beam_head.x > 0 {
                        beam_head.x -= 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
                BeamDirection::Up => {
                    if beam_head.y > 0 {
                        beam_head.y -= 1;
                    } else {
                        beam_heads_to_remove.push(beam_head.clone());
                    }
                }
            }
        }

        // remove beam heads that went out of bounds
        for beam_head_to_remove in beam_heads_to_remove {
            let position = beam_heads
                .iter()
                .position(|beam_head| *beam_head == beam_head_to_remove)
                .unwrap();
            beam_heads.remove(position);
        }

        let mut beam_heads_to_add = vec![];
        for beam_head in &mut beam_heads {
            let beam_head_x = beam_head.x as usize;
            let beam_head_y = beam_head.y as usize;
            // energize the item at the new beam head's position
            if !grid.items[beam_head_y][beam_head_x].is_energized {
                grid.items[beam_head_y][beam_head_x].is_energized = true;
                grid.number_of_energized_items += 1;
            }
            // handle mirrors and splitters
            match grid.items[beam_head_y][beam_head_x].kind {
                GridItemKind::MirrorFromBottomLeftToTopRight => {
                    beam_head.handle_mirror_from_bottom_left_to_top_right();
                }
                GridItemKind::MirrorFromTopLeftToBottomRight => {
                    beam_head.handle_mirror_from_top_left_to_bottom_right();
                }
                GridItemKind::VerticalSplitter => {
                    if let Some(new_beam) = beam_head.handle_vertical_splitter() {
                        beam_heads_to_add.push(new_beam);
                    }
                }
                GridItemKind::HorizontalSplitter => {
                    if let Some(new_beam) = beam_head.handle_horizontal_splitter() {
                        beam_heads_to_add.push(new_beam);
                    }
                }
                _ => {}
            }
        }
        // add beam heads that were created by splitters
        for beam_head_to_add in beam_heads_to_add {
            beam_heads.push(beam_head_to_add);
        }

        // if no more beam heads, the game is over
        if beam_heads.is_empty()
            || number_of_iterations_when_number_of_energized_items_are_identical > 100
        {
            break;
        }

        // update last_number_of_energized_items to not loop forever
        if last_number_of_energized_items == grid.number_of_energized_items {
            number_of_iterations_when_number_of_energized_items_are_identical += 1;
        }
        last_number_of_energized_items = grid.number_of_energized_items;
    }

    loop {
        // wait for next frame + capture inputs
        engine.wait_frame();
        // manual exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        // reset the screen
        engine.clear_screen();

        // draw the energized grid
        for (y, row) in grid.items.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                engine.set_pxl(
                    x as i32,
                    y as i32,
                    console_engine::pixel::pxl_fbg(
                        item.as_energized_char(),
                        Color::Yellow,
                        Color::Black,
                    ),
                );
            }
        }
        // draw the screen
        engine.draw();
    }

    grid.number_of_energized_items.to_string()
}

fn part2(lines: Vec<String>) -> String {
    let grid = Grid::from(lines);

    let starting_beam_heads_from_left: Vec<BeamHead> = (0..grid.height)
        .into_iter()
        .map(|y| BeamHead {
            direction: BeamDirection::Right,
            x: -1,
            y: y as isize,
        })
        .collect();
    let starting_beam_heads_from_top: Vec<BeamHead> = (0..grid.width)
        .into_iter()
        .map(|x| BeamHead {
            direction: BeamDirection::Down,
            x: x as isize,
            y: -1,
        })
        .collect();
    let starting_beam_heads_from_right: Vec<BeamHead> = (0..grid.height)
        .into_iter()
        .map(|y| BeamHead {
            direction: BeamDirection::Left,
            x: grid.width as isize,
            y: y as isize,
        })
        .collect();
    let starting_beam_heads_from_bottom: Vec<BeamHead> = (0..grid.width)
        .into_iter()
        .map(|x| BeamHead {
            direction: BeamDirection::Up,
            x: x as isize,
            y: grid.height as isize,
        })
        .collect();

    starting_beam_heads_from_left
        .into_par_iter()
        .chain(starting_beam_heads_from_top.into_par_iter())
        .chain(starting_beam_heads_from_right.into_par_iter())
        .chain(starting_beam_heads_from_bottom.into_par_iter())
        .enumerate()
        .map(|(i, starting_beam_head)| {
            let number_of_energized_items = get_number_of_energized_items_for_starting_beam_head(
                grid.clone(),
                starting_beam_head,
            );
            println!("starting spot n°{i} = {number_of_energized_items}");
            number_of_energized_items
        })
        .reduce(
            || 0,
            |acc, number_of_energized_items| {
                if number_of_energized_items > acc {
                    number_of_energized_items
                } else {
                    acc
                }
            },
        )
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // use true here to see what's going wrong in real time
        let animated_debug_mode = false;
        let lines = vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines, animated_debug_mode), "46");
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "51");
    }
}
