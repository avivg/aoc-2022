use std::collections::HashSet;

mod shapes;
use shapes::{Shapes, Shape, Tile};

use super::{Moves, moves::Move};

pub enum StepResult {
    Flying,
    Landed,
}

pub struct Tetris {
    pub floor: Vec<HashSet<isize>>,
    moves: Moves,
    shapes: Shapes,
    cur_shape: Shape,
}

impl Tetris {
    pub fn new(moves: Moves) -> Self {
        let mut shapes = Shapes::new();
        Self {
            floor: vec![HashSet::from([0]); 7],
            moves,
            cur_shape: shapes.next().unwrap().shift(2, 4).unwrap(),
            shapes,
        }
    }

    pub fn step(&mut self) -> StepResult {
        let next_move = self.moves.next().unwrap();
        let shape_in_next_pos = self.cur_shape.apply_move(&next_move);
        if self.intersect_with_floor(&shape_in_next_pos) {
            match next_move {
                Move::Sideways(_) => StepResult::Flying,
                Move::Down => {
                    self.land_shape();
                    StepResult::Landed
                }
            }
        } else {
            self.cur_shape = shape_in_next_pos;
            StepResult::Flying
        }
    }

    pub fn top(&self) -> isize {
        *self.floor.iter().map(|hs| hs.iter().max().unwrap()).max().unwrap()
    }

    fn intersect_with_floor(&self, shape: &Shape) -> bool {
        for Tile(x,y) in &shape.tiles {
            if self.floor[*x as usize].contains(y as &isize) {
                return true;
            }
        }
        false
    }
    
    fn land_shape(&mut self) {
        let cur = self.cur_shape.clone();
        for Tile(x, y) in cur.tiles {
            self.floor[x as usize].insert(y as isize);
        }
        self.trunc_floor();
        let top = self.top();
        self.cur_shape = 
            self.shapes
            .next()
            .unwrap()
            .shift(2, top + 4)
            .unwrap();
    }

    fn trunc_floor(&mut self) {
        let tops: Vec<_> = self.floor.iter().map(|hs| hs.iter().max().unwrap()).copied().collect();
        for i in 0..tops.len()-1 {
            let d = tops[i] - tops[i+1];
            let dir = d/d.abs();
            for n in 0..d.abs() {
                if ! self.floor[i].contains(&(tops[i] + n*dir)) {
                    return;
                }
            }
        }
        self.floor = tops.iter().map(|t| HashSet::from([*t])).collect();
    }
}

