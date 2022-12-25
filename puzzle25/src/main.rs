fn main() {
    for func in [day25::part1, day25::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day25 {
    pub fn part1(input: &str) -> String {
        let sum_dec = input.lines().map(Snafu::to_dec).sum::<i64>();
        dbg!(sum_dec);
        Snafu::from_dec(sum_dec)
    }

    pub fn part2(input: &str) -> String {
        todo!()
    }

    struct Snafu;

    impl Snafu {
        fn to_dec(snafu: &str) -> i64 {
            let mut num = 0;
            for c in snafu.chars() {
                num = 5 * num
                    + match c {
                        '0'..='2' => (c as u8 - '0' as u8) as i64,
                        '-' => -1,
                        '=' => -2,
                        _ => unreachable!(),
                    }
            }
            num
        }

        fn from_dec(mut num: i64) -> String {
            let mut bytes: Vec<char> = vec![];
            while num > 0 {
                let rem = num % 5;
                num /= 5;
                let b = match rem {
                    0..=2 => ('0' as u8 + rem as u8) as char,
                    3 => {
                        num += 1;
                        '='
                    }
                    4 => {
                        num += 1;
                        '-'
                    }
                    _ => unreachable!(),
                };
                bytes.push(b);
            }
            String::from_iter(bytes.iter().rev())
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!("2=-1=0", super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!("2-02===-21---2002==0", super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            // assert_eq!("", super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            // assert_eq!("", super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    }
}

const INPUT: &str = include_str!("input.txt");
