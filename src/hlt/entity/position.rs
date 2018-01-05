use std::fmt::{Display, Formatter, Result};
use std::cmp::Eq;
use hlt::parse::Decodable;
use hlt::entity::Entity;

/// A simple wrapper for a coordinate.
/// Intended to be passed to some functions in place of a ship or planet.
#[derive(Debug, Clone, Copy)]
pub struct Position(pub f64, pub f64);

impl Position {
    pub fn from_origin<T: Entity>(pos: &T, angle: f64, distance: f64) -> Position {
        let Position(x, y) = pos.position();
        let x = distance * f64::cos(angle) + x;
        let y = distance * f64::sin(angle) + y;
        Position(x, y)
    }
}

impl Decodable for Position {
    fn parse<'a, I>(tokens: &mut I) -> Position
    where
        I: Iterator<Item = &'a str>,
    {
        let x = f64::parse(tokens);
        let y = f64::parse(tokens);

        Position(x, y)
    }
}

impl Entity for Position {
    fn position(&self) -> Position {
        *self
    }

    fn radius(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.distance_with(other) <= 0.0001
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Position({}, {})", self.0 as f32, self.1 as f32)
    }
}
