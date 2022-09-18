#![allow(dead_code)]

use std::{fs};
use std::{path::Path};

use trie::*;
use gamestate::*;

fn main() {
    let words_filename = Path::new("./game/src/data/most_common_10_000.txt");

    let words_file_contents =
        fs::read_to_string(words_filename).expect("Couldn't read file at words_filename");

    let words = words_file_contents
        .split("\n")
        .filter(|w| w.len() == 5);

    let mut root: Trie = Trie::new_root();

    for word in words {
        root.add(word);
    }

    let all_words = root.get_all_words();
    println!("{}", all_words.len());

    // for word in all_words {
    //     println!("{}", word);
    // }

    let mut all_freqs_of_length_3 = root
        .get_all_substring_frequencies(3)
        .into_iter()
        .collect::<Vec<(String, u16)>>();
        //.sort_by(|a, b| b.1.cmp(&a.1));
    all_freqs_of_length_3.sort_by(|a, b| a.1.cmp(&b.1));

    for (k, v) in all_freqs_of_length_3 {
        println!("{}: {}", k, v);
    }

    // root.print();
}