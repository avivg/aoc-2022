fn main() {
    for func in [day14::part1, day14::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day14 {
    pub fn part1(input: &str) -> u64 {
        let mut cave = Cave::from(input);
        cave.print();
        let mut sand_count = 0;
        loop {
            match cave.drop(500, 0) {
                DropResult::Rest => sand_count += 1,
                DropResult::Floor => break,
                DropResult::Clogged => unreachable!(),
            }
            cave.print();
        }
        sand_count
    }

    pub fn part2(input: &str) -> u64 {
        let mut cave = Cave::from(input);
        cave.print();
        let mut sand_count = 0;
        loop {
            match cave.drop(500, 0) {
                DropResult::Clogged => break,
                _ => sand_count += 1,
            }
            cave.print();
        }
        sand_count
    }

    enum DropResult {
        Rest,
        Floor,
        Clogged,
    }

    struct Cave {
        grid: Vec<Vec<bool>>,
        minx: usize,
    }

    impl Cave {
        fn drop(&mut self, x: usize, y: usize) -> DropResult {
            const X_PRIO: [isize; 3] = [0, -1, 1];
            let mut cur = Coord { x, y };
            if !self.empty(&cur) {
                return DropResult::Clogged;
            }
            'dropping: loop {
                for adv in X_PRIO {
                    let next = Coord {
                        x: (cur.x as isize + adv) as usize,
                        y: cur.y + 1,
                    };
                    if self.empty(&next) {
                        cur = next;
                        continue 'dropping;
                    }
                }
                // no where to go
                self.fill(&cur);
                if cur.y == self.grid.len() - 2 {
                    return DropResult::Floor;
                }
                return DropResult::Rest;
            }
        }

        fn empty(&self, coord: &Coord) -> bool {
            !self.grid[coord.y][coord.x - self.minx]
        }

        fn fill(&mut self, coord: &Coord) {
            let actualx = coord.x - self.minx;
            self.grid[coord.y][actualx] = true;
        }

        fn print(&self) {
            // For debugging - uncomment
            // for (i, l) in self.grid.iter().enumerate() {
            //     print!("{i:2} ");
            //     for b in l {
            //         if *b {
            //             print!("#");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            // println!();
        }
    }

    struct Coord {
        x: usize,
        y: usize,
    }

    impl From<&str> for Coord {
        fn from(s: &str) -> Self {
            let mut parts = s.split(',').map(|ps| ps.parse::<usize>().unwrap());
            Self {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
            }
        }
    }

    // cave parsing
    impl From<&str> for Cave {
        fn from(input: &str) -> Self {
            use std::cmp::{max, min};

            let mut ranges: Vec<(Coord, Coord)> = vec![];
            for l in input.lines() {
                let vertices = l.split(" -> ");
                let ends = vertices.clone().skip(1);
                for (start, end) in vertices.zip(ends) {
                    ranges.push((start.into(), end.into()));
                }
            }
            let maxy = ranges.iter().map(|r| max(r.0.y, r.1.y)).max().unwrap();
            let minx = 500 - maxy - 2;
            let maxx = 500 + maxy + 2;

            let mut cave = Self {
                grid: vec![vec![false; maxx - minx + 1]; maxy + 2],
                minx,
            };
            cave.grid.push(vec![true; maxx - minx + 1]);

            for r in ranges {
                let startx = min(r.0.x, r.1.x);
                let endx = max(r.0.x, r.1.x);
                let starty = min(r.0.y, r.1.y);
                let endy = max(r.0.y, r.1.y);
                for x in startx..=endx {
                    for y in starty..=endy {
                        cave.fill(&Coord { x, y });
                    }
                }
            }

            cave
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn real_part1() {
            assert_eq!(1001, super::part1(crate::INPUT));
        }

        #[test]
        fn real_part2() {
            assert_eq!(27976, super::part2(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = "\
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9";
