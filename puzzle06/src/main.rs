fn main() {
    for foo in [six::part1, six::part2] {
        let start = std::time::Instant::now();
        let res = foo(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

const INPUT: &str = include_str!("input.txt");

mod six {
    pub fn part1(input: &str) -> usize {
        first_distinct(input, 4)
    }

    pub fn part2(input: &str) -> usize {
        first_distinct(input, 14)
    }

    pub fn first_distinct(input: &str, seq_len: usize) -> usize {
        for start in 0..input.len()-seq_len {
            let slice = &input[start..start+seq_len];
            let mut bits = 0u64; // all lowercase - 64b are enough
            slice.chars().for_each(|c| bits |= 1u64 << (c as u8 - 'a' as u8));
            if bits.count_ones() >= seq_len.try_into().unwrap() {
                return start + seq_len;
            }
        }
        0
    }
}