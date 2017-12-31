use std::cmp::min;
use std::cmp::Ordering::{Less, Equal, Greater};
use hlt::constants::{MAX_SPEED, DOCK_RADIUS};
use hlt::command::Command;
use hlt::entity::{Entity, Ship, Position, Planet};
use hlt::game_map::GameMap;

use super::behavior::Behavior;

struct UnitState {}

#[derive(Debug)]
pub struct Unit {
    pub ship_id: i32,
    // ship: &'a Ship
    pub behavior: Behavior,
    pub target: Option<i32>,
    target_pos: Option<Position>,
}

impl Unit {
    pub fn new(ship: &Ship, behavior: Behavior) -> Unit {
        Unit {
            behavior: behavior,
            ship_id: ship.id,
            target: None,
            target_pos: None,
        }
    }

    pub fn to_string(&self, game_map: &GameMap) -> String {
        let ship = game_map.me().get_ship(self.ship_id);
        format!("Unit(\n\tbehavior={}, \n\ttarget_id={}, \n\ttarget_pos={}, ship={})",
               self.behavior,
               format!("{}",
                       self.target
                       .and_then(|id| Some(id.to_string()))
                       .unwrap_or("None".to_string())),
               format!("{}",
                       self.target_pos
                       .and_then(|pos| Some(format!("{}", pos)))
                       .unwrap_or("None".to_string())),
               ship.and_then(|ship| Some(format!("{}", ship))).unwrap_or("None".to_string()),
        )
    }

    /// Updates the units target if necessary
    pub fn update(&mut self, ship: &Ship, game_map: &GameMap) {
        if ship.id != self.ship_id {
            panic!(format!(
                "Mismatch ship and unit id: Ship id == {}, Unit id == {}",
                ship.id,
                self.ship_id
            ))
        }
        if ship.is_docked() {
            return;
        }

        match self.behavior {
            Behavior::Settler => self.update_settler(&ship, game_map),
            Behavior::Raider => self.update_raider(&ship, game_map),
            _ => (),
        }
    }

    /// Executes the logic for the play
    pub fn execute(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        if ship.id != self.ship_id {
            panic!(format!(
                "Mismatch ship and unit id: Ship id == {}, Unit id == {}",
                ship.id,
                self.ship_id
            ))
        }

        if ship.is_docked() {
            return Some(Command::Nop);
        }

        match self.behavior {
            Behavior::Settler => self.exec_settler(&ship, game_map),
            Behavior::Raider => self.exec_raider(&ship, game_map),
            _ => self.exec_settler(&ship, game_map),
        }
    }

    fn update_settler(&mut self, ship: &Ship, game_map: &GameMap) {
        let me = game_map.me().id;

        // find a new target
        let planet_iter = game_map.all_planets().iter();
        let target = planet_iter
            .filter(|planet| if planet.hp <= 0 {
                false
            } else {
                planet.owner == Some(me) && !planet.is_full()
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
            .and_then(|planet| Some(planet.id));

        if target.is_none() {
            self.behavior = Behavior::Raider;
            self.target = None;
            self.target_pos = None;
            self.update(ship, game_map);
        } else {
            self.target = target;
        }
    }

    fn update_raider(&mut self, ship: &Ship, game_map: &GameMap) {
        let me = game_map.me().id;

        let need_new_target = self.target
            .and_then(|target| game_map.all_planets().get(target as usize))
            .and_then(|planet| if planet.hp <= 0 {
                Some(true)
            } else {
                Some(planet.owner == Some(me))
            })
            .unwrap_or(true);

        if !need_new_target {
            return;
        }

        let planet_iter = game_map.all_planets().iter();
        let planet = planet_iter
            .filter(|planet| if let Some(owner) = planet.owner {
                owner != me
            } else {
                true
            })
            .min_by(|&a, &b| {
                let dist_a = ship.distance_with(a);
                let dist_b = ship.distance_with(b);

                if a.owner.is_none() && b.owner.is_some() {
                    Less
                } else if a.owner != Some(me) && b.owner == Some(me) {
                    Less
                } else if b.owner.is_none() && a.owner.is_some() {
                    Greater
                } else if b.owner != Some(me) && a.owner == Some(me) {
                    Greater
                } else {
                    if dist_a < dist_b {
                        Less
                    } else if dist_a == dist_b {
                        Equal
                    } else {
                        Greater
                    }
                }
            });

        if let Some(target) = planet {
            self.target = Some(target.id);
            if target.owner.is_some() {
                self.target_pos = Some(ship.furthest_point_to(target, 3.0));
            } else {
                self.target_pos = Some(ship.closest_point_to(target, 3.0));
            }
        }
    }

    fn exec_settler(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        self.target.and_then(|id| {
            let planet = game_map.get_planet(id).expect("wtf");
            if ship.can_dock(planet) {
                Some(ship.dock(planet))
            } else {
                ship.navigate(&ship.closest_point_to(planet, 3.0), game_map)
            }
        })
    }

    fn exec_raider(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        self.target
            .and_then(|id| game_map.all_planets().get(id as usize))
            .and_then(|planet| if ship.can_dock(planet) &&
                (ship.distance_with(&self.target_pos.unwrap()) <
                     DOCK_RADIUS)
            {
                Some(ship.dock(planet))
            } else {
                ship.navigate(&self.target_pos.unwrap(), game_map)
            })
    }


    fn exec_defender(&self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        None
    }

    fn update_defender_target(&self, ship: &Ship, game_map: &GameMap) -> Option<i32> {
        None
    }

    // fn navigate<T: Entity>(&self, ship: &Ship, target: &T, game_map: &GameMap) -> Option<Command> {
    //     let planet_collision = game_map.planet_between(ship, target, 0.1);
    //     let ship_collision = game_map.ship_between(ship, target);

    //     if planet_collision.is_some() && ship_collision.is_some() {
    //         let ship_distance = ship.distance_with(ship_collision.unwrap());
    //         let planet_distance = ship.distance_with(ship_collision.unwrap());
    //         if (ship_distance < planet_distance) && (ship_distance < MAX_SPEED as f64) {
    //             return Some(self.handle_collision(ship, ship_collision.unwrap()));
    //         }
    //     }

    //     if ship_collision.is_some() {
    //         let collidee = ship_collision.unwrap();
    //         let ship_distance = ship.distance_with(collidee);
    //         if ship_distance < MAX_SPEED as f64 {
    //             return Some(self.handle_collision(ship, collidee));
    //         }
    //     }

    //     if planet_collision.is_some() {
    //         return Some(self.handle_collision(ship, planet_collision.unwrap()));
    //     }

    //     let distance = ship.distance_with(target);
    //     let angle = ship.angle_with(target);
    //     Some(ship.thrust(min(MAX_SPEED, distance as i32), angle as i32))
    // }

    // fn handle_collision<E: Entity>(&self, ship: &Ship, target: &E) -> Command {
    //     // gives just enough fudge that ships do crash might be able to grow
    //     // lower
    //     let magic_number = 0.45;

    //     let speed = MAX_SPEED;
    //     let distance = ship.distance_with(target) + magic_number;
    //     let radius = target.radius() + ship.radius() + magic_number;
    //     let angle = ship.angle_with(target);
    //     if distance < radius {
    //         panic!(
    //             "Distance {} must be larger than radius {}",
    //             distance,
    //             radius
    //         );
    //     }
    //     let offset = f64::asin(radius / distance).to_degrees();
    //     ship.thrust(
    //         min(speed, (distance - magic_number) as i32),
    //         (angle + offset) as i32,
    //     )
    // }
}
