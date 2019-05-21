use crate::entity::player::Player;
use crate::traits;
use piston::input::GenericEvent;
pub struct PlayerController {
    pub player: Player,
}

impl traits::Controller<[f64; 2], Player> for PlayerController {

    fn new(player: Player) -> Self {
        Self { player: player }
    } 

    fn event<E: GenericEvent>(&mut self, e: &E) {
        // TODO
    }

}