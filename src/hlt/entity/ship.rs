use std::f64;
use std::cmp::min;
use std::fmt::{Display, Formatter, Result};
use hlt::constants::{DOCK_RADIUS, SHIP_RADIUS, MAX_SPEED};
use hlt::command::Command;
use hlt::parse::Decodable;
use hlt::game_map::GameMap;
use super::{Position, Planet, DockingStatus};
use super::Entity;

/// A ship in the game.
#[derive(PartialEq, Debug)]
pub struct Ship {
    pub id: i32,
    pub position: Position,
    pub hp: i32,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub docking_status: DockingStatus,
    pub docked_planet: Option<i32>,
    pub progress: i32,
    pub cooldown: i32,
}

impl Ship {
    /// Generate a command to accelerate this ship.
    pub fn thrust(&self, magnitude: i32, angle: i32) -> Command {
        Command::Thrust(self.id, magnitude, angle)
    }

    /// Generate a command to dock to a planet.
    pub fn dock(&self, planet: &Planet) -> Command {
        Command::Dock(self.id, planet.id)
    }

    /// Generate a command to undock from the current planet.
    pub fn undock(&self) -> Command {
        Command::Undock(self.id)
    }

    /// Determine whether a ship is already docked to a planet.
    pub fn is_docked(&self) -> bool {
        self.docking_status == DockingStatus::DOCKED ||
            self.docking_status == DockingStatus::UNDOCKING
    }

    /// Determine whether a ship can dock to a planet.
    pub fn can_dock(&self, planet: &Planet) -> bool {
        if self.is_docked() {
            return false;
        }
        self.distance_with(planet) <= (DOCK_RADIUS + planet.radius + SHIP_RADIUS)
    }

    /// Find the furthest point to the given ship near the given target, outside
    /// its given radius a given distance away.
    pub fn furthest_point_to<T: Entity>(&self, target: &T, distance: f64) -> Position {
        let angle = self.angle_with(target);
        let radius = target.radius() + distance;
        let Position(target_x, target_y) = target.position();
        let x = target_x + radius * f64::cos(angle.to_radians());
        let y = target_y + radius * f64::sin(angle.to_radians());

        Position(x, y)
    }

    /// Find the closest point to the given ship near the given target, outside
    /// its given radius, with an added fudge of min_distance.
    pub fn closest_point_to<T: Entity>(&self, target: &T, min_distance: f64) -> Position {
        let angle = target.angle_with(self);
        let radius = target.radius() + min_distance;
        let Position(target_x, target_y) = target.position();
        let x = target_x + radius * f64::cos(angle.to_radians());
        let y = target_y + radius * f64::sin(angle.to_radians());

        Position(x, y)
    }

    pub fn navigate<T: Entity>(&self, target: &T, game_map: &GameMap) -> Option<Command> {
        let colliding_planet = game_map.planet_between(self, target, self.radius() + 0.1);
        if colliding_planet.is_none() {
            let distance = self.distance_with(target);
            let angle = self.angle_with(target);
            return Some(self.thrust(min(MAX_SPEED, distance as i32), angle as i32));
        }

        let planet = colliding_planet.unwrap();
        let (sub_target, is_left) = {
            let (left, right) = self.find_avoidance_pair(planet, 3.0);
            if left.distance_with(target) < right.distance_with(target) {
                (left, true)
            } else {
                (right, false)
            }
        };

        let distance = self.distance_with(&sub_target);
        let angle = self.angle_with(&sub_target);
        Some(self.thrust(min(MAX_SPEED, distance as i32), angle as i32))
    }

    fn find_avoidance_pair<T: Entity>(&self, target: &T, min_distance: f64) -> (Position, Position) {
        let pos = target.position();
        let radius = target.radius() + self.radius() + min_distance;
        let distance = self.distance_with(target);
        let angle = self.angle_with(target);
        let offset = if radius >= distance {
            f64::consts::PI/2.0
        } else {
            f64::asin(radius / distance)
        };

        // cos(a+o+90)(p_r+f)+p_x, sin(a+o+90)(p_r+f)t+p_y)

        let pi = f64::consts::PI;
        let left = {
            let x = pos.0 + (pos.0 + min_distance) * f64::cos(angle + offset + pi / 2.0);
            let y = pos.1 + (pos.1 + min_distance) * f64::sin(angle + offset + pi / 2.0);
            Position(x, y)
        };
        let right = {
            let x = pos.0 + (pos.0 + min_distance) * f64::cos(angle + offset + pi / 2.0);
            let y = pos.1 + (pos.1 + min_distance) * f64::sin(angle + offset - pi / 2.0);
            Position(x, y)
        };

        (left, right)
    }
}

impl Decodable for Ship {
    fn parse<'a, I>(tokens: &mut I) -> Ship
    where
        I: Iterator<Item = &'a str>,
    {
        let id = i32::parse(tokens);
        let position = Position::parse(tokens);
        let hp = i32::parse(tokens);
        let velocity_x = f64::parse(tokens);
        let velocity_y = f64::parse(tokens);
        let docking_status = DockingStatus::parse(tokens);
        let docked_planet_raw = i32::parse(tokens);
        let docked_planet = match docking_status {
            DockingStatus::UNDOCKED => None,
            _ => Some(docked_planet_raw),
        };
        let progress = i32::parse(tokens);
        let cooldown = i32::parse(tokens);

        Ship {
            id,
            position,
            hp,
            velocity_x,
            velocity_y,
            docking_status,
            docked_planet,
            progress,
            cooldown,
        }
    }
}

impl Entity for Ship {
    fn position(&self) -> Position {
        self.position
    }

    fn radius(&self) -> f64 {
        SHIP_RADIUS
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Ship(\n\tid={},\n\tpos={},\n\thp={},\n\tvelocity=({}, {}),\n\tdocking_status={})",
               self.id,
               self.position,
               self.hp,
               self.velocity_x,
               self.velocity_y,
               self.docking_status,
        )
    }
}
