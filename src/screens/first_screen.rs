use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

pub fn first_screen() -> BigText<'static> {
    BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec![
            "In".green().into(),
            "First".white().into(),
            "Let's see".green().into(),
            "~~~~~".into(),
        ])
        .build()
}
