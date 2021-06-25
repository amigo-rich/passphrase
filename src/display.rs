pub enum Preference<'a> {
    Seperator(&'a str),
}

pub struct Formatter<'a> {
    seperator: &'a str,
}

impl Default for Formatter<'_> {
    fn default() -> Self {
        Formatter { seperator: " " }
    }
}

impl<'a> Formatter<'a> {
    pub fn new(preferences: &[Preference<'a>]) -> Self {
        let mut f = Formatter::default();
        for preference in preferences {
            match preference {
                Preference::Seperator(s) => f.seperator = *s,
            }
        }
        f
    }
    pub fn format(&self, words: &[&str]) -> String {
        let mut output = String::new();
        let count = words.len();
        for (index, word) in words.iter().enumerate() {
            output.push_str(word);
            if index != count - 1 {
                output.push_str(self.seperator);
            }
        }
        output
    }
}
