use std::fmt::{Display, Formatter, Result};
use std::cmp::min;
use std::cmp::Ordering::{Less, Equal, Greater};
use hlt::constants::MAX_SPEED;
use hlt::command::Command;
use hlt::entity::{Entity, Ship};
use hlt::game_map::GameMap;

use super::behavior::Behavior;

struct UnitState {}

#[derive(Debug)]
pub struct Unit {
    pub ship_id: i32,
    pub behavior: Behavior,
    pub target: Option<i32>,
}

impl Unit {
    pub fn new(ship: &Ship, behavior: Behavior) -> Unit {
        Unit {
            behavior: behavior,
            ship_id: ship.id,
            target: None,
        }
    }

    /// Updates the units target if necessary
    pub fn update(&mut self, ship: &Ship, game_map: &GameMap) {
        match self.behavior {
            Behavior::Settler => {
                let target = self.update_settler_target(&ship, game_map);
                if target.is_none() {
                    self.behavior = Behavior::Raider;
                    self.update(ship, game_map)
                } else {
                    self.target = target;
                }
            }
            Behavior::Raider => self.target = self.update_raider_target(&ship, game_map),
            Behavior::Defense => self.target = self.update_defender_target(&ship, game_map),
        }
    }

    /// Executes the logic for the play
    pub fn execute(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        match self.behavior {
            Behavior::Settler => self.exec_settler(&ship, game_map),
            Behavior::Raider => self.exec_raider(&ship, game_map),
            Behavior::Defense => self.exec_defender(&ship, game_map),
        }
    }

    fn update_settler_target(&self, ship: &Ship, game_map: &GameMap) -> Option<i32> {
        if ship.is_docked() {
            return self.target;
        }

        let bot_id = game_map.me().id;
        let target = if let Some(target) = self.target {
            game_map.all_planets().get(target as usize)
        } else {
            None
        };

        // find a new target
        let planet_iter = game_map.all_planets().iter();

        planet_iter
            .filter(|planet| if let Some(owner) = planet.owner {
                (owner == bot_id) && !planet.is_full()
            } else {
                true
            })
            .min_by(|&a, &b| {
                let dist_a = ship.distance_with(a);
                let dist_b = ship.distance_with(b);
                if dist_a < dist_b {
                    Less
                } else if dist_a == dist_b {
                    Equal
                } else {
                    Greater
                }
            })
            .and_then(|planet| Some(planet.id))
    }

    fn exec_settler(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        self.target
            .and_then(|id| game_map.all_planets().get(id as usize))
            .and_then(|planet| if ship.can_dock(planet) {
                Some(ship.dock(planet))
            } else {
                self.navigate(ship, &ship.closest_point_to(planet, 3.0), game_map)
            })
    }

    fn exec_raider(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        self.target
            .and_then(|id| game_map.all_planets().get(id as usize))
            .and_then(|planet| if ship.can_dock(planet) {
                Some(ship.dock(planet))
            } else {
                self.navigate(ship, &ship.closest_point_to(planet, 3.0), game_map)
            })
    }

    fn update_raider_target(&self, ship: &Ship, game_map: &GameMap) -> Option<i32> {
        if ship.is_docked() {
            return self.target;
        }

        let bot_id = game_map.me().id;
        let target = if let Some(target) = self.target {
            game_map.all_planets().get(target as usize)
        } else {
            None
        };

        if target.is_some() {
            if ship.is_docked() {
                return self.target;
            }
        }

        let planet_iter = game_map.all_planets().iter();
        planet_iter
            .filter(|planet| {
                let bot_id = game_map.me().id;
                if let Some(owner) = planet.owner {
                    (owner != bot_id)
                } else {
                    true
                }
            })
            .min_by(|&a, &b| {
                let dist_a = ship.distance_with(a);
                let dist_b = ship.distance_with(b);
                if dist_a < dist_b {
                    Less
                } else if dist_a == dist_b {
                    Equal
                } else {
                    Greater
                }
            })
            .and_then(|planet| Some(planet.id))
    }

    fn exec_defender(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        None
    }

    fn update_defender_target(&self, ship: &Ship, game_map: &GameMap) -> Option<i32> {
        None
    }

    fn navigate<T: Entity>(&self, ship: &Ship, target: &T, game_map: &GameMap) -> Option<Command> {
        let planet_collision = game_map.planet_between(ship, target);
        let ship_collision = game_map.ship_between(ship, target);

        if planet_collision.is_some() && ship_collision.is_some() {
            let ship_distance = ship.distance_with(ship_collision.unwrap());
            let planet_distance = ship.distance_with(ship_collision.unwrap());
            if (ship_distance < planet_distance) && (ship_distance < MAX_SPEED as f64) {
                return Some(self.handle_collision(ship, ship_collision.unwrap()));
            }
        }

        if ship_collision.is_some() {
            let collidee = ship_collision.unwrap();
            let ship_distance = ship.distance_with(collidee);
            if ship_distance < MAX_SPEED as f64 {
                return Some(self.handle_collision(ship, collidee));
            }
        }

        if planet_collision.is_some() {
            return Some(self.handle_collision(ship, planet_collision.unwrap()));
        }

        let distance = ship.distance_with(target);
        let angle = ship.angle_with(target);
        Some(ship.thrust(min(MAX_SPEED, distance as i32), angle as i32))
    }

    fn handle_collision<E: Entity>(&self, ship: &Ship, target: &E) -> Command {
        // gives just enough fudge that ships do crash might be able to grow
        // lower
        let magic_number = 0.45;

        let speed = MAX_SPEED;
        let distance = ship.distance_with(target) + magic_number;
        let radius = target.radius() + ship.radius() + magic_number;
        let angle = ship.angle_with(target);
        if distance < radius {
            panic!(
                "Distance {} must be larger than radius {}",
                distance,
                radius
            );
        }
        let offset = f64::asin(radius / distance).to_degrees();
        ship.thrust(min(speed, distance as i32), (angle + offset) as i32)
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unit(ship_id={}, behavior={}, target={})",
               self.ship_id,
               self.behavior,
               format!("{:?}", self.target),
        )
    }
}
