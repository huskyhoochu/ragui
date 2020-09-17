use regex::Regex;

pub struct Normalizer {
    normalized: String,
}

impl Normalizer {
    pub fn new(text: String) -> Normalizer {
        let mut normalizer = Normalizer { normalized: String::new() };
        normalizer.normalize(text);
        normalizer
    }

    fn normalize(&mut self, text: String) {
        let has_carriage = Regex::new(r"\r\n?").unwrap();
        let too_many_newline = Regex::new(r"\n+$").unwrap();
        let result = has_carriage.replace_all(text.as_str(), "\n").into_owned();
        let result = too_many_newline
            .replace_all(result.as_str(), "\n")
            .into_owned();

        self.normalized = result;
    }

    pub fn get(&self) -> Vec<&str> {
        self.normalized.split("\n").collect()
    }
}

#[cfg(test)]
mod tests;
