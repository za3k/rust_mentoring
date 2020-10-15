use std::fs;
use regex::Regex;
use std::collections::HashMap;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct Dictionary {
    words: Vec<String>,
    definitions: HashMap<String, Vec<String>>,
}

const START_DICT: &str = "*** START OF THIS PROJECT GUTENBERG EBOOK WEBSTER'S UNABRIDGED DICTIONARY ***";
const WORD_REGEX: &str = r"(?m)^[[:upper:]]+$";
fn main() -> std::io::Result<()> {
    let contents: String = fs::read_to_string("pg29765.txt")?;
    let contents: String = String::from(contents.split(START_DICT).nth(1).expect("Not the 1911 webster dictionary"));
    let contents = contents.replace("\r\n", "\n");
    let contents = contents.replace("CONSOLATION\nRACE", "CONSOLATION RACE"); // Take care of one multi-line word

    // Find words (all caps, on a line by themselves)
    let re = Regex::new(WORD_REGEX).unwrap();
    let word_ranges: Vec<(String, usize, usize)> = re.captures_iter(contents.as_str()).map(|cap| {
        let x = cap.get(0).unwrap();
        (String::from(x.as_str().trim().to_lowercase()), x.start(), x.end())
    }).collect();

    // Definitions are taken to be anything between a pair of words. A word can have more than one definition, so collect them as a list, not a string.
    let mut definitions: HashMap<String, Vec<String>> = HashMap::new();
    let mut words: Vec<String> = word_ranges.iter().map(|x| x.0.clone()).collect();
    words.dedup();
    for i in 0..word_ranges.len() {
        let word = word_ranges[i].0.clone();
        let definition = if i+1 < word_ranges.len() {
            contents[(word_ranges[i].2)..(word_ranges[i+1].1)].trim()
        } else {
            contents[(word_ranges[i].2)..].trim()
        };
        definitions.entry(word).or_insert(Vec::new()).push(String::from(definition));
    }
    let definitions: HashMap<String, Vec<String>> = words.iter().map(|word| definitions.remove_entry(word).unwrap()).collect();

    // JSON should have both a list of words (for binary search), and a mapping from word to definition
    let dictionary = Dictionary {
        words: words,
        definitions: definitions
    };
    let json = serde_json::to_string(&dictionary)?; // No need to quote anything, this will be inserted verbatim into javascript.
    println!("{}", json);

    Ok(())
}
