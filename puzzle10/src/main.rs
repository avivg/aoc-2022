fn main() {
    for func in [day10::part1, day10::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day10 {
    pub fn part1(input: &str) -> u64 {
        let cycle_vals = val_per_cycle(input);
        let mut total = 0;
        for i in (19..=219).step_by(40) {
            // println!("{}", cycle_vals[i]);
            total += (cycle_vals[i] * (i as i32 + 1)) as u64
        }
        total
    }

    pub fn part2(input: &str) -> u64 {
        let cycle_vals = val_per_cycle(input);
        let mut crt = vec![];
        for cycle in 0..240 {
            let x = cycle_vals[cycle];
            if [x-1, x, x+1].contains(&((cycle % 40) as i32)) {
                crt.push('#');
            } else {
                crt.push(' ');
            }
        }
        let mut it = crt.iter();
        for _ in 0..6 {
            for _ in 0..8 {
                for _ in 0..5 {
                    print!("{}", it.next().unwrap())
                }
                print!(" ");
            }
            println!();
        }
        0
    }

    fn val_per_cycle(input: &str) -> Vec<i32> {
        let mut cycle_vals = vec![];
        let mut val = 1;
        for l in input.lines() {
            if let Cmd::Addx(num) = Cmd::from(l) {
                // addx num
                cycle_vals.push(val);
                cycle_vals.push(val);
                val += num;
            } else {
                // noop
                cycle_vals.push(val);
            }
        }
        cycle_vals
    }

    enum Cmd {
        Nop,
        Addx(i32),
    }

    impl From<&str> for Cmd {
        fn from(s: &str) -> Self {
            let re = regex::Regex::new(r"(noop|addx\s(-?\d+))").unwrap();
            let caps = re.captures(s).unwrap();
            assert!(caps.get(1).is_some());
            if let Some(num_str) = caps.get(2) {
                Cmd::Addx(num_str.as_str().parse().unwrap())
            } else {
                Cmd::Nop
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::part1;

        #[test]
        fn real_part1() {
            assert_eq!(12640, part1(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");

