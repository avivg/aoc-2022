fn main() {
    for func in [four::part1, four::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod four {

    pub fn part1(input: &str) -> usize {
        range_pairs(input)
            .filter(|(r1, r2)| r1.completely_overlap(r2))
            .count()
    }

    pub fn part2(input: &str) -> usize {
        range_pairs(input).filter(|(r1, r2)| r1.overlap(r2)).count()
    }

    fn range_pairs<'a>(input: &'a str) -> impl Iterator<Item = (RangeBits, RangeBits)> + 'a {
        input
            .lines()
            .map(|l| l.split_at(l.find(",").unwrap()))
            .map(|(r1, r2)| (RangeBits::from(r1), RangeBits::from(&r2[1..]))) // r2 contains the ','
    }

    struct RangeBits(u128);

    impl RangeBits {
        fn from(range_str: &str) -> Self {
            let (start, end): (&str, &str) = range_str.split_at(range_str.find("-").unwrap());
            let (start, end): (usize, usize) =
                (start.parse().unwrap(), (&end[1..]).parse().unwrap()); // end contains the '-'
            Self((1u128 << (end + 1)) - (1u128 << start))
        }

        fn completely_overlap(&self, other: &RangeBits) -> bool {
            (self.0 | other.0) == self.0 || (self.0 | other.0) == other.0
        }

        fn overlap(&self, other: &RangeBits) -> bool {
            self.0 & other.0 != 0
        }
    }
}

const INPUT: &str = include_str!("input.txt");
