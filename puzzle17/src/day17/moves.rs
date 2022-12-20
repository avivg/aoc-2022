use itertools::Itertools;
use std::iter::once;

#[derive(Clone, Debug)]
pub enum Move {
    Sideways(isize),
    Down
}

pub struct Moves {
    move_iter: Box<dyn Iterator<Item = Move>>,
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        let side_moves = String::from(s)
            .into_bytes()
            .into_iter()
            .map(|c| match c as char {
                '<' => Move::Sideways(-1),
                '>' => Move::Sideways(1),
                _ => unreachable!(),
            })
            .cycle();
        let downs = once(Move::Down).cycle();
        Self {
            move_iter: Box::new(side_moves.interleave(downs)),
        }
    }
}

impl Iterator for Moves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.move_iter.next()
    }
}
