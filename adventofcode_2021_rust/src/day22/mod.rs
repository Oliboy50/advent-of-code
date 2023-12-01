use lazy_static::lazy_static;
use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day22][part1] = {}", part1(lines.clone()));
    println!("[day22][part2] = {}", part2(lines));
}

#[derive(Debug)]
struct Grid {
    cubes: Vec<Cube>,
}

impl Grid {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Self {
        let mut cubes = vec![];
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    cubes.push(Cube {
                        x,
                        y,
                        z,
                        active: false,
                    })
                }
            }
        }

        Grid { cubes }
    }

    fn apply_cuboid(&mut self, cuboid: Cuboid) {
        for cube in &mut self.cubes {
            if cube.active != cuboid.active
                && cuboid.x.contains(cube.x)
                && cuboid.y.contains(cube.y)
                && cuboid.z.contains(cube.z)
            {
                cube.active = cuboid.active
            }
        }
    }

    fn count_active_cubes(&self) -> u64 {
        self.cubes
            .iter()
            .fold(0, |acc, c| if c.active { acc + 1 } else { acc })
    }
}

#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    active: bool,
}

#[derive(Debug)]
struct ActiveCuboidRanges {
    applied_active_cuboids: Vec<Cuboid>,
}

impl ActiveCuboidRanges {
    fn new() -> Self {
        ActiveCuboidRanges {
            applied_active_cuboids: vec![],
        }
    }

    fn apply_cuboid(&mut self, cuboid: Cuboid) {
        let mut new_cuboids = vec![];

        for applied_cuboid in self.applied_active_cuboids.clone() {
            for c in applied_cuboid.overlapped_by(cuboid.clone()) {
                new_cuboids.push(c);
            }
        }
        if cuboid.active {
            new_cuboids.push(cuboid);
        }

        self.applied_active_cuboids = dbg!(new_cuboids);
    }

    fn count(&self) -> u64 {
        self.applied_active_cuboids.iter().fold(0, |acc, cuboid| {
            acc + ((1 + (cuboid.x.to - cuboid.x.from))
                * (1 + (cuboid.y.to - cuboid.y.from))
                * (1 + (cuboid.z.to - cuboid.z.from))) as u64
        })
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    active: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl From<String> for Cuboid {
    fn from(cuboid: String) -> Self {
        let cuboid_parts = cuboid
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let activate = match cuboid_parts[0].as_str() {
            "on" => true,
            _ => false,
        };
        lazy_static! {
            static ref RANGE_PARTS_REGEX: Regex = Regex::new(r"x=(?P<xmin>-?\d{1,6})\.\.(?P<xmax>-?\d{1,6}),y=(?P<ymin>-?\d{1,6})\.\.(?P<ymax>-?\d{1,6}),z=(?P<zmin>-?\d{1,6})\.\.(?P<zmax>-?\d{1,6})").unwrap();
        }
        let range_parts = RANGE_PARTS_REGEX
            .captures(cuboid_parts[1].as_str())
            .unwrap();

        Cuboid {
            active: activate,
            x: Range {
                from: range_parts.name("xmin").unwrap().as_str().parse().unwrap(),
                to: range_parts.name("xmax").unwrap().as_str().parse().unwrap(),
            },
            y: Range {
                from: range_parts.name("ymin").unwrap().as_str().parse().unwrap(),
                to: range_parts.name("ymax").unwrap().as_str().parse().unwrap(),
            },
            z: Range {
                from: range_parts.name("zmin").unwrap().as_str().parse().unwrap(),
                to: range_parts.name("zmax").unwrap().as_str().parse().unwrap(),
            },
        }
    }
}

impl Cuboid {
    fn overlaps(&self, cuboid: &Cuboid) -> bool {
        self.x.contains(cuboid.x.from)
            || self.y.contains(cuboid.y.from)
            || self.z.contains(cuboid.z.from)
            || self.x.contains(cuboid.x.to)
            || self.y.contains(cuboid.y.to)
            || self.z.contains(cuboid.z.to)
    }

    fn overlapped_by(self, cuboid: Cuboid) -> Vec<Cuboid> {
        if !self.overlaps(&cuboid) {
            return vec![self];
        }

        let mut cuboids = vec![];

        // @TODO: understand how to split "self" to be correctly overlapped by cuboid
        if self.x.contains(cuboid.x.from)
            && self.y.contains(cuboid.y.from)
            && self.z.contains(cuboid.z.from)
        {
            cuboids.push(Cuboid {
                x: Range {
                    from: self.x.from,
                    to: cuboid.x.from - 1,
                },
                y: Range {
                    from: self.y.from,
                    to: cuboid.y.from - 1,
                },
                z: Range {
                    from: self.z.from,
                    to: cuboid.z.from - 1,
                },
                active: true,
            });
        } else if self.x.contains(cuboid.x.from) {
            cuboids.push(Cuboid {
                x: Range {
                    from: self.x.from,
                    to: cuboid.x.from - 1,
                },
                y: Range {
                    from: self.y.from,
                    to: self.y.to,
                },
                z: Range {
                    from: self.z.from,
                    to: self.z.to,
                },
                active: true,
            });
        } else if self.y.contains(cuboid.y.from) {
            cuboids.push(Cuboid {
                x: Range {
                    from: self.x.from,
                    to: self.x.to,
                },
                y: Range {
                    from: self.y.from,
                    to: cuboid.y.from - 1,
                },
                z: Range {
                    from: self.z.from,
                    to: self.z.to,
                },
                active: true,
            });
        } else if self.z.contains(cuboid.z.from) {
            cuboids.push(Cuboid {
                x: Range {
                    from: self.x.from,
                    to: self.x.to,
                },
                y: Range {
                    from: self.y.from,
                    to: self.y.to,
                },
                z: Range {
                    from: self.z.from,
                    to: cuboid.z.from - 1,
                },
                active: true,
            });
        }

        cuboids
    }
}

#[derive(Debug, Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn contains(&self, num: i32) -> bool {
        (self.from..=self.to).contains(&num)
    }
}

fn part1(lines: Vec<String>) -> u64 {
    let min = -50;
    let max = 50;
    let cuboids: Vec<Cuboid> = lines
        .into_iter()
        .map(Cuboid::from)
        .filter(|cuboid| {
            cuboid.x.from >= min
                || cuboid.y.from >= min
                || cuboid.z.from >= min
                || cuboid.x.to <= max
                || cuboid.y.to <= max
                || cuboid.z.to <= max
        })
        .collect();

    let mut grid = Grid::new(min, max, min, max, min, max);

    for cuboid in cuboids {
        grid.apply_cuboid(cuboid);
    }

    grid.count_active_cubes()

    // @TODO: uncomment when fixed
    // let mut active_cuboids = ActiveCuboidRanges::new();
    //
    // for cuboid in cuboids {
    //     active_cuboids.apply_cuboid(cuboid);
    // }
    //
    // active_cuboids.count()
}

fn part2(lines: Vec<String>) -> u64 {
    let cuboids: Vec<Cuboid> = lines.into_iter().map(Cuboid::from).collect();

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_z = 0;
    let mut max_z = 0;

    for cuboid in &cuboids {
        if cuboid.x.from < min_x {
            min_x = cuboid.x.from
        }
        if cuboid.y.from < min_y {
            min_y = cuboid.y.from
        }
        if cuboid.z.from < min_z {
            min_z = cuboid.z.from
        }
        if cuboid.x.to > max_x {
            max_x = cuboid.x.to
        }
        if cuboid.y.to > max_y {
            max_y = cuboid.y.to
        }
        if cuboid.z.to > max_z {
            max_z = cuboid.z.to
        }
    }

    // @TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_part1_0_0() {
        assert_eq!(
            27,
            part1(
                vec!["on x=10..12,y=10..12,z=10..12",]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_1() {
        assert_eq!(
            46,
            part1(
                vec![
                    "on x=10..12,y=10..12,z=10..12",
                    "on x=11..13,y=11..13,z=11..13",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_2() {
        assert_eq!(
            46,
            part1(
                vec![
                    "on x=10..12,y=10..12,z=10..12",
                    "on x=9..11,y=9..11,z=9..11",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_3() {
        assert_eq!(
            64,
            part1(
                vec![
                    "on x=10..13,y=10..13,z=10..13",
                    "on x=11..12,y=11..12,z=11..12",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_4() {
        assert_eq!(
            72,
            part1(
                vec![
                    "on x=10..13,y=10..13,z=10..13",
                    "on x=11..12,y=9..14,z=11..12",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_5() {
        assert_eq!(
            36,
            part1(
                vec![
                    "on x=10..12,y=10..12,z=10..12",
                    "on x=11..13,y=10..12,z=10..12",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_0_6() {
        assert_eq!(
            46,
            part1(
                vec![
                    "on x=10..12,y=10..12,z=10..12",
                    "on x=11..13,y=11..13,z=11..13",
                    "on x=10..10,y=10..10,z=10..10",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_1() {
        assert_eq!(
            39,
            part1(
                vec![
                    "on x=10..12,y=10..12,z=10..12",
                    "on x=11..13,y=11..13,z=11..13",
                    "off x=9..11,y=9..11,z=9..11",
                    "on x=10..10,y=10..10,z=10..10",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_2() {
        assert_eq!(
            590784,
            part1(
                vec![
                    "on x=-20..26,y=-36..17,z=-47..7",
                    "on x=-20..33,y=-21..23,z=-26..28",
                    "on x=-22..28,y=-29..23,z=-38..16",
                    "on x=-46..7,y=-6..46,z=-50..-1",
                    "on x=-49..1,y=-3..46,z=-24..28",
                    "on x=2..47,y=-22..22,z=-23..27",
                    "on x=-27..23,y=-28..26,z=-21..29",
                    "on x=-39..5,y=-6..47,z=-3..44",
                    "on x=-30..21,y=-8..43,z=-13..34",
                    "on x=-22..26,y=-27..20,z=-29..19",
                    "off x=-48..-32,y=26..41,z=-47..-37",
                    "on x=-12..35,y=6..50,z=-50..-2",
                    "off x=-48..-32,y=-32..-16,z=-15..-5",
                    "on x=-18..26,y=-33..15,z=-7..46",
                    "off x=-40..-22,y=-38..-28,z=23..41",
                    "on x=-16..35,y=-41..10,z=-47..6",
                    "off x=-32..-23,y=11..30,z=-14..3",
                    "on x=-49..-5,y=-3..45,z=-29..18",
                    "off x=18..30,y=-20..-8,z=-3..13",
                    "on x=-41..9,y=-7..43,z=-33..15",
                    "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
                    "on x=967..23432,y=45373..81175,z=27513..53682",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part1_3() {
        assert_eq!(
            542711,
            part1(
                vec![
                    "on x=-3..43,y=-22..22,z=-15..34",
                    "on x=-9..42,y=-40..12,z=-30..19",
                    "on x=-5..46,y=-9..38,z=-31..21",
                    "on x=-27..25,y=-30..22,z=-9..42",
                    "on x=-25..27,y=-43..3,z=-22..31",
                    "on x=-39..9,y=-45..8,z=-17..29",
                    "on x=-10..36,y=-2..44,z=-9..35",
                    "on x=-45..4,y=-41..5,z=-3..49",
                    "on x=-24..26,y=-34..12,z=-6..44",
                    "on x=-35..17,y=-36..11,z=-42..10",
                    "off x=5..19,y=6..21,z=-21..-3",
                    "on x=-45..1,y=-40..11,z=-31..20",
                    "off x=16..30,y=-16..-4,z=21..33",
                    "on x=-38..7,y=-30..15,z=-46..3",
                    "off x=36..48,y=-39..-22,z=-47..-34",
                    "on x=-23..22,y=-22..29,z=-27..24",
                    "off x=-39..-29,y=-47..-34,z=-13..-1",
                    "on x=-18..28,y=-3..44,z=-8..38",
                    "off x=-38..-24,y=-48..-39,z=14..28",
                    "on x=-32..21,y=-36..9,z=-38..16",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn try_part2_1() {
        assert_eq!(
            2758514936282235,
            part2(
                vec![
                    "on x=-5..47,y=-31..22,z=-19..33",
                    "on x=-44..5,y=-27..21,z=-14..35",
                    "on x=-49..-1,y=-11..42,z=-10..38",
                    "on x=-20..34,y=-40..6,z=-44..1",
                    "off x=26..39,y=40..50,z=-2..11",
                    "on x=-41..5,y=-41..6,z=-36..8",
                    "off x=-43..-33,y=-45..-28,z=7..25",
                    "on x=-33..15,y=-32..19,z=-34..11",
                    "off x=35..47,y=-46..-34,z=-11..5",
                    "on x=-14..36,y=-6..44,z=-16..29",
                    "on x=-57795..-6158,y=29564..72030,z=20435..90618",
                    "on x=36731..105352,y=-21140..28532,z=16094..90401",
                    "on x=30999..107136,y=-53464..15513,z=8553..71215",
                    "on x=13528..83982,y=-99403..-27377,z=-24141..23996",
                    "on x=-72682..-12347,y=18159..111354,z=7391..80950",
                    "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709",
                    "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856",
                    "on x=-52752..22273,y=-49450..9096,z=54442..119054",
                    "on x=-29982..40483,y=-108474..-28371,z=-24328..38471",
                    "on x=-4958..62750,y=40422..118853,z=-7672..65583",
                    "on x=55694..108686,y=-43367..46958,z=-26781..48729",
                    "on x=-98497..-18186,y=-63569..3412,z=1232..88485",
                    "on x=-726..56291,y=-62629..13224,z=18033..85226",
                    "on x=-110886..-34664,y=-81338..-8658,z=8914..63723",
                    "on x=-55829..24974,y=-16897..54165,z=-121762..-28058",
                    "on x=-65152..-11147,y=22489..91432,z=-58782..1780",
                    "on x=-120100..-32970,y=-46592..27473,z=-11695..61039",
                    "on x=-18631..37533,y=-124565..-50804,z=-35667..28308",
                    "on x=-57817..18248,y=49321..117703,z=5745..55881",
                    "on x=14781..98692,y=-1341..70827,z=15753..70151",
                    "on x=-34419..55919,y=-19626..40991,z=39015..114138",
                    "on x=-60785..11593,y=-56135..2999,z=-95368..-26915",
                    "on x=-32178..58085,y=17647..101866,z=-91405..-8878",
                    "on x=-53655..12091,y=50097..105568,z=-75335..-4862",
                    "on x=-111166..-40997,y=-71714..2688,z=5609..50954",
                    "on x=-16602..70118,y=-98693..-44401,z=5197..76897",
                    "on x=16383..101554,y=4615..83635,z=-44907..18747",
                    "off x=-95822..-15171,y=-19987..48940,z=10804..104439",
                    "on x=-89813..-14614,y=16069..88491,z=-3297..45228",
                    "on x=41075..99376,y=-20427..49978,z=-52012..13762",
                    "on x=-21330..50085,y=-17944..62733,z=-112280..-30197",
                    "on x=-16478..35915,y=36008..118594,z=-7885..47086",
                    "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456",
                    "off x=2032..69770,y=-71013..4824,z=7471..94418",
                    "on x=43670..120875,y=-42068..12382,z=-24787..38892",
                    "off x=37514..111226,y=-45862..25743,z=-16714..54663",
                    "off x=25699..97951,y=-30668..59918,z=-15349..69697",
                    "off x=-44271..17935,y=-9516..60759,z=49131..112598",
                    "on x=-61695..-5813,y=40978..94975,z=8655..80240",
                    "off x=-101086..-9439,y=-7088..67543,z=33935..83858",
                    "off x=18020..114017,y=-48931..32606,z=21474..89843",
                    "off x=-77139..10506,y=-89994..-18797,z=-80..59318",
                    "off x=8476..79288,y=-75520..11602,z=-96624..-24783",
                    "on x=-47488..-1262,y=24338..100707,z=16292..72967",
                    "off x=-84341..13987,y=2429..92914,z=-90671..-1318",
                    "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188",
                    "off x=-27365..46395,y=31009..98017,z=15428..76570",
                    "off x=-70369..-16548,y=22648..78696,z=-1892..86821",
                    "on x=-53470..21291,y=-120233..-33476,z=-44150..38147",
                    "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
            )
        );
    }
}
