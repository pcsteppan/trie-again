#![allow(dead_code)]

use std::{fs};
use std::{collections::HashMap, path::Path};

fn main() {
    let words_filename = Path::new("src/data/most_common_10_000.txt");

    let words_file_contents =
        fs::read_to_string(words_filename).expect("Couldn't read file at words_filename");

    let words = words_file_contents.split("\n").filter(|w| w.len() >= 4);

    let mut root: Trie = Trie::new_root();

    for word in words {
        root.add(word);
    }

    let all_words = root.get_all_words();
    println!("{}", all_words.len());

    for word in all_words {
        println!("{}", word);
    }

    let all_freqs_of_length_3 = root.get_all_substring_frequencies(3);

    for (k, v) in all_freqs_of_length_3.into_iter() {
        println!("{}: {}", k, v);
    }

    // root.print();
}

#[derive(PartialEq, Debug)]
struct Trie {
    value: Option<char>,
    children: Vec<Trie>,
    is_word: bool,
}

type WordFrequencies = HashMap<String, u16>;

impl Trie {
    fn new_root() -> Trie {
        Trie {
            value: None,
            is_word: false,
            children: vec![],
        }
    }

    fn new(value: char) -> Trie {
        Trie {
            value: Some(value),
            is_word: false,
            children: vec![],
        }
    }

    fn add(&mut self, string: &str) {
        match string.len() {
            // base case
            0 => {
                self.is_word = true;
            }
            // recursive case
            _ => {
                let (head, tail) = head_tail(string);

                match self.get_child(head) {
                    None => {
                        let mut new_child = Trie::new(head);

                        new_child.add(tail);

                        self.children.push(new_child);
                    }
                    Some(branch) => {
                        branch.add(tail);
                    }
                }
            }
        }
    }

    fn print(&self) {
        self.print_helper("", "", false);
    }

    // TODO: add variant that collapses single-child nodes into same line
    fn print_helper(&self, prefix: &str, word: &str, is_last: bool) {
        let mut new_prefix = prefix.clone().to_owned();
        let mut new_wordpart = word.clone().to_owned();

        if self.value != None {
            print!("{}", prefix);

            let current_node_value = self.value.unwrap();
            new_wordpart.push(current_node_value);

            if is_last {
                print!("└");
                new_prefix.push(' ');
            } else {
                print!("├");
                new_prefix.push('│');
            }

            println!(
                "{} {}",
                current_node_value,
                if self.is_word {
                    format!("{} {}", '✅', new_wordpart)
                } else {
                    String::new()
                }
            );
        }

        for (i, child) in self.children.as_slice().iter().enumerate() {
            let is_last = i == self.children.len() - 1;
            child.print_helper(&new_prefix, &new_wordpart, is_last);
        }
    }

    fn get_child(&mut self, value: char) -> Option<&mut Trie> {
        let mut matching_branches = self
            .children
            .iter_mut()
            .filter(|child| child.value == Some(value));

        return matching_branches.next();
    }

    fn get_word(&mut self, string: &str) -> Option<&mut Trie> {
        match string.len() {
            0 => Some(self),
            _ => {
                let (head, tail) = head_tail(string);

                match self.get_child(head) {
                    None => None,
                    Some(child) => child.get_word(tail),
                }
            }
        }
    }

    fn contains(&mut self, string: &str) -> bool {
        self.get_word(string).is_some()
    }

    fn has_word(&mut self, string: &str) -> bool {
        self.get_word(string).map_or(false, |t| t.is_word)
    }
    
    fn get_all_substring_frequencies(&self, substring_length: usize) -> WordFrequencies {
        let transform = |u: &String, trie: &Trie| {
            let mut word_freqs = HashMap::new();

            let mut new_word = u.to_owned();
            if trie.value.is_some() {
                new_word += trie.value.unwrap().to_string().as_str();
            }

            let new_word_len = new_word.len();
            if new_word_len >= substring_length {
                new_word = new_word[new_word_len-substring_length..].to_string();
                let is_word_increment = if trie.is_word {1} else {0};
                word_freqs.insert(new_word.clone(), trie.children.len() as u16 + is_word_increment);
            }

            (new_word, word_freqs)
        };

        let merge = |x: WordFrequencies, y: WordFrequencies| {
            let mut new_word_freqs = WordFrequencies::new();

            for key in x.clone().into_keys().chain(y.clone().into_keys()) {
                new_word_freqs.insert(key.clone(), x.get(&key).unwrap_or(&0) + y.get(&key).unwrap_or(&0));
            }

            new_word_freqs
        };

        let str = String::new();
        self.traverse(&transform, &merge, &str).unwrap_or(WordFrequencies::new())
    }

    fn get_all_words(&self) -> Vec<String> {
        let transform = |u: &String, trie: &Trie| {
            let curr_word = if trie.value.is_some() {
                u.to_owned() + &trie.value.unwrap().to_string()
            } else {
                u.to_owned()
            };
            let aggregate_words = if trie.is_word {
                vec![curr_word.clone()]
            } else {
                Vec::new()
            };
            
            (curr_word, aggregate_words)
        };

        let merge = 
            |x: Vec<String>, y: Vec<String>| x.into_iter().chain(y).collect();

        let str = String::new();
        self.traverse(&transform, &merge, &str).unwrap_or(Vec::new())
    }

    fn get_all_words_with_containing_substring(&self, substring: &str) -> Vec<String> {
        let transform = |u: &(String, bool), trie: &Trie| {
            let curr_word = if trie.value.is_some() {
                u.0.to_owned() + &trie.value.unwrap().to_string()
            } else {
                u.0.to_owned()
            };

            let should_add = u.1 || curr_word.contains(substring); 

            let aggregate_words = if should_add && trie.is_word {
                vec![curr_word.clone()]
            } else {
                Vec::new()
            };

            ((curr_word, should_add), aggregate_words)
        };

        let merge = |x: Vec<String>, y: Vec<String>| x.into_iter().chain(y).collect();

        let u = (String::new(), false);
        self.traverse(&transform, &merge, &u).unwrap_or(Vec::new())
    }

    fn traverse<T, U, V, W>(&self, transform: &T, merge: &W, u: &U) -> Option<V> 
        where T: Fn(&U, &Trie) -> (U, V),
              W: Fn(V, V) -> V {
        let u_delta: (U, V) = transform(u, self);
        
        self.children
            .iter()
            .map(|child| child.traverse(transform, merge, &u_delta.0))
            .filter_map(|res| res)
            .chain(std::iter::once(u_delta.1))
            .reduce(merge)
    }
}

fn head_tail(string: &str) -> (char, &str) {
    let mut chars = string.chars();
    let head: char = chars.next().unwrap();
    let tail = chars.as_str();

    (head, tail)
}

#[cfg(test)]
mod tests;
