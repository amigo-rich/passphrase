use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Dice {
    roll: u8,
}

impl Dice {
    pub fn new() -> Dice {
        let mut rng = thread_rng();
        Dice {
            roll: rng.gen_range(1..=6),
        }
    }
}

#[derive(Debug)]
pub struct Sequence {
    rolls: Vec<Dice>,
}

impl Sequence {
    pub fn new(count: usize) -> Sequence {
        let mut rolls: Vec<Dice> = Vec::new();

        for _ in 0..count {
            rolls.push(Dice::new());
        }

        Sequence {
            rolls,
        }
    }
}

impl From<Sequence> for u32 {
    fn from(sequence: Sequence) -> u32 {
        let mut result: u32 = 0;

        for (index, value) in sequence.rolls.iter().enumerate() {
            let pow = (sequence.rolls.len() - index) - 1;
            let step = 10_u32.pow(pow as u32) as u32;
            result += value.roll as u32 * step as u32;
        }
        result
    }
}