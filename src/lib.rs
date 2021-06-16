mod dice;
use dice::Sequence;
mod error;
use error::Error;
mod map;
use map::Map;

use std::fs::File;

const NWORDS: usize = 6;

pub fn run() -> Result<(), Error> {
    let mut file = File::open("assets/eff_large_wordlist.txt")?;
    let map = Map::new(&mut file)?;
    for _ in 0..NWORDS {
        let sequence = Sequence::new(5);
        println!("{} ", map.get(&u32::from(sequence)).unwrap());
    }
    Ok(())
}
