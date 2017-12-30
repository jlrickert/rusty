mod hlt;
mod rusty;

use hlt::game::Game;
use rusty::Bot;

fn main() {
    let name = "Rusty";

    // Initiailize the game
    let game = Game::new();

    // Initiailize the bot
    let mut rusty = Bot::new(&game);

    // Initialize logging
    // let mut logger = Logger::new(game.my_id);
    // logger.log(&format!("Starting my {} bot!", name));

    // Retrieve the first game map
    let game_map = game.update_map();

    // You can preprocess things here,
    // you have 60 seconds...
    rusty.initialize(&game_map);

    // Once you are done, send a "ready to work"
    game.send_ready(&rusty.name);

    let mut command_queue = Vec::new();
    loop {
        // Update the game state
        let game_map = game.update_map();

        rusty.play_round(&game_map, &mut command_queue);

        // Send our commands to the game
        game.send_command_queue(&command_queue);
        command_queue.clear();
    }
}
