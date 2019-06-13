extern crate rust_game;

mod common;

#[test]
fn test_level_generation(){
    use rust_game::levels::map::MapIdx;
    use rust_game::entity::tile::TileVariant;
    
    let lvl = common::setup_level();
    
    // Check that all tiles are walls, floors, or spawners
    for h in 0..lvl.height {
        for w in 0..lvl.width {

            assert!(
                if let Some(tile) = lvl.map.get(&MapIdx::new(w, h)) {
                    match tile.variant {
                        TileVariant::Wall => true,
                        TileVariant::Floor => true,
                        TileVariant::Spawner => true,
                        _ => false
                    }
                } else {
                    false
                }
            );

        }
    }

    // Check that all the edges are walls
    for h in 0..lvl.height {

        assert!(
            if let Some(tile) = lvl.map.get(&MapIdx::new(0, h)) {
                match tile.variant {
                    TileVariant::Wall => true,
                    _ => false,
                }
            } else {
                false
            }
        );

        assert!(
            if let Some(tile) = lvl.map.get(&MapIdx::new(lvl.width - 1, h)) {
                match tile.variant {
                    TileVariant::Wall => true,
                    _ => false,
                }
            } else {
                false
            }
        );

    }

    for w in 0..lvl.width {

        assert!(
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, 0)) {
                match tile.variant {
                    TileVariant::Wall => true,
                    _ => false,
                }
            } else {
                false
            }
        );

        assert!(
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, lvl.height-1)) {
                match tile.variant {
                    TileVariant::Wall => true,
                    _ => false,
                }
            } else {
                false
            }
        );

    }

}