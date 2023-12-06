pub fn exec(lines: Vec<String>) {
    println!("[day05][part1] = {}", part1(lines.clone()));
    println!("[day05][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let seed_ids = get_seed_ids(&lines);
    let seed_to_soil_map = get_seed_to_soil_map(&lines);
    let soil_to_fertilizer_map = get_soil_to_fertilizer_map(&lines);
    let fertilizer_to_water_map = get_fertilizer_to_water_map(&lines);
    let water_to_light_map = get_water_to_light_map(&lines);
    let light_to_temperature_map = get_light_to_temperature_map(&lines);
    let temperature_to_humidity_map = get_temperature_to_humidity_map(&lines);
    let humidity_to_location_map = get_humidity_to_location_map(&lines);

    seed_ids
        .into_iter()
        .map(|id| {
            get_seed_from_maps(
                id,
                &seed_to_soil_map,
                &soil_to_fertilizer_map,
                &fertilizer_to_water_map,
                &water_to_light_map,
                &light_to_temperature_map,
                &temperature_to_humidity_map,
                &humidity_to_location_map,
            )
        })
        .fold(
            u64::MAX,
            |acc, Seed { location, .. }| {
                if location < acc {
                    location
                } else {
                    acc
                }
            },
        )
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Seed {
    id: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn get_destination_id_for_source_id(&self, source_id: u64) -> u64 {
        for MapRange {
            destination_range_start,
            source_range_start,
            range_length,
        } in self.ranges.iter()
        {
            for (i, id) in (*source_range_start..(source_range_start + range_length))
                .into_iter()
                .enumerate()
            {
                if id == source_id {
                    return destination_range_start + (i as u64);
                }
            }
        }

        source_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MapRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn get_seed_ids(lines: &[String]) -> Vec<u64> {
    let numbers: &str = lines[0].split(": ").collect::<Vec<_>>()[1];

    numbers
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn get_seed_from_maps(
    id: u64,
    seed_to_soil_map: &Map,
    soil_to_fertilizer_map: &Map,
    fertilizer_to_water_map: &Map,
    water_to_light_map: &Map,
    light_to_temperature_map: &Map,
    temperature_to_humidity_map: &Map,
    humidity_to_location_map: &Map,
) -> Seed {
    let soil = seed_to_soil_map.get_destination_id_for_source_id(id);
    let fertilizer = soil_to_fertilizer_map.get_destination_id_for_source_id(soil);
    let water = fertilizer_to_water_map.get_destination_id_for_source_id(fertilizer);
    let light = water_to_light_map.get_destination_id_for_source_id(water);
    let temperature = light_to_temperature_map.get_destination_id_for_source_id(light);
    let humidity = temperature_to_humidity_map.get_destination_id_for_source_id(temperature);
    let location = humidity_to_location_map.get_destination_id_for_source_id(humidity);

    Seed {
        id,
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    }
}

fn get_map(name: &str, lines: &[String]) -> Map {
    let ranges: Vec<MapRange> = lines
        .iter()
        .skip_while(|l| !l.contains(name))
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|numbers| MapRange {
            destination_range_start: numbers[0],
            source_range_start: numbers[1],
            range_length: numbers[2],
        })
        .collect();

    Map { ranges }
}

fn get_seed_to_soil_map(lines: &[String]) -> Map {
    get_map("seed-to-soil", lines)
}

fn get_soil_to_fertilizer_map(lines: &[String]) -> Map {
    get_map("soil-to-fertilizer", lines)
}

fn get_fertilizer_to_water_map(lines: &[String]) -> Map {
    get_map("fertilizer-to-water", lines)
}

fn get_water_to_light_map(lines: &[String]) -> Map {
    get_map("water-to-light", lines)
}

fn get_light_to_temperature_map(lines: &[String]) -> Map {
    get_map("light-to-temperature", lines)
}

fn get_temperature_to_humidity_map(lines: &[String]) -> Map {
    get_map("temperature-to-humidity", lines)
}

fn get_humidity_to_location_map(lines: &[String]) -> Map {
    get_map("humidity-to-location", lines)
}

fn part2(lines: Vec<String>) -> String {
    let seed_ids = get_seed_ids_from_ranges(&lines);
    let seed_to_soil_map = get_seed_to_soil_map(&lines);
    let soil_to_fertilizer_map = get_soil_to_fertilizer_map(&lines);
    let fertilizer_to_water_map = get_fertilizer_to_water_map(&lines);
    let water_to_light_map = get_water_to_light_map(&lines);
    let light_to_temperature_map = get_light_to_temperature_map(&lines);
    let temperature_to_humidity_map = get_temperature_to_humidity_map(&lines);
    let humidity_to_location_map = get_humidity_to_location_map(&lines);

    println!("Computing each seed location...");

    seed_ids
        .into_iter()
        .map(|id| {
            get_seed_from_maps(
                id,
                &seed_to_soil_map,
                &soil_to_fertilizer_map,
                &fertilizer_to_water_map,
                &water_to_light_map,
                &light_to_temperature_map,
                &temperature_to_humidity_map,
                &humidity_to_location_map,
            )
        })
        .fold(
            u64::MAX,
            |acc, Seed { location, .. }| {
                if location < acc {
                    location
                } else {
                    acc
                }
            },
        )
        .to_string()
}

fn get_seed_ids_from_ranges(lines: &[String]) -> Vec<u64> {
    let numbers: &str = lines[0].split(": ").collect::<Vec<_>>()[1];
    let numbers = numbers
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect::<Vec<u64>>();

    // get vec capacity to allocate once
    let mut vec_capacity = 0;
    let num = numbers.clone();
    let mut num = num.iter();
    for _ in 0..(num.len() / 2) {
        num.next();
        let range_length = num.next().unwrap();

        vec_capacity += *range_length as usize;
    }

    println!("{vec_capacity} seeds will be processed...");

    let mut result = Vec::with_capacity(vec_capacity);
    let mut numbers = numbers.iter();
    for _ in 0..(numbers.len() / 2) {
        let start_id = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();

        for id in *start_id..(start_id + range_length) {
            result.push(id);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "35");
    }

    #[test]
    fn get_seed_ids_success() {
        let lines = ["seeds: 79 14 55 13", ""]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(get_seed_ids(&lines), vec![79, 14, 55, 13]);
    }

    #[test]
    fn get_seed_to_soil_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_seed_to_soil_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 50,
                        source_range_start: 98,
                        range_length: 2,
                    },
                    MapRange {
                        destination_range_start: 52,
                        source_range_start: 50,
                        range_length: 48,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_soil_to_fertilizer_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_soil_to_fertilizer_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 0,
                        source_range_start: 15,
                        range_length: 37,
                    },
                    MapRange {
                        destination_range_start: 37,
                        source_range_start: 52,
                        range_length: 2,
                    },
                    MapRange {
                        destination_range_start: 39,
                        source_range_start: 0,
                        range_length: 15,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_fertilizer_to_water_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_fertilizer_to_water_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 49,
                        source_range_start: 53,
                        range_length: 8,
                    },
                    MapRange {
                        destination_range_start: 0,
                        source_range_start: 11,
                        range_length: 42,
                    },
                    MapRange {
                        destination_range_start: 42,
                        source_range_start: 0,
                        range_length: 7,
                    },
                    MapRange {
                        destination_range_start: 57,
                        source_range_start: 7,
                        range_length: 4,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_water_to_light_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_water_to_light_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 88,
                        source_range_start: 18,
                        range_length: 7,
                    },
                    MapRange {
                        destination_range_start: 18,
                        source_range_start: 25,
                        range_length: 70,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_light_to_temperature_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_light_to_temperature_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 45,
                        source_range_start: 77,
                        range_length: 23,
                    },
                    MapRange {
                        destination_range_start: 81,
                        source_range_start: 45,
                        range_length: 19,
                    },
                    MapRange {
                        destination_range_start: 68,
                        source_range_start: 64,
                        range_length: 13,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_temperature_to_humidity_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_temperature_to_humidity_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 0,
                        source_range_start: 69,
                        range_length: 1,
                    },
                    MapRange {
                        destination_range_start: 1,
                        source_range_start: 0,
                        range_length: 69,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_humidity_to_location_map_success() {
        let lines = [
            "seeds: 79 14 55 13",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(
            get_humidity_to_location_map(&lines),
            Map {
                ranges: vec![
                    MapRange {
                        destination_range_start: 60,
                        source_range_start: 56,
                        range_length: 37,
                    },
                    MapRange {
                        destination_range_start: 56,
                        source_range_start: 93,
                        range_length: 4,
                    }
                ]
            },
        );
    }

    #[test]
    fn get_destination_id_for_source_id_success() {
        let map = Map {
            ranges: vec![
                MapRange {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                MapRange {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48,
                },
            ],
        };

        assert_eq!(map.get_destination_id_for_source_id(79), 81);
        assert_eq!(map.get_destination_id_for_source_id(14), 14);
        assert_eq!(map.get_destination_id_for_source_id(55), 57);
        assert_eq!(map.get_destination_id_for_source_id(13), 13);
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "46");
    }

    #[test]
    fn get_seed_ids_from_ranges_success() {
        let lines = ["seeds: 79 14 55 13", ""]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(
            get_seed_ids_from_ranges(&lines),
            vec![
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67
            ]
        );
    }
}
