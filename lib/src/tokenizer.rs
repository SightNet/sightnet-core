use hashbrown::HashSet;
use lazy_static::lazy_static;
use rust_stemmers::{Algorithm, Stemmer};

use crate::term::Term;

lazy_static! {
    static ref STEMMER: Stemmer = Stemmer::create(Algorithm::English);
    static ref SEPARATORS: HashSet<char> = vec![' ', '.', ',', '!', ':', '?'].into_iter().collect();
}

pub fn tokenize(text: &str) -> Vec<Term> {
    text.split(|x: char| (&SEPARATORS).contains(&x))
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| Term {
            value: STEMMER.stem(x.to_lowercase().as_str()).to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("Hello, world!"),
            vec!["hello".into(), "world".into()]
        );
    }
}
