mod string;
mod words;

use std::path::Path;
use std::io;

use string::{compare_strings, mask_word};
use words::{load_words,select_random_word};

fn main() {
    println!("Hangman");

    let filename = Path::new("src/words.txt");
    let words = load_words(&filename).unwrap();
    let target_word = select_random_word(&words).unwrap();
    // println!("{}", target_word);
    println!("The word is {} letters long", target_word.len());
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut game_progress = vec!['*'; target_word.len()];
    
    loop {
        let formatted_string: String = guessed_letters.iter().map(|&c| c.to_string()).collect::<Vec<String>>().join(", ");
        println!("Guessed letters: {}", formatted_string);

        println!("Please input your next guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guessed_letter = guess.chars().next().unwrap();
        if guessed_letters.contains(&guessed_letter) {
            println!("You have already guessed: {guessed_letter}\n");
            continue
        }
        else {
            println!("You guessed: {guessed_letter}");
        }

        guessed_letters.push(guessed_letter);

        for (i, c) in target_word.chars().enumerate() {
            if c == guessed_letter {
                game_progress[i] = guessed_letter;
            }
        }
        let updated_progress: String = game_progress.clone().into_iter().collect();
        println!("{updated_progress}");

        // let match_vec = compare_strings(&guess, &target_word);
        // let masked_word = mask_word(match_vec, &target_word);

        // println!("{masked_word}");
        
        if !game_progress.contains(&'*') {
            break
        }
    }
    println!("You won in {} guesses!", guessed_letters.len())
}
