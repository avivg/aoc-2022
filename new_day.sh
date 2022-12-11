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
}

const INPUT: &str = include_str!(\"input.txt\");
" > $name/src/main.rs

code $name/src/main.rs
code $name/src/input.txt
