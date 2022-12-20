use crate::day17::moves::Move;

pub struct Shapes {
    next_shape: Box<dyn Iterator<Item = Shape>>,
}

impl Shapes {
    pub fn new() -> Self {
        let shapes = [
            // y\x 0123
            // 0   ####
            Shape {
                tiles: vec![Tile(0, 0), Tile(1, 0), Tile(2, 0), Tile(3, 0)],
            },
            // y\x 012
            // 2   .#.
            // 1   ###
            // 0   .#.
            Shape {
                tiles: vec![Tile(1, 2), Tile(0, 1), Tile(1, 1), Tile(2, 1), Tile(1, 0)],
            },
            // y\x 012
            // 2   ..#
            // 1   ..#
            // 0   ###
            Shape {
                tiles: vec![Tile(2, 2), Tile(2, 1), Tile(0, 0), Tile(1, 0), Tile(2, 0)],
            },
            // y\x 0
            // 3   #
            // 2   #
            // 1   #
            // 0   #
            Shape {
                tiles: vec![Tile(0, 0), Tile(0, 1), Tile(0, 2), Tile(0, 3)],
            },
            // y\x 01
            // 1   ##
            // 0   ##
            Shape {
                tiles: vec![Tile(0, 1), Tile(1, 1), Tile(0, 0), Tile(1, 0)],
            },
        ];
        Self {
            next_shape: Box::new(shapes.into_iter().cycle()),
        }
    }
}

impl Iterator for Shapes {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_shape.next()
    }
}

#[derive(Clone)]
pub struct Shape {
    pub tiles: Vec<Tile>,
}

#[derive(Clone)]
pub struct Tile(pub isize, pub isize);

impl Shape {
    pub fn apply_move(&self, m: &Move) -> Self {
        match m {
            Move::Sideways(dx) => self.shift(*dx, 0).unwrap_or_else(|()| self.clone()),
            Move::Down => self.shift(0, -1).unwrap(),
        }
    }

    pub fn shift(&self, dx: isize, dy: isize) -> Result<Self, ()> {
        let mut res = self.clone();
        for t in &mut res.tiles {
            t.0 += dx;
            if t.0 < 0 || t.0 > 6 {
                return Err(());
            }
            t.1 += dy;
        }
        Ok(res)
    }

}
