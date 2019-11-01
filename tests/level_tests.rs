extern crate rust_game;

mod common;

#[test]
fn test_level_generation() {
    use rust_game::entity::tile::TileVariant;
    use rust_game::levels::map::MapIdx;

    let lvl = common::setup_level();

    // Check that all tiles are walls, floors, or spawners
    for h in 0..lvl.height {
        for w in 0..lvl.width {
            assert!(if let Some(tile) = lvl.map.get(&MapIdx::new(w, h)) {
                match tile.variant {
                    TileVariant::Wall => true,
                    TileVariant::Floor => true,
                    TileVariant::Spawner => true,
                    _ => false,
                }
            } else {
                false
            });
        }
    }

    // Check that all the edges are walls
    for h in 0..lvl.height {
        assert!(if let Some(tile) = lvl.map.get(&MapIdx::new(0, h)) {
            match tile.variant {
                TileVariant::Wall => true,
                _ => false,
            }
        } else {
            false
        });

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
        assert!(if let Some(tile) = lvl.map.get(&MapIdx::new(w, 0)) {
            match tile.variant {
                TileVariant::Wall => true,
                _ => false,
            }
        } else {
            false
        });

        assert!(
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, lvl.height - 1)) {
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

#[test]
fn test_pathfinding() {
    use rust_game::entity::tile::TileVariant;
    use rust_game::levels::map::{pathfind, MapIdx};

    let lvl = common::setup_level();

    // test pathfinding from wall to floor
    let mut wall_idx = MapIdx::new(-1, -1);
    for h in 0..lvl.height {
        for w in 0..lvl.width {
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, h)) {
                match tile.variant {
                    TileVariant::Wall => {
                        if let Some(n) = lvl.map.get(&MapIdx::new(w - 1, h)) {
                            match n.variant {
                                TileVariant::Floor => {
                                    wall_idx = MapIdx::new(w, h);
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if let Some(n) = lvl.map.get(&MapIdx::new(w + 1, h)) {
                            match n.variant {
                                TileVariant::Floor => {
                                    wall_idx = MapIdx::new(w, h);
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if let Some(n) = lvl.map.get(&MapIdx::new(w, h - 1)) {
                            match n.variant {
                                TileVariant::Floor => {
                                    wall_idx = MapIdx::new(w, h);
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if let Some(n) = lvl.map.get(&MapIdx::new(w, h + 1)) {
                            match n.variant {
                                TileVariant::Floor => {
                                    wall_idx = MapIdx::new(w, h);
                                    break;
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        if wall_idx.x > 0 {
            break;
        }
    }

    let mut floor_idx_1 = MapIdx::new(-1, -1);
    for h in (0..lvl.height).rev() {
        for w in (0..lvl.width).rev() {
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, h)) {
                match tile.variant {
                    TileVariant::Floor => {
                        floor_idx_1 = MapIdx::new(w, h);
                        break;
                    }
                    _ => (),
                }
            }
        }
        if floor_idx_1.x > 0 {
            break;
        }
    }

    let mut floor_idx_2 = MapIdx::new(-1, -1);
    for h in 0..lvl.height {
        for w in 0..lvl.width {
            if let Some(tile) = lvl.map.get(&MapIdx::new(w, h)) {
                match tile.variant {
                    TileVariant::Floor => {
                        floor_idx_2 = MapIdx::new(w, h);
                        break;
                    }
                    _ => (),
                }
            }
        }
        if floor_idx_2.x > 0 {
            break;
        }
    }

    println!("wall_idx: {:?}", wall_idx);
    println!("floor_idx_1: {:?}", floor_idx_1);
    println!("floor_idx_2: {:?}", floor_idx_2);

    // test pathfinding from floor to wall
    if let Some(_path) = pathfind(&lvl.map, &floor_idx_1, &wall_idx) {
        panic!("Found path from a floor into a wall")
    }

    // test pathfinding from floor to floor

    if let Some(_path) = pathfind(&lvl.map, &floor_idx_1, &floor_idx_2) {
    } else {
        panic!("Unable to find path from floor to floor")
    }
}
