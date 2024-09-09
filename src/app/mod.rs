mod guess;
mod keyboard;
mod letter;
mod words;

use super::tui;
use anyhow::Result;
use guess::Guess;
use keyboard::{KeyState, Keyboard};
use letter::LetterState;
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    text::Span,
    widgets::Widget,
    Frame,
};
use words::WORDS;

const WORD_LENGTH: usize = 5;
const GUESS_COUNT: usize = 6;

#[derive(Default)]
pub struct App<'a> {
    message: String,
    past_gusses: Vec<guess::Guess>,
    current_guess: String,
    keyboard: Keyboard,
    word: &'a str,
    game_over: bool,
    exit: bool,
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [message_area, gusses_area, _, keyboard_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(GUESS_COUNT as u16),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .flex(Flex::Center)
        .areas(area);

        Span::raw(self.message.clone()).render(
            Layout::horizontal([Constraint::Length(self.message.len() as u16)])
                .flex(Flex::Center)
                .split(message_area)[0],
            buf,
        );

        let guess_locations =
            Layout::vertical([Constraint::Length(1); GUESS_COUNT]).split(gusses_area);

        for (index, guess) in self.past_gusses.iter().enumerate() {
            guess.render(guess_locations[index], buf);
        }

        if self.past_gusses.len() < 6 {
            let current_guess_area = Layout::horizontal([Constraint::Length(WORD_LENGTH as u16)])
                .flex(Flex::Center)
                .split(guess_locations[self.past_gusses.len()])[0];
            self.current_guess.clone().render(current_guess_area, buf);
        }

        self.keyboard.render(keyboard_area, buf);
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            word: *WORDS.choose(&mut rng).unwrap(),
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if self.game_over {
                self.exit = true;
            }

            self.message.clear();

            match key.code {
                KeyCode::Char(ch) => {
                    if self.current_guess.len() < 5 && ch.is_ascii_alphabetic() {
                        self.current_guess.push(ch.to_ascii_uppercase());
                    }
                }
                KeyCode::Enter => match Guess::new(&self.current_guess, self.word) {
                    Ok(guess) => {
                        for letter in guess.letters {
                            match letter.state {
                                LetterState::CorrectIndex => {
                                    self.keyboard
                                        .set_key_state(letter.charecter, KeyState::CorrectIndex);
                                }
                                LetterState::InWord => {
                                    self.keyboard
                                        .set_key_state(letter.charecter, KeyState::InWord);
                                }
                                LetterState::NotInWord => {
                                    self.keyboard
                                        .set_key_state(letter.charecter, KeyState::NotInWord);
                                }
                            }
                        }

                        self.past_gusses.push(guess);
                        self.current_guess.clear();

                        if self.past_gusses[self.past_gusses.len() - 1]
                            .letters
                            .iter()
                            .all(|letter| letter.state == LetterState::CorrectIndex)
                        {
                            self.message = "You win!".to_string();
                            self.game_over = true;
                        }

                        if self.past_gusses.len() == GUESS_COUNT {
                            self.message = self.word.to_string();
                            self.game_over = true;
                        }
                    }
                    Err(error) => {
                        self.message = error.to_string();
                    }
                },
                KeyCode::Backspace => {
                    self.current_guess.pop();
                }
                KeyCode::Esc => {
                    self.exit = true;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
