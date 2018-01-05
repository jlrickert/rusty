use std::f64::consts::PI;
use std::collections::vec_deque::VecDeque;
use std::cmp::min;
use std::cmp::Ordering::{Less, Equal, Greater};
use hlt::constants::{MAX_SPEED, DOCK_RADIUS};
use hlt::command::Command;
use hlt::entity::{Entity, Ship, Position, Planet};
use hlt::game_map::GameMap;

use super::constants::MIN_PLANET_DISTANCE;
use super::behavior::Behavior;

#[derive(Debug)]
pub struct Unit {
    pub ship_id: i32,
    pub behavior: Behavior,
    pub target: Option<i32>,
    target_queue: VecDeque<Position>,
}

impl Unit {
    pub fn new(ship: &Ship, behavior: Behavior) -> Self {
        Unit {
            behavior: behavior,
            ship_id: ship.id,
            target: None,
            target_queue: VecDeque::new(),
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
               format!("{:?}", self.target_queue),
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
            self.target = None;
            self.target_queue.clear();
            debug!("Ship {}: Already docked or docking. Nothing to do", self.ship_id);
            return;
        }

        match self.behavior {
            Behavior::Settler => self.update_settler(&ship, game_map),
            Behavior::Raider => self.update_raider(&ship, game_map),
            _ => (),
        }
    }

    /// Execute the logic for this units turn
    pub fn execute(&mut self, ship: &Ship, game_map: &GameMap) -> Option<Command> {
        if ship.id != self.ship_id {
            panic!(format!(
                "Mismatch ship and unit id: Ship id == {}, Unit id == {}",
                ship.id,
                self.ship_id
            ))
        }

        if ship.is_docked() {
            debug!("Ship {}: docked with {:?}", self.ship_id, self.target);
            return Some(Command::Nop);
        }

        if let Some(target) = self.target.and_then(|id| game_map.get_planet(id)) {
            match self.behavior {
                Behavior::Raider => if ship.can_dock(target) {
                    debug!("Ship {}: docking with {:?}", self.ship_id, self.target);
                    return Some(ship.dock(target))
                },
                _ => if ship.can_dock(target) {
                    debug!("Ship {}: docking with {:?}", self.ship_id, self.target);
                    return Some(ship.dock(target))
                },
            }

            let pos = match self.behavior {
                Behavior::Raider =>{
                    if target.owner.is_none() {
                        ship.closest_point_to(target, MIN_PLANET_DISTANCE)
                    } else {
                        ship.furthest_point_to(target, MIN_PLANET_DISTANCE)
                    }
                }
                _ => ship.closest_point_to(target, MIN_PLANET_DISTANCE)
            };
            return ship.navigate_to(&pos, game_map);
            // if let Some(sub_target) = self.target_queue.pop_front() {
            //     if sub_target.distance_with(ship) > MAX_SPEED as f64 {
            //         self.target_queue.push_front(sub_target);
            //     }

            //     if sub_target.position() == target.position() {
            //         let pos = match self.behavior {
            //             Behavior::Raider => if target.owner.is_none() {
            //                 ship.closest_point_to(target, MIN_PLANET_DISTANCE)
            //             } else {
            //                 ship.furthest_point_to(target, MIN_PLANET_DISTANCE)
            //             },
            //             _ => ship.closest_point_to(target, MIN_PLANET_DISTANCE),
            //         };
            //         return ship.navigate_to(&pos, game_map, Some(MIN_PLANET_DISTANCE))
            //     } else {
            //         return ship.navigate_to(&sub_target, game_map, Some(MIN_PLANET_DISTANCE))
            //     }
        }
        return None
    }

    fn update_route(& mut self, game_map: &GameMap) {
        let ship = game_map.get_ship(self.ship_id).expect(
            "Bot updating a unit that no longer exists",
        );
        let target = self.target.and_then(|id| game_map.get_planet(id)).expect(
            "Target updated to an invalid planet",
        );

        let mut cur = ship.position();
        debug!("Ship {}: Updating route for target {}", ship.id, target);
        loop {
            // Possible conflicting planet
            if let Some(conflict) = game_map.planet_between(&cur, target, MIN_PLANET_DISTANCE) {
                if conflict == target {
                    self.target_queue.push_back(target.position);
                    break;
                }

                trace!("Ship {}: conflict detected between {} and {};", ship.id, cur, conflict.position());
                let angle = cur.angle_with(conflict);
                let distance = cur.distance_with(conflict);
                let radius = conflict.radius + MIN_PLANET_DISTANCE;
                let Position(planet_x, planet_y) = conflict.position();
                let pos = {
                    let p1 = {
                        let x = radius * f64::cos(angle + PI / 2.0) + planet_x;
                        let y = radius * f64::sin(angle + PI / 2.0) + planet_y;
                        Position(x, y)
                    };
                    let p2 = {
                        let x = radius * f64::cos(angle - PI / 2.0) + planet_x;
                        let y = radius * f64::sin(angle - PI / 2.0) + planet_y;
                        Position(x, y)
                    };
                    if target.distance_with(&p1) < target.distance_with(&p2) {
                        p1
                    } else {
                        p2
                    }
                };
                let middle = {
                    let Position(cur_x, cur_y) = cur.position();
                    let offset =  {
                        if radius < distance {
                            let sign = (cur.angle_with(&pos) - cur.angle_with(conflict)).signum();
                            sign * f64::asin(radius / distance)
                        } else {
                            PI / 2.0
                        }
                    };
                    let distance = MAX_SPEED as f64;
                    let x = distance * f64::cos(angle + offset) + cur_x;
                    let y = distance * f64::sin(angle + offset) + cur_y;
                    Position(x, y)
                };

                self.target_queue.push_back(middle);
                self.target_queue.push_back(pos);
                cur = pos;
            } else {
                trace!("No conflict found");
                self.target_queue.push_back(target.position);
                break;
            }
        }
        debug!("Ship {}: Route calculated {:?}", self.ship_id, self.target_queue)
    }

    fn update_settler(&mut self, ship: &Ship, game_map: &GameMap) {
        debug!("Ship {}: updating with settler settings", self.ship_id);
        let me = game_map.me().id;

        let need_new_target = self.target
            .and_then(|target| game_map.get_planet(target))
            .and_then(|planet| Some(planet.owner == Some(me) && planet.is_full()))
            .unwrap_or(true);

        if !need_new_target {
            return;
        }

        // find a new target
        let planet_iter = game_map.all_planets().iter();
        let target = planet_iter
            .filter(|planet| if planet.is_dead() {
                false
            } else {
                planet.owner == Some(me) && !planet.is_full() || planet.owner != Some(me)
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
            self.target_queue.clear();
            self.update(ship, game_map);
        } else {
            self.target = target;
            self.update_route(game_map);
        }
    }

    fn update_raider(&mut self, ship: &Ship, game_map: &GameMap) {
        debug!("Ship {}: updating with raider settings", self.ship_id);
        let me = game_map.me().id;

        let need_new_target = self.target
            .and_then(|target| game_map.all_planets().get(target as usize))
            .and_then(|planet| Some(planet.owner == Some(me)))
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
            self.update_route(game_map);
        }
    }
}
