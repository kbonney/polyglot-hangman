use rand::Rng; // Add `rand` crate to your `Cargo.toml`
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

// Function to load words from a file
pub fn load_words(filename: &Path) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let words = lines.collect::<Result<_, _>>()?;
    Ok(words)
}

// Function to randomly select a word
pub fn select_random_word(words: &[String]) -> Option<&String> {
    let mut rng = rand::thread_rng();
    if words.is_empty() {
        None
    } else {
        let index = rng.gen_range(0..words.len());
        words.get(index)
    }
}
