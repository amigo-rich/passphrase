use crate::error::Error;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct Pair {
    roll: u32,
    word: String,
}

impl Pair {
    pub fn new(line: &str) -> Result<Pair, Error> {
        if line.is_empty() || line.len() > 128 {
            return Err(Error::ParseLine(line.to_string()));
        }

        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 2 {
            return Err(Error::ParseLine(line.to_string()));
        }
        let roll: u32 = match fields[0].parse() {
            Ok(roll) => roll,
            Err(_) => {
                return Err(Error::ParseLineNumber(line.to_string()));
            }
        };
        let word = String::from(fields[1].trim());
        Ok(Pair { roll, word })
    }
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<u32, String>,
}

impl Map {
    pub fn new(file: &mut File) -> Result<Map, Error> {
        let mut input = String::new();
        let mut map: HashMap<u32, String> = HashMap::new();

        file.read_to_string(&mut input)?;
        if input.is_empty() {
            return Err(Error::IO);
        }
        for line in input.lines() {
            let pair = Pair::new(line)?;
            map.insert(pair.roll, pair.word);
        }
        Ok(Map { map })
    }
    pub fn get(&self, k: &u32) -> Option<&String> {
        self.map.get(k)
    }
}
