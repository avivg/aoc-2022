fn main() {
    for f in [nine::part1, nine::part2] {
        let start = std::time::Instant::now();
        let res = f(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

// const INPUT: &str = "\
// R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2";
const INPUT: &str = include_str!("input.txt");

mod nine {
    use std::collections::HashSet;

    pub fn part1(input: &str) -> usize {
        let mut tracker = RopeTracker::new(1);
        for l in input.lines() {
            tracker.next_move(l);
        }
        tracker.tail_locs(0).collect::<HashSet<_>>().len()
    }
    pub fn part2(input: &str) -> usize {
        let mut tracker = RopeTracker::new(9);
        for l in input.lines() {
            tracker.next_move(l);
        }
        tracker.tail_locs(8).collect::<HashSet<_>>().len()
    }

    struct RopeTracker {
        positions: Vec<Vec<(i32, i32)>>,
    }

    impl RopeTracker {
        fn new(tail_len: usize) -> Self {
            Self {
                positions: vec![vec![(0, 0)]; tail_len + 1],
            }
        }

        fn tail_locs(&self, tail_idx: usize) -> impl Iterator<Item = (i32, i32)> {
            self.positions[tail_idx + 1].clone().into_iter()
        }

        fn next_move(&mut self, m: &str) {
            let mut parts = m.split_whitespace();
            let (dir, amount) = (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            );
            for _ in 0..amount {
                self.move_h(dir);
            }
        }

        fn move_h(&mut self, dir: &str) {
            let mut next_h = self.positions[0].last().unwrap().clone();
            match dir {
                "R" => next_h.0 += 1,
                "L" => next_h.0 -= 1,
                "U" => next_h.1 += 1,
                "D" => next_h.1 -= 1,
                _ => unreachable!(),
            };
            self.positions[0].push(next_h);
            for i in 1..self.positions.len() {
                self.snap_tail(i);
            }
        }

        fn snap_tail(&mut self, idx: usize) {
            let prev_pos = self.positions[idx - 1].last().unwrap();
            let cur_pos = self.positions[idx].last().unwrap().clone();
            let dx = prev_pos.0 - cur_pos.0;
            let dy = prev_pos.1 - cur_pos.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                // needs snapping
                let normalized = |n: i32| n.checked_div(n.abs()).unwrap_or_default();
                self.positions[idx].push((cur_pos.0 + normalized(dx), cur_pos.1 + normalized(dy)))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn real_part1() {
            assert_eq!(5735, super::part1(crate::INPUT));
        }
    }
}
