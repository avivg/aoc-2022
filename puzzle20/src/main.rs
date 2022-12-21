fn main() {
    for func in [day20::part1, day20::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day20 {
    mod swapvec;
    use swapvec::SwapVec;

    pub fn part1(input: &str) -> i64 {
        let mut sv = SwapVec::new(input.lines().map(|l| l.parse::<i64>().unwrap()));
        mix(&mut sv);
        let mixed: Vec<_> = sv.mixed_iter().take(sv.len()).collect();
        let zero_idx = mixed.iter().enumerate().find(|(_, &v)| v == 0).unwrap().0;
        (1000..=3000)
            .step_by(1000)
            .map(|i| dbg!(mixed[dbg!((zero_idx + i) % mixed.len())]) as i64)
            .sum()
    }

    pub fn part2(input: &str) -> i64 {
        let mut sv = SwapVec::new(
            input
                .lines()
                .map(|l| l.parse::<i64>().unwrap() * 811_589_153i64),
        );
        for _ in 0..10 {
            mix(&mut sv);
        }
        let mixed: Vec<_> = sv.mixed_iter().take(sv.len()).collect();
        let zero_idx = mixed.iter().enumerate().find(|(_, &v)| v == 0).unwrap().0;
        (1000..=3000)
            .step_by(1000)
            .map(|i| dbg!(mixed[dbg!((zero_idx + i) % mixed.len())]) as i64)
            .sum()
    }

    fn mix(sv: &mut SwapVec) {
        let len = sv.len();
        for i in 0..len {
            let val = sv[i];
            if val >= 0 {
                let swaps = val as usize % (len - 1);
                for _ in 0..swaps {
                    sv.swap_with_next(i);
                }
            } else {
                let swaps = val.abs() as usize % (len - 1);
                for _ in 0..swaps {
                    sv.swap_with_prev(i);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4";

        #[test]
        fn example_part1() {
            assert_eq!(3, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(3346, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(1623178306, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(4265712588168, super::part2(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
