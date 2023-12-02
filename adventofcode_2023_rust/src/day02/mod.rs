use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day02][part1] = {}", part1(lines.clone()));
    println!("[day02][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| get_game_info_from_line(line))
        .filter_map(|game_info| {
            let mut is_allowed = true;
            for handful_of_cubes in game_info.handfuls_of_cubes {
                for cube in handful_of_cubes.list_of_cubes {
                    match cube {
                        Cubes::Red(number) => {
                            if number > MAX_CUBES_RED {
                                is_allowed = false
                            }
                        }
                        Cubes::Green(number) => {
                            if number > MAX_CUBES_GREEN {
                                is_allowed = false
                            }
                        }
                        Cubes::Blue(number) => {
                            if number > MAX_CUBES_BLUE {
                                is_allowed = false
                            }
                        }
                    }
                }
            }
            if is_allowed {
                Some(game_info.id)
            } else {
                None
            }
        })
        .map(|id| id as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| get_game_info_from_line(line))
        .map(|game_info| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            for handful_of_cubes in game_info.handfuls_of_cubes {
                for cube in handful_of_cubes.list_of_cubes {
                    match cube {
                        Cubes::Red(number) => {
                            if number > max_red {
                                max_red = number
                            }
                        }
                        Cubes::Green(number) => {
                            if number > max_green {
                                max_green = number
                            }
                        }
                        Cubes::Blue(number) => {
                            if number > max_blue {
                                max_blue = number
                            }
                        }
                    }
                }
            }
            (max_red * max_green * max_blue) as u64
        })
        .sum::<u64>()
        .to_string()
}

const MAX_CUBES_RED: u16 = 12;
const MAX_CUBES_GREEN: u16 = 13;
const MAX_CUBES_BLUE: u16 = 14;

#[derive(Debug, Clone, PartialEq)]
struct GameInfo {
    id: u16,
    handfuls_of_cubes: Vec<HandfulOfCubes>,
}

#[derive(Debug, Clone, PartialEq)]
struct HandfulOfCubes {
    list_of_cubes: Vec<Cubes>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Cubes {
    Red(u16),
    Green(u16),
    Blue(u16),
}

fn get_game_info_from_line(line: &str) -> GameInfo {
    let mut handfuls_of_cubes = Vec::new();

    let regex_for_game = Regex::new(r"^Game (?<id>\d+): (?<handful>.+)$").unwrap();
    let game_captures = regex_for_game.captures(line).unwrap();

    let game_id = game_captures
        .name("id")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap();
    let handfuls = game_captures
        .name("handful")
        .unwrap()
        .as_str()
        .split("; ")
        .collect::<Vec<&str>>();
    let regex_for_number_and_color =
        Regex::new(r"(?<number>\d+) (?<color>red|green|blue)").unwrap();

    for handful_of_cubes in handfuls {
        let number_and_color_of_cubes = handful_of_cubes.split(", ").collect::<Vec<&str>>();
        let list_of_cubes = number_and_color_of_cubes
            .into_iter()
            .map(|number_and_color| {
                let number_and_color_captures = regex_for_number_and_color
                    .captures(number_and_color)
                    .unwrap();
                let number = number_and_color_captures
                    .name("number")
                    .unwrap()
                    .as_str()
                    .parse::<u16>()
                    .unwrap();
                match number_and_color_captures.name("color").unwrap().as_str() {
                    "red" => Cubes::Red(number),
                    "green" => Cubes::Green(number),
                    "blue" => Cubes::Blue(number),
                    _ => panic!("Unknown color"),
                }
            })
            .collect::<Vec<Cubes>>();
        handfuls_of_cubes.push(HandfulOfCubes { list_of_cubes });
    }

    GameInfo {
        id: game_id,
        handfuls_of_cubes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "8");
    }

    #[test]
    fn get_game_info_from_line_success() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            get_game_info_from_line(line),
            GameInfo {
                id: 1,
                handfuls_of_cubes: vec![
                    HandfulOfCubes {
                        list_of_cubes: vec![Cubes::Blue(3), Cubes::Red(4)],
                    },
                    HandfulOfCubes {
                        list_of_cubes: vec![Cubes::Red(1), Cubes::Green(2), Cubes::Blue(6)],
                    },
                    HandfulOfCubes {
                        list_of_cubes: vec![Cubes::Green(2)],
                    },
                ],
            }
        );
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "2286");
    }
}
