use std::{collections::HashMap, hash::Hash};

const INPUT: &str = "\
A Y
B X
C Z";

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn decode(c: &str) -> Result<Hand, &str> {
        match c {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err("Error parsing"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Score {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Score {
    fn decode(c: &str) -> Result<Score, &str> {
        match c {
            "X" => Ok(Score::Lose),
            "Y" => Ok(Score::Draw),
            "Z" => Ok(Score::Win),
            _ => Err("Error parsing"),
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Hand,
    score: Score,
}

impl Round {
    fn from(s: &str) -> Self {
        let mut hands = s.split_whitespace();
        Self {
            opponent: Hand::decode(hands.next().expect("No first hand"))
                .expect("First is not a legal hand"),
            score: Score::decode(hands.next().expect("No second hand"))
                .expect("Second is not a legal hand"),
        }
    }

    fn score(&self) -> u64 {
        let mut wins = HashMap::new();
        wins.insert(Hand::Rock, Hand::Paper);
        wins.insert(Hand::Paper, Hand::Scissors);
        wins.insert(Hand::Scissors, Hand::Rock);

        let mut lose_to = HashMap::new();
        wins.iter().for_each(|(k, v)| {
            lose_to.insert(*v, *k);
        });

        let my = match self.score {
            Score::Win => wins[&self.opponent],
            Score::Lose => lose_to[&self.opponent],
            Score::Draw => self.opponent,
        };

        my as u64 + self.score as u64
    }
}

fn main() {
    // let input = INPUT;
    let input = std::fs::read_to_string("input.txt").expect("Where's the input file??");

    let total = input
        .lines()
        .map(|l| Round::from(l))
        .map(|r| r.score())
        .sum::<u64>();
    println!("{total}");
}
