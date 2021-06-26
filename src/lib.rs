mod dice;
use dice::Sequence;
mod display;
use display::{Formatter, Preference};
mod error;
use error::Error;
mod map;
use map::Map;

use std::fs::File;

pub fn run(
    wordlist: &str,
    wordcount: u32,
    seperator: &str,
    want_capital: bool,
) -> Result<(), Error> {
    let mut file = File::open(wordlist).map_err(|_| Error::FileNotFound(wordlist.to_string()))?;
    let map = Map::new(&mut file)?;
    let mut words: Vec<&str> = Vec::new();
    for _ in 0..wordcount {
        let sequence = Sequence::new(5);
        words.push(map.get(&u32::from(sequence)).unwrap());
    }
    let output = Formatter::new(
        [
            Preference::Seperator(seperator),
            Preference::CapitalCase(want_capital),
        ]
        .as_ref(),
    )
    .format(&words)?;
    println!("{}", output);
    Ok(())
}
