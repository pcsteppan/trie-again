#[allow(dead_code)]

use std::fs;
use std::{path::Path, collections::HashMap};

fn main() {
    let words_filename = Path::new("src/wordlist.txt");

    let words_file_contents = fs::read_to_string(words_filename)
        .expect("Couldn't read file at words_filename");

    let words = words_file_contents.split("\n");

    let mut root: Trie = Trie::new_root();

    for word in words {
        root.add(word);
    }

    let frequencies = root.get_all_substring_frequencies(6);
    let mut count_vec: Vec<_> = frequencies.iter().collect();

    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (k, v) in &count_vec[0..10] {
        println!("{}:\t{}", k, v);
    }

    // root.print();
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Trie {
    value: Option<char>,
    children: Vec<Trie>,
    is_word: bool,
}

impl Trie {
    fn new_root() -> Trie {
        Trie {
            value: None,
            is_word: false,
            children: vec![]
        }    
    }
    
    fn new(value: char) -> Trie {
        Trie {
            value: Some(value),
            is_word: false,
            children: vec![]
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
            }
            else {
                print!("├");
                new_prefix.push('│');
            }

            println!(
                "{} {}",
                current_node_value,
                if self.is_word { format!("{} {}", '✅', new_wordpart) } else { String::new() }
            );
        }

        for (i, child) in self.children.as_slice().iter().enumerate() {
            let is_last = i == self.children.len()-1;
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
            0 => {
                Some(self)
            },
            _ => {
                let (head, tail) = head_tail(string);
                
                match self.get_child(head) {
                    None => {
                        None
                    },
                    Some(child) => {
                        child.get_word(tail)
                    }
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

    fn get_all_words(&self) -> Vec<String> {
        let mut all_words = Vec::new();
        let mut frontier: Vec<(&Trie, String)> = Vec::new();
        frontier.push((&self, String::new()));

        while !frontier.is_empty() {
            let (curr_node, mut curr_word) = frontier.pop().unwrap();
            if curr_node.value.is_some() {
                curr_word += curr_node.value.unwrap().to_string().as_str();
            }

            if curr_node.is_word {
                all_words.push(curr_word.clone());
            }
            for child in curr_node.children.as_slice() {
                frontier.push((child, curr_word.clone()));
            }
        }

        return all_words;
    }

    fn get_all_substring_frequencies(&self, substring_length: usize) -> HashMap<String, u16> {
        let mut frequencies: HashMap<String, u16> = HashMap::new();

        let mut frontier: Vec<(&Trie, String)> = Vec::new();
        frontier.push((&self, String::new()));

        while !frontier.is_empty() {
            let (curr_node, mut curr_word) = frontier.pop().unwrap();
            if curr_node.value.is_some() {
                let new_value = curr_node.value.unwrap().to_string();
                if curr_word.len() >= substring_length {
                    curr_word = curr_word[1..].to_string();
                }
                curr_word += new_value.as_str();
            }

            if curr_word.len() == substring_length {
                let incremented_frequency = frequencies.get(&curr_word).unwrap_or(&0) + 1;
                frequencies.insert(curr_word.clone(), incremented_frequency);
            }

            for child in curr_node.children.as_slice() {
                frontier.push((child, curr_word.clone()));
            }
        }

        return frequencies;
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