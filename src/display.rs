use crate::error::Error;

pub enum Preference<'a> {
    Seperator(&'a str),
    CapitalCase(bool),
}

pub struct Formatter<'a> {
    seperator: &'a str,
    capital_case: bool,
}

impl Default for Formatter<'_> {
    fn default() -> Self {
        Formatter {
            seperator: " ",
            capital_case: false,
        }
    }
}

impl<'a> Formatter<'a> {
    pub fn new(preferences: &[Preference<'a>]) -> Self {
        let mut f = Formatter::default();
        for preference in preferences {
            match preference {
                Preference::Seperator(s) => f.seperator = *s,
                Preference::CapitalCase(b) => f.capital_case = *b,
            }
        }
        f
    }
    pub fn format(&self, words: &[&str]) -> Result<String, Error> {
        if words.is_empty() {
            return Err(Error::InvalidArgument(
                "Formatter::format".to_string(),
                "argument 'words' is empty".to_string(),
            ));
        }
        let mut output = String::new();
        let count = words.len();
        for (index, word) in words.iter().enumerate() {
            if word.is_empty() {
                return Err(Error::InvalidArgument(
                    "Formatter::format".to_string(),
                    "Empty word provided".to_string(),
                ));
            }
            if self.capital_case {
                output.push(word.chars().next().unwrap().to_ascii_uppercase());
                output.push_str(&word[1..]);
            } else {
                output.push_str(word);
            }
            if index != count - 1 {
                output.push_str(self.seperator);
            }
        }
        Ok(output)
    }
}
