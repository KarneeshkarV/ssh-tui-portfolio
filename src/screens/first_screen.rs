use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use super::theme::*;

pub struct FirstScreenWidget {
    call_sign: String,
}

pub fn first_screen(call_sign: &str) -> FirstScreenWidget {
    FirstScreenWidget {
        call_sign: call_sign.to_string(),
    }
}

impl Widget for FirstScreenWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(BG_CANVAS));

        let content = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        if content.width < 40 || content.height < 12 {
            return;
        }

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Min(11),
                Constraint::Length(4),
            ])
            .split(content);

        let header = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("Karneeshkar V · {}", self.call_sign),
                Style::new().fg(ACCENT_GOLD).bold(),
            )),
            Line::from(Span::styled(
                "Software engineer crafting cloud-ready systems, AI automations, and human-centered tools.",
                Style::new().fg(FG_PRIMARY),
            )),
            Line::from(vec![
                Span::styled("Email:", Style::new().fg(FG_SECONDARY)),
                Span::styled(" karneeshkar01@gmail.com", Style::new().fg(ACCENT_TEAL).bold()),
            ]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(
                    "Welcome Aboard",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_HERO)),
        )
        .wrap(Wrap { trim: true });
        header.render(sections[0], buf);

        buf.set_style(sections[1], Style::new().bg(BG_SECTION));
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(56), Constraint::Percentage(44)])
            .split(sections[1]);

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(columns[0]);

        let expertise_items = vec![
            ListItem::new(vec![
                Line::from(Span::styled(
                    "Backend Development",
                    Style::new().fg(ACCENT_TEAL).bold(),
                )),
                Line::from(Span::styled(
                    "REST APIs · FastAPI · Axum · Node.js · PHP",
                    Style::new().fg(FG_PRIMARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(Span::styled(
                    "AI & Automation",
                    Style::new().fg(ACCENT_BLUE).bold(),
                )),
                Line::from(Span::styled(
                    "LLMs · Retrieval-Augmented Generation · Function Calling · Workflow Automation",
                    Style::new().fg(FG_PRIMARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(Span::styled(
                    "Cloud Solutions",
                    Style::new().fg(ACCENT_VIOLET).bold(),
                )),
                Line::from(Span::styled(
                    "AWS · DigitalOcean · Azure · Docker",
                    Style::new().fg(FG_PRIMARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(Span::styled(
                    "Frontend Development",
                    Style::new().fg(ACCENT_GOLD).bold(),
                )),
                Line::from(Span::styled(
                    "React · JavaScript · Responsive UI · UX Design",
                    Style::new().fg(FG_PRIMARY),
                )),
            ]),
        ];

        let expertise = List::new(expertise_items)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Software Development Expertise",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .highlight_style(Style::new().fg(ACCENT_GOLD));
        ratatui::widgets::Widget::render(expertise, left_layout[0], buf);

        let about = Paragraph::new(vec![
            Line::from(Span::styled(
                "Passionate engineer building scalable, user-friendly systems with measurable impact.",
                Style::new().fg(FG_PRIMARY),
            )),
            Line::from(Span::styled(
                "Blends embedded know-how, cloud ops, and AI to ship resilient experiences.",
                Style::new().fg(FG_SECONDARY),
            )),
        ])
        .block(
            Block::default()
                .title(Span::styled(
                    "About Me",
                    Style::new().fg(ACCENT_BLUE).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        about.render(left_layout[1], buf);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(columns[1]);

        let skill_items = vec![
            ListItem::new(vec![Line::from(vec![
                Span::styled("C++", Style::new().fg(ACCENT_TEAL).bold()),
                Span::styled(" 90%", Style::new().fg(FG_PRIMARY)),
            ])]),
            ListItem::new(vec![Line::from(vec![
                Span::styled("Python", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(" 85%", Style::new().fg(FG_PRIMARY)),
            ])]),
            ListItem::new(vec![Line::from(vec![
                Span::styled("Rust", Style::new().fg(ACCENT_VIOLET).bold()),
                Span::styled(" 80%", Style::new().fg(FG_PRIMARY)),
            ])]),
            ListItem::new(vec![Line::from(vec![
                Span::styled("JavaScript", Style::new().fg(ACCENT_BLUE).bold()),
                Span::styled(" 75%", Style::new().fg(FG_PRIMARY)),
            ])]),
        ];

        let skills = List::new(skill_items)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Technical Skills",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .highlight_style(Style::new().fg(ACCENT_TEAL));
        ratatui::widgets::Widget::render(skills, right_layout[0], buf);

        let connect = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Hire me", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(
                    " → Reach out for backend, AI, or IoT builds.",
                    Style::new().fg(FG_PRIMARY),
                ),
            ]),
            Line::from(vec![
                Span::styled("Resume:", Style::new().fg(FG_SECONDARY)),
                Span::styled(
                    " Request via karneeshkar01@gmail.com",
                    Style::new().fg(FG_PRIMARY),
                ),
            ]),
            Line::from(Span::styled(
                "Available for collaborations, product engineering, and automation engagements.",
                Style::new().fg(FG_SECONDARY),
            )),
        ])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title(Span::styled(
                    "Reach Out",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        connect.render(right_layout[1], buf);

        let footer = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Navigate", Style::new().fg(FG_SECONDARY)),
                Span::raw("  "),
                Span::styled("p", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(" prev", Style::new().fg(FG_PRIMARY)),
                Span::styled("  •  ", Style::new().fg(FG_SECONDARY)),
                Span::styled("n", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(" next", Style::new().fg(FG_PRIMARY)),
                Span::styled("  •  ", Style::new().fg(FG_SECONDARY)),
                Span::styled("q", Style::new().fg(Color::Rgb(244, 105, 130)).bold()),
                Span::styled(" quit", Style::new().fg(FG_PRIMARY)),
            ]),
            Line::from(Span::styled(
                "n for experience & projects",
                Style::new().fg(FG_MUTED).italic(),
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
