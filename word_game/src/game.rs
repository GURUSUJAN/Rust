use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Time's up! The word was '{0}'.")]
    TimeUp(String),
    #[error("Invalid guess length. Expected {expected}, got {actual}.")]
    InvalidGuessLength { expected: usize, actual: usize },
    #[error("Game has already ended.")]
    GameEnded,
}

pub struct GameState {
    secret_word: String,
    time_limit: Duration,
    start_time: Option<Instant>,
    attempts_left: u32,
    game_over: bool,
}

impl GameState {
    pub fn new(secret_word: String, time_limit: Duration, attempts: u32) -> Self {
        GameState {
            secret_word,
            time_limit,
            start_time: None,
            attempts_left: attempts,
            game_over: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn is_time_up(&self) -> bool {
        if let Some(start) = self.start_time {
            start.elapsed() >= self.time_limit
        } else {
            false
        }
    }

    pub fn process_guess(&mut self, guess: &str) -> Result<usize, GameError> {
        if self.game_over {
            return Err(GameError::GameEnded);
        }

        if self.is_time_up() {
            self.game_over = true;
            return Err(GameError::TimeUp(self.secret_word.clone()));
        }

        if guess.len() != self.secret_word.len() {
            return Err(GameError::InvalidGuessLength {
                expected: self.secret_word.len(),
                actual: guess.len(),
            });
        }

        self.attempts_left -= 1;

        let matches = guess.chars()
            .zip(self.secret_word.chars())
            .filter(|(g, s)| g == s)
            .count();

        if matches == self.secret_word.len() {
            self.game_over = true;
        }

        Ok(matches)
    }

    pub fn has_ended(&self) -> bool {
        self.game_over || self.attempts_left == 0
    }

    pub fn get_secret_word(&self) -> &str {
        &self.secret_word
    }

    pub fn get_attempts_left(&self) -> u32 {
        self.attempts_left
    }
}
