const ROW_1_KEYS: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const ROW_2_KEYS: [char; 9] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
const ROW_3_KEYS: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

use std::char;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};

#[derive(Debug)]
pub enum KeyState {
    Unused,
    NotInWord,
    InWord,
    CorrectIndex,
}

#[derive(Debug)]
struct Key {
    charecter: char,
    state: KeyState,
}

impl Widget for &Key {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let span = match self.state {
            KeyState::Unused => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Black),
            ),
            KeyState::NotInWord => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Gray),
            ),
            KeyState::InWord => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Yellow),
            ),
            KeyState::CorrectIndex => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Green),
            ),
        };
        buf.set_span(area.x, area.y, &span, 1);
    }
}

impl Key {
    fn new(charecter: char) -> Self {
        Self {
            charecter,
            state: KeyState::Unused,
        }
    }
}

pub struct Keyboard {
    row_1: [Key; ROW_1_KEYS.len()],
    row_2: [Key; ROW_2_KEYS.len()],
    row_3: [Key; ROW_3_KEYS.len()],
}

impl Widget for &Keyboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .split(area);

        let row_1_key_areas = Layout::horizontal([Constraint::Length(1); ROW_1_KEYS.len()])
            .flex(Flex::Center)
            .split(rows[0]);
        for (index, key) in self.row_1.iter().enumerate() {
            key.render(row_1_key_areas[index], buf);
        }

        let row_2_key_areas = Layout::horizontal([Constraint::Length(1); ROW_2_KEYS.len()])
            .flex(Flex::Center)
            .split(rows[1]);
        for (index, key) in self.row_2.iter().enumerate() {
            key.render(row_2_key_areas[index], buf);
        }

        let row_3_key_areas = Layout::horizontal([Constraint::Length(1); ROW_3_KEYS.len()])
            .flex(Flex::Center)
            .split(rows[2]);
        for (index, key) in self.row_3.iter().enumerate() {
            key.render(row_3_key_areas[index], buf);
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            row_1: ROW_1_KEYS
                .iter()
                .map(|&charecter| Key::new(charecter))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            row_2: ROW_2_KEYS
                .iter()
                .map(|&charecter| Key::new(charecter))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            row_3: ROW_3_KEYS
                .iter()
                .map(|&charecter| Key::new(charecter))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl Keyboard {
    pub fn set_key_state(&mut self, charecter: char, state: KeyState) {
        if ROW_1_KEYS.contains(&charecter) {
            let index = ROW_1_KEYS.iter().position(|&c| c == charecter).unwrap();
            self.row_1[index].state = state;
        } else if ROW_2_KEYS.contains(&charecter) {
            let index = ROW_2_KEYS.iter().position(|&c| c == charecter).unwrap();
            self.row_2[index].state = state;
        } else {
            let index = ROW_3_KEYS.iter().position(|&c| c == charecter).unwrap();
            self.row_3[index].state = state;
        }
    }
}
