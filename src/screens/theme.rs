use ratatui::{
    prelude::*,
    style::Color,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub const BG_CANVAS: Color = Color::Rgb(13, 17, 23); // Darker, github-dimmed like
pub const BG_HERO: Color = Color::Rgb(22, 27, 34);
pub const BG_SECTION: Color = Color::Rgb(18, 22, 29);
pub const BG_PANEL: Color = Color::Rgb(25, 30, 39);
pub const BG_FOOTER: Color = Color::Rgb(10, 13, 18);
pub const BG_CARD: Color = Color::Rgb(20, 25, 33);

pub const ACCENT_TEAL: Color = Color::Rgb(45, 212, 191); // Teal-400
pub const ACCENT_BLUE: Color = Color::Rgb(56, 189, 248); // Sky-400
pub const ACCENT_VIOLET: Color = Color::Rgb(167, 139, 250); // Violet-400
pub const ACCENT_GOLD: Color = Color::Rgb(251, 191, 36); // Amber-400
pub const ACCENT_RED: Color = Color::Rgb(248, 113, 113); // Red-400
pub const ACCENT_GREEN: Color = Color::Rgb(74, 222, 128);

pub const FG_PRIMARY: Color = Color::Rgb(229, 231, 235); // Gray-200
pub const FG_SECONDARY: Color = Color::Rgb(156, 163, 175); // Gray-400
pub const FG_MUTED: Color = Color::Rgb(107, 114, 128); // Gray-500
pub const FG_DIM: Color = Color::Rgb(75, 85, 99);

pub const BORDER_DIM: Color = Color::Rgb(55, 65, 81);

/// Render a standardized footer with nav keys and page indicator.
pub fn render_footer(area: Rect, buf: &mut Buffer, page: usize, total: usize, hint: &str) {
    buf.set_style(area, Style::new().bg(BG_FOOTER));
    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("p", Style::new().fg(ACCENT_GOLD).bold()),
            Span::styled(" prev", Style::new().fg(FG_PRIMARY)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled("n", Style::new().fg(ACCENT_GOLD).bold()),
            Span::styled(" next", Style::new().fg(FG_PRIMARY)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled("q", Style::new().fg(ACCENT_RED).bold()),
            Span::styled(" quit", Style::new().fg(FG_PRIMARY)),
            Span::styled("    ", Style::new().fg(FG_DIM)),
            Span::styled(
                format!("{}/{}", page, total),
                Style::new().fg(ACCENT_BLUE).bold(),
            ),
        ]),
        Line::from(Span::styled(hint, Style::new().fg(FG_MUTED).italic())),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(BORDER_DIM))
            .style(Style::new().bg(BG_FOOTER)),
    );
    footer.render(area, buf);
}
