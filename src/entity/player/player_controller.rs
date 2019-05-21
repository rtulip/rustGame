use crate::entity::player::Player;
use crate::traits;
use piston::input::GenericEvent;
pub struct PlayerController {
    pub player: Player,
}

impl PlayerController {

    pub fn get_position(&self) -> [f64; 2] {
        self.player.position
    }

}

impl traits::Controller<[f64; 2], Player> for PlayerController {

    fn new(player: Player) -> Self {
        Self { player: player }
    } 

    fn event<E: GenericEvent>(&mut self, e: &E) {
        // TODO
    }

}