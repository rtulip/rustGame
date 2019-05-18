// http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust

extern crate rand_chacha;
extern crate rand_core;
extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use rand_chacha::{ChaChaCore, ChaChaRng};
use rand_core::{SeedableRng,RngCore};

type Cell = (i32, i32);
type Colony = HashSet<Cell>;

fn print_colony(col: &Colony, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            print!("{} ",
                if col.contains(&(x, y)) {"O"}
                else {"."}
            );
        }
        println!();
    }
}

fn neighbours(&(x,y): &Cell) -> Vec<Cell> {
    vec![
        (x-1,y-1), (x,y-1), (x+1,y-1),
        (x-1,y),            (x+1,y),
        (x-1,y+1), (x,y+1), (x+1,y+1),
    ]
}

fn neighbour_counts(col: &Colony) -> HashMap<Cell, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    ncnts
}

fn generation(col: Colony) -> Colony {
    neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)|
            match (cnt, col.contains(&cell)) {
                (2, true) |
                (3, ..) => Some(cell),
                _ => None
        })
        .collect()
}

fn life(init: Vec<Cell>, iters: i32, width: i32, height: i32) {
    let mut col: Colony = init.into_iter().collect(); 
    for i in 0..iters+1 {
        println!("({})", &i);
        if i != 0 {
            col = generation(col);
        }
    }
    print_colony(&col, width, height);
}

fn generate_seed(fixed: bool) -> <ChaChaCore as SeedableRng>::Seed {

    if fixed {
        let seed: <ChaChaCore as SeedableRng>::Seed = [1,2,3,4,5,6,7,8,
                                                       1,1,1,1,1,1,1,1,
                                                       2,2,2,2,2,2,2,2,
                                                       1,2,3,4,5,6,7,8];
        return seed;
    } else {
        let seed: <ChaChaCore as SeedableRng>::Seed = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];   

        return seed;
    }
}

fn main() {

    let width = 50;
    let height = 50;
    let mut board: Vec<Cell> = Vec::new();

    let seed = generate_seed(false);
    let mut rng = ChaChaRng::from_seed(seed);

    for h in 0..height {
        for w in 0..width {
            if rng.next_u32() % 2 == 1 {
                board.push((w,h));
            }
        }
    }
    
    let iters = 5;

    life(board, iters, width, height);

}