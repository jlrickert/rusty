use hlt::command::Command;
use hlt::entity::{Entity, DockingStatus};
use hlt::game_map::GameMap;
use hlt::game::Game;

use super::logging::Logger;

pub struct Bot {
    pub name: String,
    pub round: i32,
    logger: Logger,
}

impl Bot {
    pub fn new(game: &Game) -> Bot {
        Bot {
            name: "Rusty".to_string(),
            round: 0,
            logger: Logger::new(game.my_id),
        }
    }

    pub fn initialize(&mut self, game_map: &GameMap) {
        self.logger.log(&format!("Initializing  bot {}", self.name));
    }

    pub fn play_round(&mut self, game_map: &GameMap, command_queue: &mut Vec<Command>) {
        self.round += 1;
        self.logger.log(&format!("Playing round {}", self.round + 1));

        self.update(&game_map);
        // let units = self.find_units();

        // Loop over all of our player's ships
        for ship in game_map.me().all_ships() {
            // Ignore ships that are docked or in the process of docking
            if ship.docking_status != DockingStatus::UNDOCKED {
                continue;
            }

            // Loop over all planets
            for planet in game_map.all_planets() {
                // Ignore unowned planets
                if planet.is_owned() {
                    continue;
                }

                // If we are close enough to dock, do it!
                if ship.can_dock(planet) {
                    command_queue.push(ship.dock(planet))
                } else {
                    // If not, navigate towards the planet
                    let navigate_command = ship.navigate(&ship.closest_point_to(planet, 3.0), &game_map, 90);
                    if let Some(command) = navigate_command {
                        command_queue.push(command)
                    }
                }
                break;
            }
        }
    }

    fn update(&mut self, game_map: &GameMap) {
        self.logger.log(&format!("Updating data structures"));
    }
}
