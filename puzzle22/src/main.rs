extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    for func in [day22::part1, day22::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day22 {
    use monkeymap::Map as MonkeyMap;
    use monkeymap::Trace as MonkeyTrace;

    mod monkeymap {
        pub struct Map {
            rows: Vec<CyclicRange>,
            cols: Vec<CyclicRange>,
            blocked: Vec<Vec<bool>>,
        }

        impl From<&str> for Map {
            fn from(value: &str) -> Self {
                let mut map = Self {
                    rows: vec![],
                    cols: vec![],
                    blocked: vec![],
                };
                for l in value.lines() {
                    if l.is_empty() {
                        break;
                    }
                    map.blocked.push(
                        l.chars()
                            .map(|c| match c {
                                ' ' | '.' => false,
                                _ => true,
                            })
                            .collect(),
                    );
                    let start = l.find(|c| c == '.' || c == '#').unwrap();
                    map.rows.push(CyclicRange {
                        start,
                        size: l.len() - start,
                    })
                }

                let max_col = map
                    .rows
                    .iter()
                    .map(|cr| cr.start + cr.size - 1)
                    .max()
                    .unwrap();
                map.cols = vec![CyclicRange { start: 0, size: 0 }; max_col + 1];
                for (i, r) in map.rows.iter().enumerate() {
                    for c in r.start..(r.start + r.size) {
                        if map.cols[c].size == 0 {
                            // new column
                            map.cols[c].start = i;
                            map.cols[c].size = 1;
                        } else {
                            // column initialized
                            map.cols[c].size += 1;
                        }
                    }
                }

                map
            }
        }

        impl Map {
            pub fn height(&self) -> usize {
                self.rows.len()
            }

            pub fn trace(&self, trace: &Trace) -> Pos {
                let mut pos = self.initial_pos();
                for movement in trace {
                    pos = match dbg!(movement) {
                        Movement::Step(n) => self.advance(pos, *n),
                        Movement::RotateCW => Pos {
                            facing: pos.facing.next(),
                            ..pos
                        },
                        Movement::RotateCCW => Pos {
                            facing: pos.facing.prev(),
                            ..pos
                        },
                    }
                }
                pos
            }

            fn initial_pos(&self) -> Pos {
                Pos {
                    facing: Facing(0),
                    row: 0,
                    col: self.rows[0].start,
                }
            }

            fn advance(&self, pos: Pos, steps: usize) -> Pos {
                let mut pos = pos;
                let (dr, dc) = pos.direction();
                for _ in 0..steps {
                    let next_row = self.cols[pos.col].plus(pos.row, dr);
                    let next_col = self.rows[pos.row].plus(pos.col, dc);
                    if self.blocked[next_row][next_col] {
                        break;
                    } else {
                        pos.row = next_row;
                        pos.col = next_col;
                    }
                }
                pos
            }
        }

        #[derive(Clone)]
        struct CyclicRange {
            start: usize,
            size: usize,
        }

        impl CyclicRange {
            // returns n + d inside the range. Assuming n is already inside the range and d is some offset, smaller then the range size.
            fn plus(&self, n: usize, d: isize) -> usize {
                let p = (n - self.start + self.size).checked_add_signed(d).unwrap() % self.size;
                p + self.start
            }
        }

        pub struct Trace(Vec<Movement>);

        impl From<&str> for Trace {
            fn from(value: &str) -> Self {
                let moves = TraceParser::parse(Rule::trace, value)
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner();
                let mut trace = Self(vec![]);
                for p in moves {
                    trace.0.push(match p.as_rule() {
                        Rule::trace => unreachable!(),
                        Rule::num => Movement::Step(p.as_str().parse().unwrap()),
                        Rule::cw => Movement::RotateCW,
                        Rule::ccw => Movement::RotateCCW,
                    });
                }

                trace
            }
        }

        use pest::Parser;
        #[derive(Parser)]
        #[grammar = "src/trace.peg"]
        struct TraceParser;

        impl<'a> IntoIterator for &'a Trace {
            type Item = &'a Movement;

            type IntoIter = std::slice::Iter<'a, Movement>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        #[derive(Debug)]
        pub enum Movement {
            Step(usize),
            RotateCW,
            RotateCCW,
        }

        pub struct Pos {
            pub row: usize,
            pub col: usize,
            pub facing: Facing,
        }

        impl Pos {
            fn direction(&self) -> (isize, isize) {
                match self.facing.0 {
                    0 => (0, 1),
                    1 => (1, 0),
                    2 => (0, -1),
                    3 => (-1, 0),
                    _ => unreachable!(),
                }
            }
        }

        pub struct Facing(u8);

        impl Facing {
            fn next(&self) -> Self {
                Self((self.0 + 1) % 4)
            }

            fn prev(&self) -> Self {
                Self((self.0 + 4 - 1) % 4)
            }

            pub fn val(&self) -> usize {
                self.0 as usize
            }
        }
    }

    pub fn part1(input: &str) -> u64 {
        let mm = MonkeyMap::from(input);
        let trace_line = input.lines().skip(mm.height() + 1).next().unwrap();
        let trace = MonkeyTrace::from(trace_line);
        let pos = mm.trace(&trace);

        (1000 * (pos.row + 1) + 4 * (pos.col + 1) + pos.facing.val()) as u64
    }

    pub fn part2(input: &str) -> u64 {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!(6032, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(80392, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(0, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(0, super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    }
}

const INPUT: &str = include_str!("input.txt");
