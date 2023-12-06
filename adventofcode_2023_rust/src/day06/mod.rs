pub fn exec(lines: Vec<String>) {
    println!("[day06][part1] = {}", part1(lines.clone()));
    println!("[day06][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    get_races(lines)
        .iter()
        .map(|race| get_winning_durations_in_ms(race.max_time_in_ms, race.min_distance_in_ms).len())
        .reduce(|acc, number_of_ways_to_win| acc * number_of_ways_to_win)
        .unwrap()
        .to_string()
}

fn part2(_lines: Vec<String>) -> String {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Race {
    max_time_in_ms: u16,
    min_distance_in_ms: u16,
}

fn get_races(lines: Vec<String>) -> Vec<Race> {
    let mut input = lines.iter();
    let times: Vec<u16> = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u16>().unwrap())
        .collect();
    let distances: Vec<u16> = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u16>().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(max_time_in_ms, min_distance_in_ms)| Race {
            max_time_in_ms,
            min_distance_in_ms,
        })
        .collect()
}

fn get_winning_durations_in_ms(max_time_in_ms: u16, min_distance_in_ms: u16) -> Vec<u16> {
    let mut winning_durations_in_ms = vec![];
    for duration_in_ms in 1..max_time_in_ms {
        let distance_in_ms = duration_in_ms * (max_time_in_ms - duration_in_ms);
        if distance_in_ms > min_distance_in_ms {
            winning_durations_in_ms.push(duration_in_ms);
        }
    }
    winning_durations_in_ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            // Time
            "Time:      7  15   30",
            // Distance
            "Distance:  9  40  200",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "288");
    }

    #[test]
    fn get_races_success() {
        let lines = vec![
            // Time
            "Time:      7  15   30",
            // Distance
            "Distance:  9  40  200",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            get_races(lines),
            vec![
                Race {
                    max_time_in_ms: 7,
                    min_distance_in_ms: 9
                },
                Race {
                    max_time_in_ms: 15,
                    min_distance_in_ms: 40
                },
                Race {
                    max_time_in_ms: 30,
                    min_distance_in_ms: 200
                }
            ]
        );
    }

    #[test]
    fn get_winning_durations_in_ms_success() {
        let max_time_in_ms = 7u16;
        let min_distance_in_ms = 9u16;
        assert_eq!(
            get_winning_durations_in_ms(max_time_in_ms, min_distance_in_ms),
            vec![2, 3, 4, 5]
        );
    }

    #[test]
    fn part2_example() {}
}
