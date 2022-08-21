
fn main() {
    let words = vec!["car", "cat", "cab", "dog", "door", "doom", "apple"];

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
        self.print_helper(0, ' ');
    }

    fn print_helper(&self, indent: usize, prefix: char) {
        match self.value {
            None => (),
            Some(value) => {
                println!(
                    "{}{}{} {}",
                    " ".repeat(indent - 1),
                    prefix,
                    value,
                    if self.is_word { '✅' } else { ' ' }
                );
            }
        }
        for child in self.children.as_slice() {
            let prefix = if self.value == None {
                '>'
            } else {
                if self.children.len() > 1 {
                    '├'
                } else {
                    '└'
                }
            };
            child.print_helper(indent + 1, prefix);
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

    fn contains(&self, string: &str) -> bool {
        // split head tail

        // base case, empty string -> true

        // recursive case, get child is not none -> child.contains(tail)
        //                           is none     -> false
        return true;
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