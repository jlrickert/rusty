use std::collections::HashMap;
use std::cmp::Ordering::{Less, Equal, Greater};
use super::game::Game;
use super::entity::{GameState, Planet};
use super::constants::MAX_SPEED;
use super::player::Player;
use super::collision::intersect_segment_circle;
use super::entity::{Entity, Ship};

/// Map which houses the current game information/metadata.
#[derive(Debug)]
pub struct GameMap<'a> {
    game: &'a Game,
    state: GameState,
}

impl<'a> GameMap<'a> {
    pub fn new(game: &'a Game, state: GameState) -> Self {
        Self {
            game: game,
            state: state,
        }
    }

    /// Return your own player.
    pub fn me(&self) -> &Player {
        let my_id = self.game.my_id;
        &self.state.players[my_id]
    }

    /// Returns all planets at the actual game state.
    pub fn all_planets(&self) -> &[Planet] {
        &self.state.planets
    }

    /// Returns all players at the actual game state including yourself.
    pub fn all_players(&self) -> &[Player] {
        &self.state.players
    }

    pub fn all_ships(&self) -> Box<[&Ship]> {
        let mut ships = Vec::new();
        for player in self.all_players() {
            for ship in player.all_ships() {
                ships.push(ship)
            }
        }
        ships.into_boxed_slice()
    }

    pub fn get_planet(&self, planet_id: i32) -> Option<&Planet> {
        for planet in self.all_planets() {
            if planet.id == planet_id {
                if planet.hp <= 0 {
                    return None;
                } else {
                    return Some(planet);
                }
            }
        }
        None
    }

    pub fn get_ship(&self, ship_id: i32) -> Option<&Ship> {
        for player in self.all_players() {
            for ship in player.all_ships() {
                if ship.id == ship_id {
                    return Some(ship);
                }
            }
        }
        None
    }

    pub fn obstacles_between<T: Entity>(&self, ship: &Ship, target: &T, fudge: f64) -> bool {
        for planet in self.all_planets() {
            if intersect_segment_circle(ship, target, planet, ship.radius() + fudge) {
                return true;
            }
        }

        for entity in self.all_ships().iter() {
            if ship != *entity &&
                entity.distance_with(target) < MAX_SPEED as f64 &&
                intersect_segment_circle(ship, target, *entity, 0.1)
            {
                trace!("Collision detected between {} and {}", ship.id, entity.id);
                return true;
            }
        }
        return false;
    }

    pub fn planet_between<E: Entity, T: Entity>(
        &self,
        ship: &E,
        target: &T,
        fudge: f64,
    ) -> Option<&Planet> {
        self.all_planets()
            .iter()
            .filter(|planet| {
                intersect_segment_circle(ship, target, *planet, ship.radius() + fudge)
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
    }

    pub fn ship_collision<T: Entity>(&self, ship: &Ship, target: &T, fudge: f64) -> Option<&Ship> {
        for ship_ in self.me().all_ships() {
            let distance = target.distance_with(ship_);
            if distance >= MAX_SPEED as f64 || ship.id == ship_.id {
                continue;
            }
            if intersect_segment_circle(ship, target, ship_, ship_.radius() + fudge) {
                return Some(ship_);
            }
        }
        None
    }
}
