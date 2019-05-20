extern crate rand_chacha;
extern crate rand_core;
extern crate rand;

extern crate rust_game;
use rust_game::level::Level;

use std::env;
use rand_chacha::ChaChaCore;
use rand_core::SeedableRng;


/// create_seed()
/// 
/// args:
///     debug: bool: A flag to used a fixed debug seed
/// 
/// returns: A Seed which can be used for the ChaCha random number generator
/// which will be used for the entirety of a game
/// 
/// The ChaCha random number generator requires [u8: 32] as input. If in debug
/// mode, the seed defualts to [1,2,3,4,5,6,7,8,
///                             1,1,1,1,1,1,1,1,
///                             2,2,2,2,2,2,2,2,
///                             1,2,3,4,5,6,7,8]. Otherwise, a new seed is 
/// generated using rand::random::<u8>() 32 times to fill the array.
fn create_seed(debug: bool) -> <ChaChaCore as SeedableRng>::Seed {

    if debug {
        let seed: <ChaChaCore as SeedableRng>::Seed = [1,2,3,4,5,6,7,8,
                                                       1,1,1,1,1,1,1,1,
                                                       2,2,2,2,2,2,2,2,
                                                       1,2,3,4,5,6,7,8];
        seed
    } else {
        let seed: <ChaChaCore as SeedableRng>::Seed = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];   

        seed
    }
}

fn main() {

    let mut debug = false;
    let args: Vec<String> = env::args().collect();

    // Argument parsing
    // cargo run -- *arguments go here*

    // Arguments: 
    //      -d | --debug: Use a constant known seed
    match args.len(){
        len if len > 1 => {
            for i in 1..args.len(){
                match &args[i]{
                    string if *string == String::from("-d") || *string == String::from("--debug") => {
                        debug = true;
                    },
                    _ => ()
                }
            }
        },
        _ => () 

    }

    let seed = create_seed(debug);
    let width: i32 = 50;
    let height: i32 = 50;
    let iters: i32 = 5;

    let lvl: Level = Level::new(width, height, iters, seed);
    
    lvl.print_level();

}