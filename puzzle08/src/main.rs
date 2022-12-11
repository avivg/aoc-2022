fn main() {
    for foo in [eight::part1, eight::part2] {
        let start = std::time::Instant::now();
        let res = foo(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{} us]", dur as f64 * 10e-3f64);
    }
}

mod eight {
    use std::iter::repeat;

    pub fn part1(input: &str) -> u64 {
        let grove: Vec<Vec<u8>> = read_grove(input);
        let mut visible = vec![vec![false; grove[0].len()]; grove.len()];
        for r in 0..grove.len() {
            set_externaly_visible(&grove, &mut visible, repeat(r).zip(0..grove[0].len()));
            set_externaly_visible(&grove, &mut visible, repeat(r).zip((0..grove[0].len()).rev()));
        }
        for c in 0..grove[0].len() {
            set_externaly_visible(&grove, &mut visible, (0..grove.len()).zip(repeat(c)));
            set_externaly_visible(&grove, &mut visible, (0..grove.len()).rev().zip(repeat(c)));
        }
        visible
            .iter()
            .map(|r| r.iter().filter(|&v| *v).count() as u64)
            .sum()
    }

    pub fn part2(input: &str) -> u64 {
        let grove: Vec<Vec<u8>> = read_grove(input);
        let mut score = vec![vec![1u64; grove[0].len()]; grove.len()];
        for r in 0..grove.len() {
            update_scenic_score(&grove, &mut score, repeat(r).zip(0..grove[0].len()));
            update_scenic_score(&grove, &mut score, repeat(r).zip((0..grove[0].len()).rev()));
        }
        for c in 0..grove[0].len() {
            update_scenic_score(&grove, &mut score, (0..grove.len()).zip(repeat(c)));
            update_scenic_score(&grove, &mut score, (0..grove.len()).rev().zip(repeat(c)));
        }
        *score
            .iter()
            .map(|r| r.iter().max().unwrap())
            .max().unwrap()
    }

    fn set_externaly_visible(grove: &Vec<Vec<u8>>, visibility: &mut Vec<Vec<bool>>, indices: impl Iterator<Item = (usize, usize)>) {
        let mut last_h = -1;
        for (r,c) in indices {
            if grove[r][c] as i8 > last_h {
                visibility[r][c] = true;
                last_h = grove[r][c] as i8;
            }
        }
    }

    fn update_scenic_score(grove: &Vec<Vec<u8>>, score: &mut Vec<Vec<u64>>, indices: impl Iterator<Item = (usize, usize)>) {
        let indices = indices.collect::<Vec<(usize, usize)>>();
        for i in 0..indices.len() {
            let (r,c) = indices[i];
            let mut dist = 0;
            let cur_h = grove[r][c];
            for j in i+1..indices.len() {
                dist += 1;
                let (rr,cc) = indices[j];
                if grove[rr][cc] >=  cur_h {
                    break;
                }
            }
            score[r][c] *= dist;
        }
        // This has lower complexity, yet it takes longer o.O
        // let mut height_steps = [0;10];
        // let mut step = 0;
        // for (r, c) in indices {
        //     let cur_h = grove[r][c];
        //     let mut last_higher_step = 0;
        //     for i in cur_h..10 {
        //         last_higher_step = std::cmp::max(last_higher_step, height_steps[i as usize]);
        //     }
        //     score[r][c] *= step - last_higher_step;
        //     height_steps[cur_h as usize] = step;
        //     step += 1;
        // }
    }

    fn read_grove(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::eight::*;

    #[test]
    fn basic1() {
        let input = "\
111
121
111";
        assert_eq!(9, part1(input));
    }

    #[test]
    fn basic2() {
        let input = "\
222
212
222";
        assert_eq!(8, part1(input));
    }

    #[test]
    fn example() {
        let input = "\
30373
25512
65332
33549
35390";
        assert_eq!(21, part1(input));
    }

    #[test]
    fn real_part1() {
        assert_eq!(1803, part1(crate::INPUT));
    }

    #[test]
    fn real_part2() {
        assert_eq!(268912, part2(crate::INPUT));
    }

}
