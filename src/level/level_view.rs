pub struct LevelViewSettings {

    pub graphics_position: [f64; 2],
    pub size: f64,
    pub tile_size: f64,

}

impl LevelViewSettings {

    pub fn new() -> LevelViewSettings {

        LevelViewSettings{
            graphics_position: [10.0; 2],
            size: 500.0,
            tile_size: 10.0, 
        }
    }
}


pub struct LevelView {

    pub settings: LevelViewSettings,

}

impl LevelView {

    pub fn new(settings: LevelViewSettings) -> LevelView {
        LevelView {settings: settings}
    }
    
    // pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
    //     TODO
    // }

}