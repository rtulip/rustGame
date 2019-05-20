use crate::entity::player::Player;
pub struct PlayerController {
    player: Player,
}

impl PlayerController {

    pub fn new(player: Player) -> Self {
        Self { player: player }
    } 

    // pub fn event<E: GenericEvent>(&mut self, e: &E) {
    //     TODO
    // } 

}