name=puzzle$1
cargo new $name
touch $name/src/input.txt
echo "\
fn main() {
    for func in [day$1::part1, day$1::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!(\"{res} [{dur} ns]\");
    }
}

mod day$1 {
    pub fn part1(input: &str) -> u64 {
        todo!()
    }

    pub fn part2(input: &str) -> u64 {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!(0, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(0, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(0, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(0, super::part2(crate::INPUT));
        }
        const EXAMPLE: &str = \"\\
\";
    }
}

const INPUT: &str = include_str!(\"input.txt\");
" > $name/src/main.rs

code $name/
code $name/src/main.rs
code $name/src/input.txt
