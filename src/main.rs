use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let words_filename = Path::new("src/wordlist.txt");

    let words_file_contents = fs::read_to_string(words_filename)
        .expect("Couldn't read file at words_filename");

    let words = words_file_contents.split("\n");

    let mut root: Trie = Trie {
        value: Option::None,
        children: vec![],
        is_word: false,
    };

    for word in words {
        root.add(word);
    }

    root.print();
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Trie {
    value: Option<char>,
    children: Vec<Trie>,
    is_word: bool,
}

impl Trie {
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

    fn new(value: char) -> Trie {
        Trie {
            value: Some(value),
            is_word: false,
            children: vec![]
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
}


fn head_tail(string: &str) -> (char, &str) {
    let mut chars = string.chars();
    let head: char = chars.next().unwrap();
    let tail = chars.as_str();

    (head, tail)
}

#[cfg(test)]
mod tests;