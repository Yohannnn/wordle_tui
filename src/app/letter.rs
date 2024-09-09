use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LetterState {
    NotInWord,
    InWord,
    CorrectIndex,
}

#[derive(Copy, Clone, Debug)]
pub struct Letter {
    pub charecter: char,
    pub state: LetterState,
}

impl Widget for &Letter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let span = match self.state {
            LetterState::NotInWord => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Gray),
            ),
            LetterState::InWord => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Yellow),
            ),
            LetterState::CorrectIndex => Span::styled(
                self.charecter.to_string(),
                Style::default().fg(Color::White).bg(Color::Green),
            ),
        };
        buf.set_span(area.x, area.y, &span, 1);
    }
}

impl Letter {
    pub fn new(charecter: char, state: LetterState) -> Self {
        Self {
            charecter,
            state,
        }
    }
}