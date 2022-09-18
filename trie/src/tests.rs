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
    let mut trie = Trie::new_root();
    trie.add("abcde");
    trie.add("abxyz");

    assert!(trie.contains("abcde"));
    assert!(trie.contains("abxyz"));
}

#[test]
fn has_word() {
    let mut trie = Trie::new_root();

    trie.add("country");
    assert!(trie.has_word("country"));

    trie.add("county");
    assert!(trie.has_word("county"));

    assert!(!trie.has_word("count"));    
}

#[test]
fn get_word() {
    let mut trie = Trie::new_root();

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

#[test]
fn get_all_words() {
    let mut trie = Trie::new_root();

    let words_to_add = [
        "aa",
        "ab",
        "abc",
        "ba",
        "bb",
        "bbc"
    ];

    for word in words_to_add {
        trie.add(word);
    }

    let all_words_in_trie = trie.get_all_words();

    for word in all_words_in_trie {
        let word_as_str = word.as_str();
        assert!(words_to_add.contains(&word_as_str));
    }
}

#[test]
fn get_all_substring_frequencies() {
    let mut trie = Trie::new_root();

    let words_to_add = [
        "aa",
        "ab",
        "abc",
        "ba",
        "bb",
        "bbc"
    ];

    for word in words_to_add {
        trie.add(word);
    }

    let dichar_freqs = trie.get_all_substring_frequencies(2);

    assert_eq!(dichar_freqs.get("aa").unwrap_or(&0).to_owned(), 1);
    assert_eq!(dichar_freqs.get("ab").unwrap_or(&0).to_owned(), 2);
    assert_eq!(dichar_freqs.get("ac").unwrap_or(&0).to_owned(), 0);
    assert_eq!(dichar_freqs.get("bc").unwrap_or(&0).to_owned(), 2);
    assert_eq!(dichar_freqs.get("ba").unwrap_or(&0).to_owned(), 1);
    assert_eq!(dichar_freqs.get("bb").unwrap_or(&0).to_owned(), 2);
}

#[test]
fn get_all_words_with_containing_substring() {
    let mut root = Trie::new_root();

    root.add("beast");

    let es_words = vec![
        "test",
        "tester",
        "best",
        "essen",
        "fries"
    ];

    for es_word in es_words.as_slice() {
        root.add(es_word);
    }

    let all_words_with_es_substring = root.get_all_words_with_containing_substring("es");

    for word in es_words {
        let word_as_string = word.to_string();
        assert!(all_words_with_es_substring.contains(&word_as_string));
    }

    let beast = "beast".to_string();
    assert!(!all_words_with_es_substring.contains(&beast));
}