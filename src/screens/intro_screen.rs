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
}

pub fn intro_screen(frame: AsciiFrame) -> IntroScreenWidget {
    IntroScreenWidget { frame }
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
                Constraint::Percentage(55),
                Constraint::Percentage(30),
                Constraint::Percentage(15),
            ])
            .split(content_area);

        // Hero section with oversized name on the left and a short intro on the right.
        let hero_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(sections[0]);

        buf.set_style(sections[0], Style::new().bg(BG_HERO));

        let hero_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(ACCENT_TEAL))
            .lines(vec![
                "Karneeshkar".fg(ACCENT_TEAL).bold().into(),
                "Human".fg(ACCENT_BLUE).bold().into(),
                "Welcome".fg(ACCENT_GOLD).into(),
            ])
            .build();
        hero_text.render(hero_chunks[0], buf);

        let frame = self.frame;

        let about_lines = vec![
            Line::from(Span::styled(
                "Developer dedicated to crafting elegant solutions, automation, and systems software.",
                Style::new().fg(Color::Rgb(189, 198, 216)).bold(),
            )),
            Line::from(Span::styled(
                "I build accessible interfaces, automation tooling, and systems software.",
                Style::new().fg(Color::Rgb(160, 175, 197)),
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
                        "About Me",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });
        let ascii_panel_height = if hero_chunks[1].height > 6 {
            (frame.art.len() as u16 + 3).min(hero_chunks[1].height - 6)
        } else {
            hero_chunks[1].height / 2
        };

        let hero_detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(ascii_panel_height)])
            .split(hero_chunks[1]);

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
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: false });
        if hero_detail_chunks[1].height > 0 {
            ascii_panel.render(hero_detail_chunks[1], buf);
        }

        // Highlight cards to showcase focus areas and stack.
        let highlights = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(sections[1]);

        buf.set_style(sections[1], Style::new().bg(BG_SECTION));

        let focus = Paragraph::new(vec![
            Line::from(Span::styled("• AI agents", Style::new().fg(ACCENT_TEAL))),
            Line::from(Span::styled(
                "• Native Applications",
                Style::new().fg(ACCENT_BLUE),
            )),
            Line::from(Span::styled(
                "• Embedded Systems",
                Style::new().fg(ACCENT_VIOLET),
            )),
            Line::from(Span::styled(
                "• Terminal UX",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• Cloud automation",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• Developer tooling",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• And much more ",
                Style::new().fg(Color::Rgb(185, 196, 215)),
            )),
        ])
        .block(
            Block::default()
                .title(Span::styled(
                    "Focus Areas",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL)),
        )
        .alignment(Alignment::Left);
        focus.render(highlights[0], buf);

        let toolbox_lines = vec![
            Line::from(Span::styled(
                "• Rust | Go | C++ | Python",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• NVIM",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• Linux",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• Terraform | Bash",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
            Line::from(Span::styled(
                "• AWS | GCP | Digital Ocean",
                Style::new().fg(Color::Rgb(214, 221, 237)),
            )),
        ];

        let stack = Paragraph::new(toolbox_lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(Span::styled(
                        format!("Toolbox ",),
                        Style::new().fg(frame.accent).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: false });
        stack.render(highlights[1], buf);

        let contact = Paragraph::new(vec![
            Line::from(Span::styled(
                "• github.com/KarneeshkarV",
                Style::new().fg(ACCENT_TEAL),
            )),
            Line::from(Span::styled(
                "• linkedin.com/in/karneeshkar-velmurugan/",
                Style::new().fg(ACCENT_BLUE),
            )),
            Line::from(Span::styled(
                "• karneeshkar68@gmail.com",
                Style::new().fg(ACCENT_VIOLET),
            )),
        ])
        .block(
            Block::default()
                .title(Span::styled(
                    "Connect",
                    Style::new().fg(ACCENT_VIOLET).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL)),
        );
        contact.render(highlights[2], buf);

        // Footer instructions for navigation.
        buf.set_style(sections[2], Style::new().bg(BG_FOOTER));
        let footer = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Press ", Style::new().fg(Color::Rgb(150, 163, 186))),
                Span::styled("n", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(
                    " to explore projects, ",
                    Style::new().fg(Color::Rgb(150, 163, 186)),
                ),
                Span::styled("p", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(" to revisit, ", Style::new().fg(Color::Rgb(150, 163, 186))),
                Span::styled("q", Style::new().fg(Color::Rgb(244, 105, 130)).bold()),
                Span::styled(" to exit.", Style::new().fg(Color::Rgb(150, 163, 186))),
            ]),
            Line::from(Span::styled(
                "Optimized for full-screen terminals.",
                Style::new().fg(Color::Rgb(111, 123, 143)).italic(),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_FOOTER)),
        );
        footer.render(sections[2], buf);
    }
}
