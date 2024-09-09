use super::{
    letter::{Letter, LetterState},
    words::{VALID_WORDS, WORDS},
    WORD_LENGTH,
};
use anyhow::{anyhow, Result};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Widget,
};

pub struct Guess {
    pub letters: [Letter; WORD_LENGTH],
}

impl Widget for &Guess {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let letter_areas = Layout::horizontal([Constraint::Length(1); WORD_LENGTH])
            .flex(Flex::Center)
            .split(area);

        for (index, letter) in self.letters.iter().enumerate() {
            letter.render(letter_areas[index], buf);
        }
    }
}

impl From<Vec<Letter>> for Guess {
    fn from(value: Vec<Letter>) -> Guess {
        Self {
            letters: value[..5].try_into().unwrap(),
        }
    }
}

impl Guess {
    pub fn new(guess: &str, word: &str) -> Result<Guess> {
        if guess.len() < 5 {
            return Err(anyhow!("Not enough letters"));
        }

        if !WORDS.contains(&guess) && !VALID_WORDS.contains(&guess) {
            return Err(anyhow!("Not in word list"));
        }

        Ok(guess
            .chars()
            .enumerate()
            .map(|(index, charecter)| {
                if word.chars().nth(index).unwrap() == charecter {
                    Letter::new(charecter, LetterState::CorrectIndex)
                } else if word.contains(charecter) {
                    Letter::new(charecter, LetterState::InWord)
                } else {
                    Letter::new(charecter, LetterState::NotInWord)
                }
            })
            .collect::<Vec<Letter>>()
            .into())
    }
}
