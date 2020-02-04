/// Rust Hangman by Angus L'Herrou
/// v.0.1.1
/// https://github.com/angus-lherrou/rs-hangman

use rand::Rng;
use std::io;
use std::collections::HashSet;
use std::str::FromStr;
use std::io::Write;

/// Generates the hangman diagram for the current game state.
/// Input: u32, number of wrong guesses; &HashSet<char>, set of wrong guesses
/// Return: String, the diagram
fn gen_hangman(wrong_ct: u32, wrong_letters: &HashSet<char>) -> String {
    let letter_list =
        String::from_utf8((b'a'..=b'z')
            .filter_map(|x| {
                let x = x as char;
                if wrong_letters.contains(&x) {Some(x as u8)} else {None}
            }).collect()).unwrap();

    let letter_str = letter_list.as_str();

    let top = "  +---+\n  |   |\n";

    let head = match wrong_ct {
        0 => "      |",
        _ => "  O   |    Wrong: ",
    };

    let body = match wrong_ct {
        0..=1 => "      |",
        2 => "  |   |",
        3 => " /|   |",
        _ => " /|\\  |",
    };

    let legs = match wrong_ct {
        0..=4 => "      |\n",
        5 => " /    |\n",
        _ => " / \\  |\n"
    };

    let base = "|=========";

    let guesses_remaining =
        if wrong_ct > 0 {
            format!("    Guesses remaining: {}", 6 - wrong_ct)
        } else {
            String::new()
        };

    top.to_owned() + head + letter_str + "\n" + body + guesses_remaining.as_str() + "\n" + legs + base
}

/// Matches a guessed letter against the actual word.
/// Builds the output recursively using the previous guesses.
/// Input: &str, the guessed letter; &str, the actual word; &str, the previous matched letters
/// Return: (bool, String): whether any letters were guessed, and the new matched letters
fn match_guess(guess: &str, actual: &str, guessed: &str) -> (bool, String) {
    let mut comp = String::new();
    let mut bln = false;
    if actual.len() == 0 {
        return (false, String::new());
    } else if guess == &actual[..1] {
        comp.push_str(guess);
        bln = true;
    } else {
        comp.push_str(&guessed[..1]);
    }
    let prev = match_guess(guess, &actual[1..], &guessed[1..]);
    (prev.0 || bln, comp + prev.1.as_ref())
}

fn main() {
    println!("Welcome to Rust Hangman!");
    println!();
    let vocab: [&str; 15] =
        ["hangman", "jazz", "pizza", "hajj", "vocabulary",
            "rust", "stern", "queue", "apple", "banana",
            "snake", "crack", "snack", "asks", "ornithology"];
    let mut exit_game = false;
    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    // Play the game
    while !exit_game {
        let idx: usize = rng.gen_range(0, 15);
        let word: &str = vocab[idx];
        println!("Word length: {}. ", word.len());
        println!("Let's play!\n");
        let mut won = false;
        let initial_matched_letters = "_".repeat(word.len());
        let mut matched_letters = initial_matched_letters.as_str();
        let mut wrong_ct = 0;
        let mut ans: (bool, String);
        let mut hs: HashSet<char> = HashSet::new();
        // Play a round
        while !won && wrong_ct < 6 {
            println!("{}", gen_hangman(wrong_ct, &hs));
            println!("$ {} $", matched_letters);
            print!("> ");
            io::stdout().flush().unwrap();
            let mut guess = String::new();
            stdin.read_line(&mut guess)
                .expect("Failed to read line");
            guess = (&guess[..1]).to_string();

            ans = match_guess(guess.as_str(), word, matched_letters);
            matched_letters = ans.1.as_str();
            println!();
            if !ans.0 {
                let guess_char = char::from_str(guess.as_str()).unwrap();
                hs.insert(guess_char);
                wrong_ct += 1;
            }
            if matched_letters == word {
                won = true;
            }
        } // Round over
        println!("{}", gen_hangman(wrong_ct, &hs));
        println!();
        if won {
            println!("/#{}#\\", "#".repeat(word.len()));
            println!("% {} %", matched_letters);
            println!("\\#{}#/", "#".repeat(word.len()));
            println!();
            println!("You won!")
        } else {
            println!("\\&{}&/", "&".repeat(word.len()));
            println!("@ {} @ != {}", matched_letters, word);
            println!("/&{}&\\", "&".repeat(word.len()));
            println!();
            println!("You lost...")
        }
        println!("Would you like to play again? (y/_) ");
        let mut leave = String::new();
        stdin.read_line(&mut leave)
            .expect("Failed to read line");
        exit_game = match &leave[..1] {
            "y" => false,
            _ => true,
        };
        println!("Goodbye!");
    } // Game over
}
