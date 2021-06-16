mod dice;
use dice::Sequence;
mod error;
use error::Error;
mod map;
use map::Map;

use std::fs::File;

pub fn run(wordlist: &str, wordcount: u32) -> Result<(), Error> {
    let mut file = File::open(wordlist)?;
    let map = Map::new(&mut file)?;
    for _ in 0..wordcount {
        let sequence = Sequence::new(5);
        println!("{} ", map.get(&u32::from(sequence)).unwrap());
    }
    Ok(())
}
