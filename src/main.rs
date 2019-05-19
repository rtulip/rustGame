// http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust

extern crate rand_chacha;
extern crate rand_core;
extern crate rand;

use std::collections::HashMap;
use std::fmt;
use std::env;

use rand_chacha::{ChaChaCore, ChaChaRng};
use rand_core::{SeedableRng,RngCore};

enum Cell{
    Floor,
    Wall,
    Cust(i32),
}

impl fmt::Display for Cell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            Cell::Floor => write!(f, "."),
            Cell::Wall => write!(f, "0"),
            Cell::Cust(i) => write!(f, "{}", i)
        }
    }
}

type Pos = (i32, i32);
type Board = HashMap<Pos, Cell>;

fn print_board(board: &Board, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            match board.get(&(x,y)){
                Some(cell) => print!("{} ", cell),
                None => print!(". ")
            }
        }
        println!();
    }
}

fn neighbours(&(x,y): &Pos) -> Vec<Pos> {
    vec![
        (x-1,y-1), (x,y-1), (x+1,y-1),
        (x-1,y),            (x+1,y),
        (x-1,y+1), (x,y+1), (x+1,y+1),
    ]
}

fn neighbour_counts(board: &Board) -> HashMap<Pos, i32> {
    let mut ncnts = HashMap::new();
    for (pos, _cell) in board.iter(){
        for neighbour in neighbours(pos){
            *ncnts.entry(neighbour).or_insert(0) += 1;
        }
    }
    ncnts
}

fn generation(board: Board) -> Board {
    neighbour_counts(&board)
        .into_iter()
        .filter_map(|(pos, cnt)|
            match (cnt, board.contains_key(&pos)) {
                (2, true) |
                (3, ..) => Some((pos, Cell::Wall)),
                _ => None
        })
        .collect()
}

fn life(init: Board, iters: i32, width: i32, height: i32) {
    let mut board: Board = init; 
    for i in 0..iters+1 {
        if i != 0 {
            board = generation(board);
        }
    }
    print_board(&board, width, height);
}

fn generate_seed(debug: bool) -> <ChaChaCore as SeedableRng>::Seed {

    if debug {
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

    let mut debug = false;
    let args: Vec<String> = env::args().collect();

    match args.len(){
        len if len > 1 => {
            for i in 1..args.len(){
                match &args[i]{
                    string if *string == String::from("-d") => {
                        debug = true;
                    },
                    _ => ()
                }
            }
        },
        _ => () 

    }

    let width = 50;
    let height = 50;
    let mut board: Board = Board::new();

    let seed = generate_seed(debug);
    let mut rng = ChaChaRng::from_seed(seed);

    for h in 0..height {
        for w in 0..width {
            if rng.next_u32() % 2 == 1 {
                board.insert((w,h), Cell::Wall);
            }
        }
    }
    
    let iters = 5;

    life(board, iters, width, height);

}