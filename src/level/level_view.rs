use crate::level::LevelViewSettings;

pub struct LevelView {

    pub settings: LevelViewSettings,

}

impl LevelView {

    pub fn new(settings: LevelViewSettings) -> LevelView {
        LevelView {settings: settings}
    }
    
}