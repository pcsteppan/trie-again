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