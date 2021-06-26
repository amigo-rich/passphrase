mod dice;
use dice::Sequence;
mod display;
use display::{Formatter, Preference};
mod error;
use error::Error;
mod map;
use map::Map;

use std::fs::File;

pub struct Runtime<'a> {
    pub wordlist: &'a str,
    pub wordcount: u32,
    pub seperator: &'a str,
    pub want_capital: bool,
}

pub fn run(runtime: &Runtime) -> Result<(), Error> {
    let mut file = File::open(runtime.wordlist)
        .map_err(|_| Error::FileNotFound(runtime.wordlist.to_string()))?;
    let map = Map::new(&mut file)?;
    let mut words: Vec<&str> = Vec::new();
    for _ in 0..runtime.wordcount {
        let sequence = Sequence::new(5);
        words.push(map.get(&u32::from(sequence)).unwrap());
    }
    let output = Formatter::new(
        [
            Preference::Seperator(runtime.seperator),
            Preference::CapitalCase(runtime.want_capital),
        ]
        .as_ref(),
    )
    .format(&words)?;
    println!("{}", output);
    Ok(())
}
