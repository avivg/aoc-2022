fn main() {
    for func in [day18::part1, day18::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day18 {
    pub fn part1(input: &str) -> u64 {
        let coords = parse_coords(input);
        let scan = collect_scan(&coords, |_| true);
        let num_seg = count_segments(&scan, |b| *b);
        2 * num_seg
    }

    pub fn part2(input: &str) -> u64 {
        let coords = parse_coords(input);
        let mut scan = collect_scan(&coords, |_| Element::Lava);
        tag_air(&mut scan);
        let num_seg = count_segments(&scan, |e| !e.is_air());
        2 * num_seg
    }

    #[derive(Default, Clone)]
    enum Element {
        #[default]
        Untagged,
        Lava,
        Air,
    }

    impl Element {
        fn tagged(&self) -> bool {
            match self {
                Self::Untagged => false,
                _ => true,
            }
        }

        fn is_air(&self) -> bool {
            match self {
                Self::Air => true,
                _ => false,
            }
        }
    }

    fn parse_coords(input: &str) -> Vec<Coord> {
        let mut coords = vec![];
        for l in input.lines() {
            coords.push(Coord::from(l));
        }
        coords
    }

    #[derive(Clone, Copy)]
    struct Coord {
        x: usize,
        y: usize,
        z: usize,
    }

    impl From<&str> for Coord {
        fn from(s: &str) -> Self {
            let mut parts = s.split(",");
            Self {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        }
    }

    fn collect_scan<T, F>(coords: &Vec<Coord>, f: F) -> Vec<Vec<Vec<T>>>
    where
        T: Default + Clone,
        F: Fn(&Coord) -> T,
    {
        let mut max = coords[0];
        let mut min = coords[0];
        for c in coords {
            max.x = max.x.max(c.x);
            max.y = max.y.max(c.y);
            max.z = max.z.max(c.z);
            min.x = min.x.min(c.x);
            min.y = min.y.min(c.y);
            min.z = min.z.min(c.z);
        }

        // in each dimension, we want the scan to contain 1 extra element before the minimal elemnt and one after
        //
        //    |.####...|
        //    |..#.#...|
        //    |..#####.|
        //  scan    scan
        // start    end
        let xofst = min.x - 1;
        let yofst = min.y - 1;
        let zofst = min.z - 1;
        let xsize = max.x - min.x + 3;
        let ysize = max.y - min.y + 3;
        let zsize = max.z - min.z + 3;

        let mut scan = vec![vec![vec![T::default(); zsize]; ysize]; xsize];
        for c in coords {
            scan[c.x - xofst][c.y - yofst][c.z - zofst] = f(c);
        }
        scan
    }

    fn tag_air(scan: &mut Vec<Vec<Vec<Element>>>) {
        let mut to_visit = vec![Coord { x: 0, y: 0, z: 0 }];
        while !to_visit.is_empty() {
            let c = to_visit.pop().unwrap();
            if !scan[c.x][c.y][c.z].tagged() {
                scan[c.x][c.y][c.z] = Element::Air;
                if c.x > 1 {
                    to_visit.push(Coord { x: c.x - 1, ..c });
                }
                if c.x < scan.len() - 1 {
                    to_visit.push(Coord { x: c.x + 1, ..c });
                }
                if c.y > 1 {
                    to_visit.push(Coord { y: c.y - 1, ..c });
                }
                if c.y < scan[0].len() - 1 {
                    to_visit.push(Coord { y: c.y + 1, ..c });
                }
                if c.z > 1 {
                    to_visit.push(Coord { z: c.z - 1, ..c });
                }
                if c.z < scan[0][0].len() - 1 {
                    to_visit.push(Coord { z: c.z + 1, ..c });
                }
            }
        }
    }

    fn count_segments<T, P>(scan: &Vec<Vec<Vec<T>>>, pred: P) -> u64
    where
        P: Fn(&T) -> bool,
    {
        let xlen = scan.len();
        let ylen = scan[0].len();
        let zlen = scan[0][0].len();

        let mut num_seg = 0;
        let mut in_seg = false;
        for x in 0..(xlen - 1) {
            for y in 0..(ylen - 1) {
                for z in 0..zlen {
                    if pred(&scan[x][y][z]) {
                        in_seg = true;
                    } else if in_seg {
                        num_seg += 1;
                        in_seg = false;
                    }
                }
            }
        }
        for x in 0..(xlen - 1) {
            for z in 0..(zlen - 1) {
                for y in 0..ylen {
                    if pred(&scan[x][y][z]) {
                        in_seg = true;
                    } else if in_seg {
                        num_seg += 1;
                        in_seg = false;
                    }
                }
            }
        }
        for z in 0..(zlen - 1) {
            for y in 0..(ylen - 1) {
                for x in 0..xlen {
                    if pred(&scan[x][y][z]) {
                        in_seg = true;
                    } else if in_seg {
                        num_seg += 1;
                        in_seg = false;
                    }
                }
            }
        }
        num_seg
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!(64, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(4604, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(58, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(2604, super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    }
}

const INPUT: &str = include_str!("input.txt");
