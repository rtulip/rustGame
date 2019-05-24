use rust_game::game::Game;
use rust_game::input;
use rust_game::misc::random;

fn main() {
    
    // Parse command line for input commands
    let config = input::handle_init_input();
    // Create the seed used for the run
    let seed = random::create_seed(config.debug);

    // Create a new Game object and start the game loop.
    let mut game = Game::new(seed);
    game.run();

}