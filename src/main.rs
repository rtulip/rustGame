extern crate rust_game;
use rust_game::level::Level;
use rust_game::misc::random;
use rust_game::input;

fn main() {

    
    let init = input::handle_init_input();
    let seed = random::create_seed(init.debug);
    let lvl: Level = Level::new(seed);
    
    lvl.print_level();

}