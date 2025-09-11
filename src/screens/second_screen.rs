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

pub fn second_screen() -> BigText<'static> {
    BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().red())
        .lines(vec![
            "Second".yellow().into(),
            "Screen".white().into(),
            "Awesome!".green().into(),
            "#####".into(),
        ])
        .build()
}
