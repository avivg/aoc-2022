fn main() {
    for func in [day24::part1, day24::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day24 {
    use std::{collections::HashSet, hash::Hash};

    pub fn part1(input: &str) -> u64 {
        let mut basin = Basin::from(input);
        basin.set_start(0, 1);
        let mut minutes = 0;
        while !basin.is_reachable(basin.height() - 1, basin.width() - 2) {
            minutes += 1;
            basin.advence_minute();
        }
        minutes
    }

    pub fn part2(input: &str) -> u64 {
        let mut basin = Basin::from(input);
        basin.set_start(0, 1);
        let mut minutes = 0;
        while !basin.is_reachable(basin.height() - 1, basin.width() - 2) {
            minutes += 1;
            basin.advence_minute();
        }
        basin.set_start(basin.height() - 1, basin.width() - 2);
        while !basin.is_reachable(0, 1) {
            minutes += 1;
            basin.advence_minute();
        }
        basin.set_start(0, 1);
        while !basin.is_reachable(basin.height() - 1, basin.width() - 2) {
            minutes += 1;
            basin.advence_minute();
        }
        minutes
    }

    struct Basin {
        blizzards: Vec<Blizzard>,
        reachable: HashSet<Coord>,
        tiles: Vec<Vec<Tile>>,
    }

    impl From<&str> for Basin {
        fn from(s: &str) -> Self {
            let mut basin = Self {
                blizzards: vec![],
                reachable: HashSet::new(),
                tiles: vec![],
            };

            for (r, l) in s.lines().enumerate() {
                let mut row = vec![];
                for (c, b) in l.bytes().enumerate() {
                    match b as char {
                        '#' => row.push(Tile::Wall),
                        '.' => row.push(Tile::Empty),
                        b => {
                            row.push(Tile::Blizzard);
                            basin.blizzards.push(Blizzard::new(r, c, b));
                        }
                    }
                }
                basin.tiles.push(row);
            }

            basin
        }
    }

    impl Basin {
        pub fn set_start(&mut self, row: usize, col: usize) {
            self.reachable.clear();
            self.reachable.insert(Coord { row, col });
        }

        pub fn is_reachable(&self, row: usize, col: usize) -> bool {
            self.reachable.get(&Coord { row, col }).is_some()
        }

        pub fn height(&self) -> usize {
            self.tiles.len()
        }

        pub fn width(&self) -> usize {
            self.tiles[0].len()
        }

        pub fn advence_minute(&mut self) {
            self.advance_tiles();
            self.advance_reachable();
        }

        fn advance_tiles(&mut self) {
            self.tiles = self
                .tiles
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|t| match t {
                            Tile::Wall => Tile::Wall,
                            _ => Tile::Empty,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            for blz in &mut self.blizzards {
                let blz_coord = blz.next_coord(self.tiles.len(), self.tiles[0].len());
                self.tiles[blz_coord.row][blz_coord.col] = Tile::Blizzard;
            }
        }

        fn advance_reachable(&mut self) {
            let mut next_reachable = HashSet::new();
            for c in &self.reachable {
                for nc in self.next_tiles(c) {
                    if self.is_empty(&nc) {
                        next_reachable.insert(nc);
                    }
                }
            }
            self.reachable = next_reachable;
        }

        fn is_empty(&self, coord: &Coord) -> bool {
            matches!(self.tiles[coord.row][coord.col], Tile::Empty)
        }

        fn next_tiles(&self, c: &Coord) -> impl Iterator<Item = Coord> {
            let mut ncs = vec![]; // Could be a HashSet but in these small numbers, vec is faster despite the possible redundancy
            for delta in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nc = Coord {
                    row: c.row.saturating_add_signed(delta.0),
                    col: c.col.saturating_add_signed(delta.1),
                };
                if nc.row < self.height() && nc.col < self.width() {
                    ncs.push(nc);
                }
            }
            ncs.into_iter()
        }
    }

    struct Blizzard {
        row: usize,
        col: usize,
        dir: (isize, isize),
    }

    impl Blizzard {
        fn new(row: usize, col: usize, dir: char) -> Self {
            Self {
                row,
                col,
                dir: match dir {
                    '>' => (0, 1),
                    '<' => (0, -1),
                    '^' => (-1, 0),
                    'v' => (1, 0),
                    _ => unreachable!(),
                },
            }
        }

        fn next_coord(&mut self, height: usize, width: usize) -> Coord {
            self.row = self.row.checked_add_signed(self.dir.0).unwrap();
            self.col = self.col.checked_add_signed(self.dir.1).unwrap();
            if self.row == 0 {
                self.row = height - 2;
            }
            if self.row == height - 1 {
                self.row = 1;
            }
            if self.col == 0 {
                self.col = width - 2;
            }
            if self.col == width - 1 {
                self.col = 1;
            }

            Coord {
                row: self.row,
                col: self.col,
            }
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Coord {
        row: usize,
        col: usize,
    }

    #[derive(Clone)]
    enum Tile {
        Wall,
        Blizzard,
        Empty,
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!(18, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(225, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(54, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(711, super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    }
}

const INPUT: &str = include_str!("input.txt");
