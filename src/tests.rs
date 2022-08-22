use super::*;

#[test]
fn new_trie() {
    let trie = Trie::new('a');
    assert_eq!(trie.value, Some('a'));
    assert_eq!(trie.children.len(), 0);
    assert_eq!(trie.is_word, false);
}

#[test]
fn add_descendant() {
    let mut trie = Trie::new('a');
    trie.add("b");

    assert!(trie.children.first().map_or(false, |b| b.value == Some('b')));
}

#[test]
fn contains() {
    let mut trie = Trie::new(' ');
    trie.add("abcde");
    trie.add("abxyz");

    assert!(trie.contains("abcde"));
    assert!(trie.contains("abxyz"));
}

#[test]
fn has_word() {
    let mut trie = Trie::new(' ');

    trie.add("country");
    assert!(trie.has_word("country"));

    trie.add("county");
    assert!(trie.has_word("county"));

    assert!(!trie.has_word("count"));    
}

#[test]
fn get_word() {
    let mut trie = Trie::new(' ');

    let y_node = Trie {
        is_word: true,
        children: vec![],
        value: Some('y'),
    };

    let word = "country";
    trie.add(word);
    let res = trie.get_word(word);

    assert_eq!(res, Some(y_node).as_mut());
}

