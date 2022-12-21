use std::io;
use rand;

/// Main function
fn main() {
    // Game setup and random word selection
    let words = vec!["pie", "menu", "guest", "story", "role", "meat", "owner", "mode", "power", "cheek", "disk", "dad", "chest", "photo", "map", "scene", "tooth", "poet", "media", "ad", "hall", "exam", "area", "buyer", "ear", "woman", "news", "tea", "music", "thing", "cell", "lady", "depth", "video", "poem", "death", "error", "river", "week", "hat", "food", "lake", "meal", "bonus", "drama", "debt", "bread", "steak", "truth", "loss"];
    let word = words[rand::random::<usize>() % words.len()];
    let mut missed_letters = Vec::new();
    let mut correct_letters = Vec::new();
    for _ch in word.chars() {
        correct_letters.push('_');
    }

    // Cheat for testing
    //println!("The word is {}", word);

    // Difficulty selection, defaults to 10 if difficulty is not 1, 2, or 3
    println!("Select a difficulty level: 1 (15 lives), 2 (10 lives), or 3 (5 lives)");
    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");
    let difficulty = difficulty.trim();
    let difficulty = difficulty.parse::<u32>().unwrap();
    
    // Main game loop
    while !is_game_won(&correct_letters, &word) && !is_game_lost(&missed_letters, difficulty) {
        println!("Word: {:?}", correct_letters);
        println!("Missed letters: {:?}", missed_letters);
        println!("Guess a letter:");

        // Get the guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        // Convert the guess to lowercase
        let guess = guess.to_lowercase();

        // Check if the guess is the word
        if guess == word {
            println!("\nYou won! The word was {}", word);
            break;
        }

        // Check if the guess is a letter
        if !guess.chars().next().unwrap().is_alphabetic() {
            println!("\nPlease enter a letter.");
            continue;
        }

        // Check if the letter has already been guessed
        if missed_letters.contains(&guess.chars().next().unwrap()) || correct_letters.contains(&guess.chars().next().unwrap()) {
            println!("\nYou have already guessed that letter.");
            continue;
        }

        // Check if the guess is correct
        println!("{}", guess);
        println!("{}", word);

        // Check if the guess is in the word
        let guess = guess.chars().next().unwrap();
        if word.contains(guess) {
            for (i, ch) in word.chars().enumerate() {
                if ch == guess {
                    correct_letters[i] = guess;
                }
            }
        } else {
            missed_letters.push(guess);
        }
    }
    
    // Check if the game is won or lost
    if is_game_won(&correct_letters, &word) {
        println!("\nYou won! The word was {}", word);
    } else if is_game_lost(&missed_letters, difficulty){
        println!("\nYou lost! The word was {}", word);
    }
}

/// Function to check if the game is won
fn is_game_won(correct_letters: &[char], word: &str) -> bool {
    let mut word_guessed = true;
    for (i, ch) in word.chars().enumerate() {
        if correct_letters[i] != ch {
            word_guessed = false;
        }
    }
    word_guessed
}

///Function to check if the game is lost
fn is_game_lost(missed_letters: &[char], difficulty: u32) -> bool {
    if difficulty == 1 as u32 {
        missed_letters.len() >= 15
    } else if difficulty == 2 as u32 {
        missed_letters.len() >= 10
    } else if  difficulty == 3 as u32 {
        missed_letters.len() >= 5
    } else {
        missed_letters.len() >= 10
    }
}