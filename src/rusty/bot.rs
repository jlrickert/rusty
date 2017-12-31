extern crate rand;

use std::collections::HashMap;

use hlt::command::Command;
use hlt::entity::{Entity, DockingStatus};
use hlt::game::Game;
use hlt::game_map::GameMap;

use self::rand::{thread_rng, Rng};
use super::behavior::Behavior;
use super::logging::Logger;
use super::unit::Unit;

pub struct Bot {
    pub name: String,
    pub round: i32,
    fleet: HashMap<i32, Unit>,
    logger: Logger,
}

impl Bot {
    pub fn new(game: &Game) -> Bot {
        Bot {
            name: "Rusty".to_string(),
            round: 0,
            fleet: HashMap::new(),
            logger: Logger::new(game.my_id),
        }
    }

    pub fn initialize(&mut self, game_map: &GameMap) {
        self.logger.log(&format!("Initializing bot {}", self.name));
        self.logger.log(&format!(
            "Initial ship count {}",
            game_map.me().all_ships().len()
        ));
    }

    pub fn play_round(&mut self, game_map: &GameMap, command_queue: &mut Vec<Command>) {
        self.round += 1;
        self.logger.log(
            &format!("Playing round {}", self.round + 1),
        );

        self.logger.log(&format!("Owned ships {:?}", game_map.me().all_ships()));
        self.update_units(&game_map);
        self.logger.log(&format!("Current fleet {:?}", self.fleet));
        // let units = self.find_units();

        // Loop over all of our player's ships
        for ship in game_map.me().all_ships() {
            let id = ship.id;
            let unit = self.fleet.get(&id).expect(&format!(
                "Unit {} doesn't exist or is dead",
                id
            ));
            let cmd = unit.execute(&ship, game_map);
            self.logger.log(&format!("{} executing {:?}", unit, cmd));
            if cmd.is_some() {
                command_queue.push(cmd.unwrap());
            }
        }
    }

    fn update_units(&mut self, game_map: &GameMap) {
        self.logger.log(&format!("Updating data structures"));
        for ship in game_map.me().all_ships() {
            let unit = if self.fleet.contains_key(&ship.id) {
                self.fleet.get_mut(&ship.id).unwrap()
            } else {
                let behavior = if thread_rng().gen_range(0.0, 100.0) <= 50.0 {
                    Behavior::Raider
                } else {
                    Behavior::Settler
                };
                self.fleet.insert(ship.id, Unit::new(ship, behavior));
                let u = self.fleet.get_mut(&ship.id).unwrap();
                self.logger.log(&format!("Created new unit {}", u));
                u
            };
            unit.update(ship, game_map);
            self.logger.log(&format!("Updating unit {}", unit));
        }
    }
}
