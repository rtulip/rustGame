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