extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    for func in [day15::part1, day15::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day15 {
    pub fn part1(input: &str) -> u64 {
        // sensor at s = (sx, sy) receiving beacon at b = (bx, by) has exclusion radius r = dist(s,b).
        // row y has distance from sensor at s of d = row_dist(y, s) = |y - sy|
        // if d <= r, then let diff = r - d, and the sensor exclude [sx - diff, sx + diff] from row y

        let sensors = parse_sensors(input);
        let mut detectable = collect_detectable_ranges(2_000_000, &sensors, true);

        detectable.count() as u64
    }

    pub fn part2(input: &str) -> u64 {
        const LIMIT: isize = 4_000_000;
        let sensors = parse_sensors(input);
        for y in 0..=LIMIT {
            let mut detectable = collect_detectable_ranges(y, &sensors, false);
            if let Some(x) = detectable.first_free(0, LIMIT) {
                return (x * LIMIT + y) as u64;
            }
        }
        0
    }

    fn parse_sensors(input: &str) -> Vec<Sensor> {
        input.lines().map(Sensor::from).collect()
    }

    fn collect_detectable_ranges(y: isize, sensors: &Vec<Sensor>, exclude_devices: bool) -> Ranges {
        let mut ranges = Ranges::new();
        if exclude_devices {
            for s in sensors.iter().filter(|s| s.beacon.y == y) {
                ranges.set_excluded(s.beacon.x);
            }
        }
        for s in sensors {
            let r = s.radius();
            let d = s.dist(y);
            if d <= r {
                let diff = r - d;
                ranges.add_range(s.coord.x - diff, s.coord.x + diff);
            }
        }
        ranges
    }

    #[derive(Debug)]
    struct Sensor {
        coord: Coord,
        beacon: Coord,
    }

    impl Sensor {
        fn radius(&self) -> isize {
            self.coord.dist(&self.beacon)
        }

        fn dist(&self, row: isize) -> isize {
            self.coord.dist(&Coord {
                y: row,
                ..self.coord
            })
        }
    }

    impl From<&str> for Sensor {
        fn from(line: &str) -> Self {
            use pest::Parser;
            let mut parser = SensorParser::parse(Rule::parts, line).expect("Parse failed");
            let mut nums = parser.next().unwrap().into_inner();
            let sx = nums.next().unwrap().as_str().parse::<isize>().unwrap();
            let sy = nums.next().unwrap().as_str().parse::<isize>().unwrap();
            let bx = nums.next().unwrap().as_str().parse::<isize>().unwrap();
            let by = nums.next().unwrap().as_str().parse::<isize>().unwrap();
            Self {
                coord: Coord { x: sx, y: sy },
                beacon: Coord { x: bx, y: by },
            }
        }
    }

    #[derive(Parser)]
    #[grammar_inline = r#"
        parts = { "Sensor at x=" ~ num ~ ", y=" ~ num ~ ": closest beacon is at x=" ~ num ~ ", y=" ~ num }
        num = { "-"? ~ ASCII_DIGIT+ }"#]
    struct SensorParser;

    #[derive(Debug)]
    struct Coord {
        x: isize,
        y: isize,
    }

    impl Coord {
        fn dist(&self, other: &Coord) -> isize {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }

    #[derive(Debug)]
    struct Rng(isize, isize);

    #[derive(Debug)]
    struct Ranges {
        rs: Vec<Rng>,
        excluded: Vec<isize>,
    }

    impl Ranges {
        fn new() -> Self {
            Self {
                rs: vec![],
                excluded: vec![],
            }
        }

        // exclude e from all the ranges (by splitting ranges that contain them into 2 if e is in the middle)
        fn set_excluded(&mut self, e: isize) {
            self.excluded.push(e);
        }

        fn add_range(&mut self, start: isize, end: isize) {
            let start = if self.excluded.contains(&start) {
                start + 1
            } else {
                start
            };
            let end = if self.excluded.contains(&end) {
                end - 1
            } else {
                end
            };
            if end - start >= 0 {
                let mut added = false;
                for e in self.excluded.clone() {
                    if start < e && end > e {
                        self.add_range(start, e - 1);
                        self.add_range(e + 1, end);
                        added = true;
                    }
                }
                if !added {
                    self.rs.push(Rng(start, end));
                }
            }
        }

        fn count(&mut self) -> usize {
            self.rs.sort_by_key(|r| r.0);
            let mut res = 0;
            if !self.rs.is_empty() {
                let mut cur = self.rs[0].0;
                for rng in &self.rs {
                    if rng.0 > cur {
                        cur = rng.0;
                        res += 1;
                    }
                    if rng.1 >= cur {
                        let added = rng.1 - cur;
                        res += added;
                        cur = rng.1;
                    }
                }
            }
            res as usize + 1 // the first element isn't counted
        }

        fn first_free(&mut self, start: isize, end: isize) -> Option<isize> {
            self.rs.sort_by_key(|r| r.0);
            let mut rng = self.rs.iter();
            let mut cur = start; // points to the next position that may not be detectable (outside the processed ranges)
            for r in &mut rng {
                if r.1 <= cur {
                    continue;
                }
                if r.0 > cur {
                    return Some(cur);
                }
                if r.1 > cur {
                    cur = r.1 + 1;
                }
                if cur > end {
                    break;
                }
            }
            if cur <= end {
                return Some(cur);
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn real_part1() {
            assert_eq!(5607466, super::part1(crate::INPUT));
        }
        #[test]
        fn real_part2() {
            assert_eq!(12543202766584, super::part2(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
