use rand::Rng;
use std::io;

fn gen_hangman(wrong_ct: u32, wrong_letters: &String) -> String {
    let letter_list =
        String::from_utf8((b'a'..=b'z')
            .filter_map(|x| {
                let x = x as char;
                if wrong_letters.contains(x.to_string().as_str()) {Some(x as u8)} else {None}
            }).collect()).unwrap();
    let letter_str = letter_list.as_str();

    let top = "  +---+\n  |   |\n";
    let head = match wrong_ct {
        0 => "      |",
        _ => "  O   |    Wrong: ",
    };
    let body = match wrong_ct {
        0..=1 => "      |\n",
        2 => "  |   |\n",
        3 => " /|   |\n",
        _ => " /|\\  |\n",
    };
    let legs = match wrong_ct {
        0..=4 => "      |\n",
        5 => " /    |\n",
        _ => " / \\  |\n"
    };
    let base = "|=========";
    top.to_owned() + head + letter_str + "\n" + body + legs + base
}

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
        let mut acc = String::new();
        while !won && wrong_ct < 6 {
            println!("{}", gen_hangman(wrong_ct, &acc));
            println!("{}", matched_letters);
            let mut guess = String::new();
            stdin.read_line(&mut guess)
                .expect("Failed to read line");
            guess = (&guess[..1]).to_string();

            ans = match_guess(guess.as_str(), word, matched_letters);
            matched_letters = ans.1.as_str();
            println!();
            if !ans.0 {
                acc += guess.as_str();
                wrong_ct += 1;
            }
            if matched_letters == word {
                won = true;
            }
        }
        if won {
            println!("You won!")
        } else {
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
    }
}
