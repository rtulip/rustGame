extern crate rand_chacha;
extern crate rand_core;
extern crate rand;
use rand_chacha::{ChaChaCore,ChaChaRng};
use rand_core::SeedableRng;
pub use rand_core::RngCore;

pub type Seed = <ChaChaCore as SeedableRng>::Seed;
pub type RNG = ChaChaRng;

/// Returns A Seed which can be used for the ChaCha random number generator
/// which will be used for the entirety of a game. 
/// 
/// The ChaCha random number generator requires [u8: 32] as input. If in debug
/// mode, the seed defualts to [1,2,3,4,5,6,7,8,
///                             1,1,1,1,1,1,1,1,
///                             2,2,2,2,2,2,2,2,
///                             1,2,3,4,5,6,7,8]. Otherwise, a new seed is 
/// generated using rand::random::<u8>() 32 times to fill the array.
pub fn create_seed(debug: bool) -> Seed {

    if debug {
        let seed: Seed = [1,2,3,4,5,6,7,8,
                                                       1,1,1,1,1,1,1,1,
                                                       2,2,2,2,2,2,2,2,
                                                       1,2,3,4,5,6,7,8];
        seed
    } else {
        let seed: Seed = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
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

/// Wrapper function for RNG::from_seed(seed: Seed)
pub fn from_seed(seed: Seed) -> RNG {
    RNG::from_seed(seed)
}

/// Wrapper function for RNG.next_u32().
pub fn next_u32(rng: &mut RNG) -> u32 {
    rng.next_u32()
}