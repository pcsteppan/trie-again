#![allow(dead_code)]

use std::{fs, io};
use std::{path::Path};

use regex::Regex;

use game::GameController;
use trie::*;
use gamestate::*;

fn main() {
    let words_filename = Path::new("./game/src/data/wordlist.txt");

    let words_file_contents =
        fs::read_to_string(words_filename).expect("Couldn't read file at words_filename");

    let words = words_file_contents
        .split("\n")
        .filter(|w| w.len() == 5);

    let mut root: Trie = Trie::new_root();

    for word in words {
        root.add(word);
    }

    let mut gc = GameController::new(root);
    gc.start_random();

    let mut last_guess_was_correct = None;
    while gc.state.words.as_slice().iter().any(|word| !word.completed) {
        let mut user_guess = String::from("");
        for word in gc.state.words.as_slice() {
            // Shows answers side-by-side for debugging
            // println!("{} {} {}", word.masked_word, word.word, if word.completed {"c"} else {"x"});

            println!("{} {}", 
                if word.completed { word.word.to_string() } else { word.masked_word.to_string() },
                if word.completed { "✅" } else { " " });
        }

        if last_guess_was_correct.is_some() {
            println!("{}", if last_guess_was_correct.unwrap() {"✅"} else {"❓"});
        }

        println!("Enter your guess for one of the words above or q to quit:");
        
        io::stdin().read_line(&mut user_guess).expect("Failed to read line");
        println!("");
        user_guess = user_guess.trim().to_string();
        if user_guess.eq("q") {
            break;
        }

        last_guess_was_correct = Some(gc.submit_guess(&user_guess));
    }

    let missed_words = gc.state.words.iter().filter(|w| !w.completed).collect::<Vec<&GameWord>>();
    println!("\r\n\r\nYou got {} out of {} words:", 
        gc.state.words.len() - missed_words.len(),
        gc.state.words.len());

    if missed_words.len() != gc.state.words.len() {
        println!("These are the ones you got:");
        for word in gc.state.words
            .as_slice()
            .iter() 
            .filter(|w| w.completed)
            .collect::<Vec<&GameWord>>()
        {
            println!("{}", word.word);
        }
    }

    if missed_words.len() > 0 {
        println!("These are the ones you missed:");
        for word in missed_words {
            println!("{}", word.word);
        }
    }
}