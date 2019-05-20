extern crate rust_game;
use rust_game::level::{Level,LevelController};
use rust_game::misc::random;
use rust_game::input;

fn main() {

    let config = input::handle_init_input();
    let seed = random::create_seed(config.debug);
    let lvl: Level = Level::new(seed);
    let lvlctr: LevelController = LevelController::new(lvl);
    lvlctr.print_level();

}