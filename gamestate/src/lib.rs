/*
used words
substring
current words : GameWord
*/


#[derive(Clone)]
pub struct GameState {
    pub substring: Option<String>,
    pub words: Vec<GameWord>,
    pub view: View
}

impl GameState {
    pub fn new_game(substring: String, words: Vec<String>) -> Self {
        GameState {
            substring: Some(substring.clone()),
            words: words
                .iter()
                .map(|word| GameWord::new(word.to_string(), substring.clone()))
                .collect(),
            view: View::PLAY
        }
    }

    pub fn init_state() -> Self {
        GameState {
            substring: None,
            words: Vec::new(),
            view: View::MENU
        }
    }
}

#[derive(Clone)]
pub struct GameWord {
    pub word: String,
    pub masked_word: String,
    pub completed: bool,
}

impl GameWord {
    fn new(word: String, substring: String) -> Self {
        GameWord {
            word: word.clone(),
            masked_word: Self::mask(word, substring),
            completed: false
        }
    }

    fn mask(word: String, substring: String) -> String {
        let mut full_mask = "_".repeat(word.len()-substring.len());
        let substring_index = word.find(&substring).unwrap();
        full_mask.insert_str(substring_index, substring.as_str());
        full_mask
    }

    pub fn new_completed(&self) -> GameWord {
        GameWord {
            word: self.word.to_string(),
            masked_word: self.masked_word.to_string(),
            completed: true
        }
    }
}

#[derive(Clone)]
pub enum View {
    MENU,
    PLAY
}


/* GameSettings
difficulty -> word_list used
word_length
mixed_word_length
*/