use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use tui_big_text::{BigText, PixelSize};

use super::theme::*;

/// Composite intro view that fills the terminal with hero text, description, and controls.
#[derive(Copy, Clone)]
pub struct AsciiFrame {
    pub label: &'static str,
    pub art: &'static [&'static str],
    pub tagline: &'static str,
    pub accent: Color,
}

pub const ASCII_FRAMES: [AsciiFrame; 2] = [
    AsciiFrame {
        label: "Neovim",
        art: &[
            "        ,l;             c,      ",
            "    .:ooool'           loo:.   ",
            "  .,oooooooo:.         looooc, ",
            " ll:,loooooool,        looooool",
            " llll,;ooooooooc.      looooooo",
            " lllllc,coooooooo;     looooooo",
            " lllllll;,loooooool'   looooooo",
            " lllllllc .:oooooooo:. looooooo",
            " lllllllc   'loooooool,:ooooooo",
            " lllllllc     ;ooooooooc,cooooo",
            " lllllllc      .coooooooo;;looo",
            " lllllllc        ,loooooool,:ol",
            "  'cllllc         .:oooooooo;. ",
            "    .;llc           .loooo:.   ",
            "       ,;             ;l;      ",
        ],
        tagline: "Modal editing keeps the flow alive.",
        accent: ACCENT_TEAL,
    },
    AsciiFrame {
        label: "Linux",
        art: &[
            "       .--.          ",
            "      |o_o |         ",
            "      |:_/ |         ",
            "     //   \\ \\       ",
            "    (|     | )       ",
            "   /'\\_   _/`\\      ",
            "   \\___)=(___/      ",
            "  Powered by Linux   ",
        ],
        tagline: "Freedom-driven systems tinkering.",
        accent: ACCENT_GOLD,
    },
];

pub struct IntroScreenWidget {
    frame: AsciiFrame,
    page: usize,
    total: usize,
    screen_tick: u64,
    global_tick: u64,
}

pub fn intro_screen(
    frame: AsciiFrame,
    page: usize,
    total: usize,
    screen_tick: u64,
    global_tick: u64,
) -> IntroScreenWidget {
    IntroScreenWidget {
        frame,
        page,
        total,
        screen_tick,
        global_tick,
    }
}

impl Widget for IntroScreenWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(BG_CANVAS));

        let content_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        if content_area.width < 20 || content_area.height < 5 {
            return;
        }

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // status bar
                Constraint::Percentage(52), // hero
                Constraint::Percentage(30), // highlights
                Constraint::Length(5),  // footer
            ])
            .split(content_area);

        // ── Status bar ──
        self.render_status_bar(sections[0], buf);

        // ── Hero section ──
        if self.screen_tick >= 2 {
            self.render_hero(sections[1], buf);
        }

        // ── Highlight cards ──
        if self.screen_tick >= 8 {
            self.render_highlights(sections[2], buf);
        }

        // Footer
        render_footer(
            sections[3],
            buf,
            self.page,
            self.total,
            "Optimized for full-screen terminals.",
        );
    }
}

impl IntroScreenWidget {
    fn render_status_bar(&self, area: Rect, buf: &mut Buffer) {
        let progress = (self.screen_tick * 10).min(100) as usize;
        let bar_width = 10;
        let filled = (bar_width * progress) / 100;
        let unfilled = bar_width - filled;

        let bar: String = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(unfilled),
        );

        let status_line = Line::from(vec![
            Span::styled("▊ ", Style::new().fg(ACCENT_TEAL)),
            Span::styled("ssh://karneeshkar.dev", Style::new().fg(FG_SECONDARY)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled("session active", Style::new().fg(ACCENT_GREEN)),
            Span::styled("  ·  ", Style::new().fg(FG_DIM)),
            Span::styled(&bar[..filled * 3], Style::new().fg(ACCENT_TEAL)), // █ is 3 bytes
            Span::styled(&bar[filled * 3..], Style::new().fg(FG_DIM)),
            Span::styled(format!(" {}%", progress), Style::new().fg(FG_MUTED)),
            Span::styled(" ▊", Style::new().fg(ACCENT_TEAL)),
        ]);

        let status = Paragraph::new(vec![status_line])
            .alignment(Alignment::Center)
            .style(Style::new().bg(BG_HERO));
        status.render(area, buf);
    }

    fn render_hero(&self, area: Rect, buf: &mut Buffer) {
        let hero_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        buf.set_style(area, Style::new().bg(BG_HERO));

        // Fade-in effect for hero text (ticks 2-6)
        let fade_progress = if self.screen_tick < 2 {
            0.0
        } else {
            ((self.screen_tick - 2) as f64 / 4.0).min(1.0)
        };

        let teal_faded = color_lerp(FG_DIM, ACCENT_TEAL, fade_progress);
        let blue_faded = color_lerp(FG_DIM, ACCENT_BLUE, fade_progress);
        let gold_faded = color_lerp(FG_DIM, ACCENT_GOLD, fade_progress);

        let hero_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(teal_faded))
            .lines(vec![
                "Karneeshkar".fg(teal_faded).bold().into(),
                "Human".fg(blue_faded).bold().into(),
                "Welcome".fg(gold_faded).into(),
            ])
            .build();
        hero_text.render(hero_chunks[0], buf);

        // Blinking cursor after BigText
        if self.global_tick % 5 < 3 && fade_progress >= 1.0 {
            // Place cursor at a fixed position in the hero area
            let cursor_x = hero_chunks[0].x + 2;
            let cursor_y = hero_chunks[0].y + hero_chunks[0].height.saturating_sub(2);
            if cursor_x < hero_chunks[0].right() && cursor_y < hero_chunks[0].bottom() {
                buf[(cursor_x, cursor_y)].set_char('▌');
                buf[(cursor_x, cursor_y)].set_style(Style::new().fg(ACCENT_TEAL));
            }
        }

        // Right column: about + ASCII art
        if self.screen_tick >= 5 {
            self.render_hero_right(hero_chunks[1], buf);
        }
    }

    fn render_hero_right(&self, area: Rect, buf: &mut Buffer) {
        let frame = self.frame;

        let about_lines = vec![
            Line::from(Span::styled(
                "Developer dedicated to crafting elegant solutions, automation, and systems software.",
                Style::new().fg(FG_PRIMARY).bold(),
            )),
            Line::from(Span::styled(
                "I build accessible interfaces, automation tooling, and systems software.",
                Style::new().fg(FG_SECONDARY),
            )),
            Line::from(Span::styled(
                "Open to interesting collaborations and remote-first teams.",
                Style::new().fg(ACCENT_TEAL),
            )),
        ];

        let about = Paragraph::new(about_lines)
            .block(
                Block::default()
                    .title(Span::styled(
                        "── About Me ──",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_ACCENT))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });

        let ascii_panel_height = if area.height > 7 {
            (frame.art.len() as u16 + 3).min(area.height - 7)
        } else {
            area.height / 2
        };

        let hero_detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1), // spacer
                Constraint::Length(ascii_panel_height),
            ])
            .split(area);

        about.render(hero_detail_chunks[0], buf);

        let mut ascii_lines: Vec<Line> = frame
            .art
            .iter()
            .map(|line| Line::from(Span::styled(*line, Style::new().fg(frame.accent).bold())))
            .collect();
        ascii_lines.push(Line::from(Span::raw("")));
        ascii_lines.push(Line::from(Span::styled(
            frame.tagline,
            Style::new().fg(FG_SECONDARY).italic(),
        )));

        // Pulsing border on ASCII art panel
        let pulsing_border = pulsing_accent(frame.accent, self.global_tick, 10);

        let ascii_panel = Paragraph::new(ascii_lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(Span::styled(
                        frame.label,
                        Style::new().fg(frame.accent).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(pulsing_border))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: false });
        if hero_detail_chunks[2].height > 0 {
            ascii_panel.render(hero_detail_chunks[2], buf);
        }
    }

    fn render_highlights(&self, area: Rect, buf: &mut Buffer) {
        let highlights = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Length(1), // spacer
                Constraint::Percentage(33),
                Constraint::Length(1), // spacer
                Constraint::Percentage(33),
            ])
            .split(area);

        buf.set_style(area, Style::new().bg(BG_SECTION));

        // Staggered card appearance
        let card_tick_offsets = [8u64, 10, 12];

        // Focus Areas (appears first)
        if self.screen_tick >= card_tick_offsets[0] {
            let focus = Paragraph::new(vec![
                Line::from(Span::styled("◆ AI agents", Style::new().fg(ACCENT_TEAL))),
                Line::from(Span::styled(
                    "◆ Native Applications",
                    Style::new().fg(ACCENT_BLUE),
                )),
                Line::from(Span::styled(
                    "◆ Embedded Systems",
                    Style::new().fg(ACCENT_VIOLET),
                )),
                Line::from(Span::styled("◇ Terminal UX", Style::new().fg(FG_PRIMARY))),
                Line::from(Span::styled(
                    "◇ Cloud automation",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "◇ Developer tooling",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "◇ And much more",
                    Style::new().fg(FG_SECONDARY),
                )),
            ])
            .block(
                Block::default()
                    .title(Span::styled(
                        " ◆ Focus Areas ",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_DIM))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .alignment(Alignment::Left);
            focus.render(highlights[0], buf);
        }

        // Toolbox (appears second)
        if self.screen_tick >= card_tick_offsets[1] {
            let toolbox_lines = vec![
                Line::from(Span::styled(
                    "◆ Rust | Go | C++ | Python",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled("◆ NVIM", Style::new().fg(FG_PRIMARY))),
                Line::from(Span::styled("◆ Linux", Style::new().fg(FG_PRIMARY))),
                Line::from(Span::styled(
                    "◇ Terraform | Bash",
                    Style::new().fg(FG_SECONDARY),
                )),
                Line::from(Span::styled(
                    "◇ AWS | GCP | Digital Ocean",
                    Style::new().fg(FG_SECONDARY),
                )),
            ];

            let stack = Paragraph::new(toolbox_lines)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title(Span::styled(
                            " ◆ Toolbox ",
                            Style::new().fg(self.frame.accent).bold(),
                        ))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(BORDER_DIM))
                        .style(Style::new().bg(BG_PANEL)),
                )
                .wrap(Wrap { trim: false });
            stack.render(highlights[2], buf);
        }

        // Connect (appears third)
        if self.screen_tick >= card_tick_offsets[2] {
            let contact = Paragraph::new(vec![
                Line::from(Span::styled(
                    "◆ github.com/KarneeshkarV",
                    Style::new().fg(ACCENT_TEAL),
                )),
                Line::from(Span::styled(
                    "◆ linkedin.com/in/karneeshkar-velmurugan/",
                    Style::new().fg(ACCENT_BLUE),
                )),
                Line::from(Span::styled(
                    "◆ karneeshkar68@gmail.com",
                    Style::new().fg(ACCENT_VIOLET),
                )),
            ])
            .block(
                Block::default()
                    .title(Span::styled(
                        " ◆ Connect ",
                        Style::new().fg(ACCENT_VIOLET).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_DIM))
                    .style(Style::new().bg(BG_PANEL)),
            );
            contact.render(highlights[4], buf);
        }
    }
}
