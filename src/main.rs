fn main() {
    let words = vec!["car", "cat", "cab", "dog", "door", "doom", "apple"];

    let letter = "abc";

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
                let mut chars = string.chars();
                let head: char = chars.next().unwrap();
                let tail = chars;

                let mut matching_branches = self
                    .children
                    .iter_mut()
                    .filter(|child| child.value == Some(head))
                    .collect::<Vec<_>>();

                match matching_branches.first_mut() {
                    None => {
                        let mut new_child = Trie {
                            value: Some(head),
                            children: vec![],
                            is_word: false,
                        };

                        new_child.add(tail.as_str());
                        self.children.push(new_child);
                    }
                    Some(branch) => {
                        branch.add(tail.as_str());
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
}
