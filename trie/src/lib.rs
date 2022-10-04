use std::{collections::HashMap};

#[derive(PartialEq, Debug)]
pub struct Trie {
    pub value: Option<char>,
    pub children: Vec<Trie>,
    pub is_word: bool,
}

type WordFrequencies = HashMap<String, u16>;

impl Trie {
    pub fn new_root() -> Trie {
        Trie {
            value: None,
            is_word: false,
            children: vec![],
        }
    }

    pub fn new(value: char) -> Trie {
        Trie {
            value: Some(value),
            is_word: false,
            children: vec![],
        }
    }

    pub fn add(&mut self, string: &str) {
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

    pub fn print(&self) {
        self.print_helper("", "", false);
    }

    // TODO: add variant that collapses single-child nodes into same line
    pub fn print_helper(&self, prefix: &str, word: &str, is_last: bool) {
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

    /*
    traverse takes three functions:
    +   transform of type FT:
        a function which calculates the aggregated value (A) we want to see returned on each node,
        e.g, this node if it is a word and the transformed value meets some criteria
        and a transformed value for children to use in calculating their own values (T),
        e.g, a substring that is being built during the traversal of the trie
    
    +   merge of type FM:
        a function which can be used to reduce all returns (of type A) into one aggregate/accumulated thing
    
    +   recurse of type FR:
        a function which chooses which of the trie's children to recursively call traverse on, e.g,
        recursively call trie's children if current transformed value (type T) meets some criteria
    
    traverse utilizes two generics throughout its functions
    +   A:
        the return type of the traversal at each trie node

    +   T:
        a transformed/derived state calculated from the state of the current trie node, 
        and the parent trie node's T value, passed to it's children as t,
        must be clonable until I can figure how to make that not necessary

    */

    fn traverse<A, T: Clone, FT, FM, FR>(&self, transform: &FT, merge: &FM, recurse: &FR, t: &T) -> Option<A> 
        where FT: Fn(&Trie, &T) -> (T, A),
              FM: Fn(A, A) -> A,
              FR: Fn(&dyn Fn(&Trie) -> Option<A>, &Trie, (T, A)) -> Vec<A> {
        let (t_prime, acc): (T, A) = transform(self, t);

        let recursive_call = |child: &Trie| {
            child.traverse(transform, merge, recurse, &t_prime.clone())
        };
        
        recurse(&recursive_call, self, (t_prime.clone(), acc))
            .into_iter()
            .reduce(merge)
    }

    fn traverse_common_recurse<A, T>(recursive_call: &dyn Fn(&Trie) -> Option<A>, trie: &Trie, curr_transform : (T, A)) -> Vec<A> {
        trie.children
            .iter()
            .flat_map(recursive_call)
            .chain(std::iter::once(curr_transform.1))
            .collect()
    }
    
    pub fn get_child(&mut self, value: char) -> Option<&mut Trie> {
        let mut matching_branches = self
            .children
            .iter_mut()
            .filter(|child| child.value == Some(value));

        matching_branches.next()
    }

    pub fn get_word(&mut self, string: &str) -> Option<&mut Trie> {
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

    pub fn contains(&mut self, string: &str) -> bool {
        self.get_word(string).is_some()
    }

    pub fn has_word(&mut self, string: &str) -> bool {
        self.get_word(string).map_or(false, |t| t.is_word)
    }
    
    pub fn get_all_substring_frequencies(&self, substring_length: usize) -> WordFrequencies {
        type A = WordFrequencies;
        type T = String;

        let transform = |trie: &Trie, t: &String| {
            let mut word_freqs = HashMap::new();

            let mut new_word = if trie.value.is_none() {
                t.to_owned()
            } else {
                t.to_owned() + trie.value.unwrap().to_string().as_str()
            };

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
        self.traverse(&transform, &merge, &Self::traverse_common_recurse, &str).unwrap_or_default()
    }

    pub fn get_all_words(&self) -> Vec<String> {
        type A = Vec<String>;
        type T = String;

        let transform = |trie: &Trie, t: &T| {
            let curr_word = if trie.value.is_some() {
                t.to_owned() + &trie.value.unwrap().to_string()
            } else {
                t.to_owned()
            };
            let aggregate_words = if trie.is_word {
                vec![curr_word.clone()]
            } else {
                Vec::new()
            };
            
            (curr_word, aggregate_words)
        };

        let merge = |x: A, y: A| x.into_iter().chain(y).collect();

        let str = String::new();
        self.traverse(&transform, &merge, &Self::traverse_common_recurse, &str).unwrap_or_default()
    }

    pub fn get_all_words_with_containing_substring(&self, substring: &str) -> Vec<String> {
        type A = Vec<String>;
        type T = (String, bool);

        let transform = |trie: &Trie, t: &T| -> (T, A){
            let curr_word = if trie.value.is_some() {
                t.0.to_owned() + &trie.value.unwrap().to_string()
            } else {
                t.0.to_owned()
            };

            let should_add = t.1 || curr_word.contains(substring); 

            let aggregate_words = if should_add && trie.is_word {
                vec![curr_word.clone()]
            } else {
                Vec::new()
            };

            ((curr_word, should_add), aggregate_words)
        };

        let merge = |x: Vec<String>, y: Vec<String>| x.into_iter().chain(y).collect();

        let t: T = (String::new(), false);
        self.traverse(&transform, &merge, &Self::traverse_common_recurse, &t).unwrap_or_default()
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