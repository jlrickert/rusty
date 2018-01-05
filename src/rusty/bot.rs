extern crate rand;

use std::collections::HashMap;

use hlt::command::Command;
use hlt::entity::{Entity, DockingStatus};
use hlt::game::Game;
use hlt::game_map::GameMap;


use self::rand::{thread_rng, Rng};
use super::behavior::Behavior;
use super::unit::Unit;

#[derive(Debug)]
pub struct Bot {
    pub name: String,
    pub round: i32,
    pub fleet: HashMap<i32, Unit>,
}

impl Bot {
    pub fn new(game: &Game) -> Self {
        Bot {
            name: format!("rusty{}", game.my_id),
            round: 0,
            fleet: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, game_map: &GameMap) {
        info!("Initializing bot {}", self.name);
        info!("Initial ship count {}", game_map.me().all_ships().len());
    }

    pub fn play_round(&mut self, game_map: &GameMap, command_queue: &mut Vec<Command>) {
        self.round += 1;
        info!("Playing round {}", self.round + 1);

        self.update_units(&game_map);

        // Loop over all of our player's ships
        for ship in game_map.me().all_ships() {
            let id = ship.id;
            let unit = self.fleet.get_mut(&id).expect(&format!(
                "Unit {} doesn't exist or is dead",
                id
            ));

            let cmd = unit.execute(&ship, game_map);
            info!("{}\n executing {:?}", unit.to_string(game_map), cmd);
            if cmd.is_some() {
                command_queue.push(cmd.unwrap());
            }
        }
    }

    fn update_units(&mut self, game_map: &GameMap) {
        debug!("Updating data structures");
        for ship in game_map.me().all_ships() {
            let unit = if self.fleet.contains_key(&ship.id) {
                self.fleet.get_mut(&ship.id).unwrap()
            } else {
                let behavior = if self.round < 50 {
                    Behavior::Raider
                } else if thread_rng().gen_range(0.0, 100.0) <= 75.0 {
                    Behavior::Settler
                } else {
                    Behavior::Raider
                };
                self.fleet.insert(ship.id, Unit::new(ship, behavior));
                let u = self.fleet.get_mut(&ship.id).unwrap();
                info!("New unit:\n{}", u.to_string(game_map));
                u
            };

            unit.update(ship, game_map);
            info!(
                "Updating unit {}\n {}",
                unit.ship_id,
                unit.to_string(game_map)
            );
        }
    }
}
