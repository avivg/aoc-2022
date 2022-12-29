fn main() {
    for func in [day23::part1, day23::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day23 {
    use std::collections::HashMap;

    pub fn part1(input: &str) -> u64 {
        let mut elves = ElvesSpread::from(input);
        for _ in 0..10 {
            elves.disperse();
        }
        elves.open_tiles()
    }

    pub fn part2(input: &str) -> u64 {
        let mut elves = ElvesSpread::from(input);
        let mut count = 0;
        while elves.disperse() {
            count += 1;
        }
        count + 1 // they moved for count rounds and 'count + 1' is the first round where they didn't.
    }

    struct ElvesSpread {
        elves: Vec<Coord>,
        dir_iter: Box<dyn Iterator<Item = PossibleDirection>>,
    }

    #[derive(Clone)]
    struct PossibleDirection {
        dir: (isize, isize),
        check: [(isize, isize); 3],
    }

    impl ElvesSpread {
        fn disperse(&mut self) -> bool {
            let mut sugg = vec![];
            let mut dests: HashMap<Coord, usize> = HashMap::new();
            for ec in &self.elves {
                if [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .all(|&delta| self.is_open_tile(ec + delta))
                {
                    // No one around
                    sugg.push(None);
                } else {
                    let mut suggested = false;
                    for _ in 0..4 {
                        let npd = self.dir_iter.next().unwrap();
                        if !suggested
                            && npd.check.iter().all(|&delta| self.is_open_tile(ec + delta))
                        {
                            // Found a direction to suggest
                            let suggestion = ec + npd.dir;
                            sugg.push(Some(suggestion.clone()));
                            dests.entry(suggestion).and_modify(|c| *c += 1).or_insert(1);
                            suggested = true;
                        } // Keep iterating over the directions, so the next elf would start from the beginning of the cycle.
                    }
                    if !suggested {
                        sugg.push(None);
                    }
                }
            }

            self.dir_iter.next(); // So that next time will start with the next direction first

            let mut moved = false;
            for (i, suggestion) in sugg.into_iter().enumerate() {
                if let Some(c) = suggestion {
                    if let Some(1) = dests.get(&c) {
                        self.elves[i] = c;
                        moved = true;
                    }
                }
            }
            moved
        }

        fn open_tiles(&self) -> u64 {
            let mut minx = self.elves[0].x;
            let mut maxx = self.elves[0].x;
            let mut miny = self.elves[0].y;
            let mut maxy = self.elves[0].y;
            for elf in &self.elves {
                minx = minx.min(elf.x);
                maxx = maxx.max(elf.x);
                miny = miny.min(elf.y);
                maxy = maxy.max(elf.y);
            }
            ((maxx - minx + 1) * (maxy - miny + 1)).saturating_sub_unsigned(self.elves.len()) as u64
        }

        fn print(&self) {
            let mut minx = self.elves[0].x;
            let mut maxx = self.elves[0].x;
            let mut miny = self.elves[0].y;
            let mut maxy = self.elves[0].y;
            for elf in &self.elves {
                minx = minx.min(elf.x);
                maxx = maxx.max(elf.x);
                miny = miny.min(elf.y);
                maxy = maxy.max(elf.y);
            }
            let mut field = vec![vec!['.'; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
            for e in self.elves.iter() {
                field[(e.y - miny) as usize][(e.x - minx) as usize] = '#';
            }
            for l in field {
                for c in l {
                    print!("{c}");
                }
                println!()
            }
            println!()
        }

        fn is_open_tile(&self, c: Coord) -> bool {
            self.elves.iter().all(|ec| *ec != c)
        }
    }

    impl From<&str> for ElvesSpread {
        fn from(s: &str) -> Self {
            let mut es = Self {
                elves: vec![],
                dir_iter: Box::new(
                    [
                        PossibleDirection {
                            dir: (0, -1), // N
                            check: [(-1, -1), (0, -1), (1, -1)],
                        },
                        PossibleDirection {
                            dir: (0, 1), // S
                            check: [(-1, 1), (0, 1), (1, 1)],
                        },
                        PossibleDirection {
                            dir: (-1, 0), // W
                            check: [(-1, -1), (-1, 0), (-1, 1)],
                        },
                        PossibleDirection {
                            dir: (1, 0), // E
                            check: [(1, -1), (1, 0), (1, 1)],
                        },
                    ]
                    .into_iter()
                    .cycle(),
                ),
            };
            for (y, l) in s.lines().enumerate() {
                for (x, c) in l.chars().enumerate() {
                    if c == '#' {
                        es.elves.push(Coord {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                }
            }
            es
        }
    }

    #[derive(Eq, Hash, PartialEq, Clone, Debug)]
    struct Coord {
        x: isize,
        y: isize,
    }

    impl std::ops::Add<(isize, isize)> for &Coord {
        type Output = Coord;

        fn add(self, rhs: (isize, isize)) -> Self::Output {
            Self::Output {
                x: self.x + rhs.0,
                y: self.y + rhs.1,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::ElvesSpread;

        #[test]
        fn tight_four() {
            let mut elves = ElvesSpread::from("##\n##");
            elves.disperse();
            assert_eq!(4, elves.open_tiles());
        }

        #[test]
        fn tight_five() {
            // .#.
            // ###
            // .#.
            let mut elves = ElvesSpread::from(".#.\n###\n.#.");
            elves.disperse();
            // ..#..
            // .....
            // #.#.#
            // .....
            // ..#..
            assert_eq!(20, elves.open_tiles());
        }

        #[test]
        fn example_part1() {
            assert_eq!(110, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(4056, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(20, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(0, super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    }
}

const INPUT: &str = include_str!("input.txt");
