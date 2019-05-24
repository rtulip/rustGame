use rust_game::game::Game;
use rust_game::input;
use rust_game::misc::random;

fn main() {
    
    let config = input::handle_init_input();
    let seed = random::create_seed(config.debug);

    let mut game = Game::new(seed);
    game.run();

}