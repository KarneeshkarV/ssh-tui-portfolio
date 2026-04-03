use ratatui::{
    prelude::*,
    style::Color,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
pub const BORDER_ACCENT: Color = Color::Rgb(75, 85, 110);

/// Linearly interpolate between two RGB colors. `t` is clamped to [0.0, 1.0].
/// Returns `from` unchanged if either color is not RGB.
pub fn color_lerp(from: Color, to: Color, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (from, to) {
        let r = (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8;
        let g = (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8;
        let b = (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8;
        Color::Rgb(r, g, b)
    } else {
        from
    }
}

/// Returns a color that gently pulses in brightness around `base`.
/// `tick` is the global tick counter, `period` controls the cycle length in ticks.
/// Oscillates brightness by ~20% using a simple triangle wave (no libm needed).
pub fn pulsing_accent(base: Color, tick: u64, period: u8) -> Color {
    if period == 0 {
        return base;
    }
    let phase = (tick % period as u64) as f64 / period as f64;
    // Triangle wave: 0→1→0 over one period
    let wave = if phase < 0.5 {
        phase * 2.0
    } else {
        2.0 - phase * 2.0
    };
    // Oscillate between 80% and 100% brightness
    let factor = 0.8 + 0.2 * wave;
    if let Color::Rgb(r, g, b) = base {
        let r = (r as f64 * factor).min(255.0) as u8;
        let g = (g as f64 * factor).min(255.0) as u8;
        let b = (b as f64 * factor).min(255.0) as u8;
        Color::Rgb(r, g, b)
    } else {
        base
    }
}

/// Screen names for the tab bar footer.
const SCREEN_NAMES: [&str; 4] = ["Intro", "Skills", "Experience", "Telemetry"];

/// Render a standardized footer with nav keys, page indicator, and screen tab bar.
pub fn render_footer(area: Rect, buf: &mut Buffer, page: usize, total: usize, hint: &str) {
    buf.set_style(area, Style::new().bg(BG_FOOTER));

    // Build tab bar line
    let mut tab_spans: Vec<Span> = Vec::new();
    for (i, name) in SCREEN_NAMES.iter().enumerate() {
        let screen_num = i + 1;
        if screen_num == page {
            tab_spans.push(Span::styled(
                format!(" [{}] ", name),
                Style::new().fg(ACCENT_TEAL).bold(),
            ));
        } else {
            tab_spans.push(Span::styled(
                format!("  {}  ", name),
                Style::new().fg(FG_DIM),
            ));
        }
        if screen_num < total {
            tab_spans.push(Span::styled("·", Style::new().fg(FG_DIM)));
        }
    }

    // Build progress dots
    let mut dot_spans: Vec<Span> = vec![Span::styled("    ", Style::new().fg(FG_DIM))];
    for i in 1..=total {
        if i == page {
            dot_spans.push(Span::styled("●", Style::new().fg(ACCENT_TEAL)));
        } else {
            dot_spans.push(Span::styled("○", Style::new().fg(FG_DIM)));
        }
        if i < total {
            dot_spans.push(Span::styled(" ", Style::new().fg(FG_DIM)));
        }
    }

    // Ornamental separator
    let separator = Line::from(vec![Span::styled(
        "──────────── ◆ ────────────",
        Style::new().fg(FG_DIM),
    )]);

    let footer = Paragraph::new(vec![
        separator,
        Line::from(tab_spans),
        Line::from(vec![
            Span::styled("p", Style::new().fg(ACCENT_GOLD).bold()),
            Span::styled(" prev", Style::new().fg(FG_PRIMARY)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled("n", Style::new().fg(ACCENT_GOLD).bold()),
            Span::styled(" next", Style::new().fg(FG_PRIMARY)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled("q", Style::new().fg(ACCENT_RED).bold()),
            Span::styled(" quit", Style::new().fg(FG_PRIMARY)),
            Span::styled("   ", Style::new().fg(FG_DIM)),
            dot_spans[0].clone(),
            dot_spans
                .get(1)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(2)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(3)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(4)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(5)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(6)
                .cloned()
                .unwrap_or(Span::raw("")),
            dot_spans
                .get(7)
                .cloned()
                .unwrap_or(Span::raw("")),
        ]),
        Line::from(Span::styled(hint, Style::new().fg(FG_MUTED).italic())),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::NONE)
            .style(Style::new().bg(BG_FOOTER)),
    );
    footer.render(area, buf);
}
