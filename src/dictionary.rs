use itertools::Itertools;
use serde_json::Value;
use std::fs;

use crate::word::Word;

const ALL_WORDS: &str = "./dictionaries/dictionary-full-official.json";
const SOLUTIONS: &str = "./dictionaries/dictionary-answers-official.json";

pub fn get_all_words() -> Vec<Word> {
    let all_words = get_words(ALL_WORDS);
    let solutions = get_words(SOLUTIONS);

    all_words
        .chain(solutions)
        .unique()
        .sorted()
        .map(|w| Word::new(&w))
        .collect()
}

pub fn get_soln_words() -> Vec<Word> {
    get_words(SOLUTIONS).map(|w| Word::new(&w)).collect()
}

fn get_words(path: &str) -> impl Iterator<Item = String> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json = serde_json::from_str(&data).expect("JSON was not well-formatted");

    if let Value::Array(vector) = json {
        return vector.into_iter().map(|value| match value {
            Value::String(word) => word,
            _ => panic!("JSON element was not a valid string"),
        });
    }

    panic!("JSON was not a valid vector")
}
