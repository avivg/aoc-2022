fn main() {
    for func in [day17::part1, day17::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day17 {
    mod moves;
    use moves::Moves;

    mod tetris;
    use tetris::{Tetris, StepResult};

    pub fn part1(input: &str) -> u64 {
        let moves = Moves::from(input);
        let mut tetris = Tetris::new(moves);
        let mut resting_pieces = 0;
        while resting_pieces < 2022 {
            if let StepResult::Landed = tetris.step() {
                resting_pieces += 1;
            }
        }
        tetris.top() as u64
    }

    pub fn part2(input: &str) -> u64 {
        let moves = Moves::from(input);
        let mut tetris = Tetris::new(moves);
        let mut resting_pieces = 0;
        while resting_pieces < 1_000_000_000_000u64 {
            if let StepResult::Landed = tetris.step() {
                resting_pieces += 1;
            }
        }
        tetris.top() as u64
    }

    #[cfg(test)]
    mod tests {
        const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        #[test]
        fn example_part1() {
            assert_eq!(3068, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(3114, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(1_514_285_714_288u64, super::part2(EXAMPLE));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
