use gamestate::{GameState, GameWord};
use trie::Trie;
use rand::prelude::*;

pub struct GameController {
    pub model: Trie,
    pub state: GameState
}

impl GameController {
    pub fn new(trie: Trie) -> Self {
        GameController {
            model: trie,
            state: GameState::init_state()
        }
    }

    pub fn start_random(&mut self) -> &GameState {
        let mut substring_frequencies = self.model
            .get_all_substring_frequencies(3)
            .into_iter()
            .collect::<Vec<(String, u16)>>();
        substring_frequencies.sort_by(|a, b| b.1.cmp(&a.1));
        
        let mut eligible_substrings = &mut substring_frequencies[0..20];
        
        let mut rng = rand::thread_rng();
        eligible_substrings.shuffle(&mut rng);
        let substring_pick = eligible_substrings.first().unwrap();
        let game_words = self.model.get_all_words_with_containing_substring(substring_pick.0.as_str());
        
        self.state = GameState::new_game(substring_pick.0.to_string(), game_words);
        &self.state
    }

    pub fn submit_guess(&mut self, guess: &String) -> bool {
        let prev_completed_count = self.state.words
            .iter()
            .filter(|w| w.completed)
            .collect::<Vec<&GameWord>>()
            .len();

        let mut new_state = self.state.clone();
        let new_words: Vec<GameWord> = self.state.clone().words.into_iter()
            .map(|word| if word.word.eq(guess) { GameWord::new_completed(&word) } else { word })
            .collect();

        new_state.words = new_words;
        self.state = new_state;

        let new_completed_count = self.state.words
            .iter()
            .filter(|w| w.completed)
            .collect::<Vec<&GameWord>>()
            .len();

        new_completed_count > prev_completed_count
    }
}