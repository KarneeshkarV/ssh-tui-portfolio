use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use super::theme::*;

pub struct FirstScreenWidget {
    call_sign: String,
    page: usize,
    total: usize,
    screen_tick: u64,
}

pub fn first_screen(
    call_sign: &str,
    page: usize,
    total: usize,
    screen_tick: u64,
) -> FirstScreenWidget {
    FirstScreenWidget {
        call_sign: call_sign.to_string(),
        page,
        total,
        screen_tick,
    }
}

/// Render a single skill gauge line directly into the buffer with gradient fill.
fn render_skill_gauge(
    buf: &mut Buffer,
    area: Rect,
    name: &str,
    pct: u16,
    accent: Color,
    y_offset: u16,
    screen_tick: u64,
) {
    if area.y + y_offset + 1 >= area.y + area.height {
        return;
    }

    let label_y = area.y + y_offset;
    let bar_y = area.y + y_offset + 1;
    let inner_x = area.x + 1;
    let inner_w = area.width.saturating_sub(2);

    if inner_w < 10 {
        return;
    }

    // Animate fill from 0% to target over 15 ticks
    let animated_pct = if screen_tick < 15 {
        (pct as u64 * screen_tick / 15) as u16
    } else {
        pct
    };

    // Label line: ◆ name ····· pct%
    let pct_str = format!("{}%", animated_pct);
    let prefix = format!("◆ {} ", name);
    let dots_len = inner_w as usize - prefix.len() - pct_str.len();

    let mut x = inner_x;
    for ch in prefix.chars() {
        if x < area.x + area.width {
            buf[(x, label_y)].set_char(ch);
            buf[(x, label_y)].set_style(Style::new().fg(accent).bold());
            x += 1;
        }
    }
    for _ in 0..dots_len {
        if x < area.x + area.width {
            buf[(x, label_y)].set_char('·');
            buf[(x, label_y)].set_style(Style::new().fg(FG_DIM));
            x += 1;
        }
    }
    for ch in pct_str.chars() {
        if x < area.x + area.width {
            buf[(x, label_y)].set_char(ch);
            buf[(x, label_y)].set_style(Style::new().fg(FG_PRIMARY));
            x += 1;
        }
    }

    // Bar line: filled ━ with gradient + unfilled ─
    let bar_w = inner_w as usize;
    let filled = (bar_w * animated_pct as usize) / 100;
    let unfilled = bar_w - filled;

    x = inner_x;
    for i in 0..filled {
        if x < area.x + area.width {
            // Gradient: dim → accent across the bar
            let progress = if filled > 1 {
                i as f64 / (filled - 1) as f64
            } else {
                1.0
            };
            let grad_color = color_lerp(FG_DIM, accent, progress);
            buf[(x, bar_y)].set_char('━');
            buf[(x, bar_y)].set_style(Style::new().fg(grad_color));
            x += 1;
        }
    }
    for _ in 0..unfilled {
        if x < area.x + area.width {
            buf[(x, bar_y)].set_char('─');
            buf[(x, bar_y)].set_style(Style::new().fg(FG_DIM));
            x += 1;
        }
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
                Constraint::Length(5),
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
                Span::styled(
                    " karneeshkar01@gmail.com",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ),
            ]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(
                    "── Welcome Aboard ──",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(BORDER_ACCENT))
                .style(Style::new().bg(BG_HERO)),
        )
        .wrap(Wrap { trim: true });
        header.render(sections[0], buf);

        buf.set_style(sections[1], Style::new().bg(BG_SECTION));
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Length(1), // spacer
                Constraint::Percentage(45),
            ])
            .split(sections[1]);

        // Left column (stagger: appears at tick 2)
        if self.screen_tick >= 2 {
            self.render_left_column(columns[0], buf);
        }

        // Right column (stagger: appears at tick 6)
        if self.screen_tick >= 6 {
            self.render_right_column(columns[2], buf);
        }

        render_footer(
            sections[2],
            buf,
            self.page,
            self.total,
            "n for experience & projects",
        );
    }
}

impl FirstScreenWidget {
    fn render_left_column(&self, area: Rect, buf: &mut Buffer) {
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(58),
                Constraint::Length(1), // spacer
                Constraint::Percentage(42),
            ])
            .split(area);

        // Expertise items with styled dividers
        let expertise_data = [
            (
                "Backend Development",
                "REST APIs · FastAPI · Axum · Node.js · PHP",
                ACCENT_TEAL,
                false,
            ),
            (
                "AI & Automation",
                "LLMs · RAG · Function Calling · Workflow Automation",
                ACCENT_BLUE,
                false,
            ),
            (
                "Cloud Solutions",
                "AWS · DigitalOcean · Azure · Docker",
                ACCENT_VIOLET,
                false,
            ),
            (
                "Frontend Development",
                "React · JavaScript · Responsive UI · UX Design",
                ACCENT_GOLD,
                true,
            ),
        ];

        let mut expertise_lines: Vec<Line> = Vec::new();
        for (title, desc, accent, is_last) in &expertise_data {
            let connector = if *is_last { "└─" } else { "├─" };
            let cont = if *is_last { "   " } else { "│  " };
            expertise_lines.push(Line::from(vec![
                Span::styled(connector, Style::new().fg(FG_DIM)),
                Span::styled(format!(" {}", title), Style::new().fg(*accent).bold()),
            ]));
            expertise_lines.push(Line::from(vec![
                Span::styled(cont, Style::new().fg(FG_DIM)),
                Span::styled(format!(" {}", desc), Style::new().fg(FG_PRIMARY)),
            ]));
        }

        let expertise = Paragraph::new(expertise_lines)
            .block(
                Block::default()
                    .title(Span::styled(
                        "── Software Development Expertise ──",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_DIM))
                    .padding(Padding::new(1, 1, 0, 0))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });
        expertise.render(left_layout[0], buf);

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
                    "── About Me ──",
                    Style::new().fg(ACCENT_BLUE).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(BORDER_DIM))
                .padding(Padding::new(1, 1, 0, 0))
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        about.render(left_layout[2], buf);
    }

    fn render_right_column(&self, area: Rect, buf: &mut Buffer) {
        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Length(1), // spacer
                Constraint::Percentage(45),
            ])
            .split(area);

        // Skill gauges with highlighted border
        let skills_block = Block::default()
            .title(Span::styled(
                "── Technical Skills ──",
                Style::new().fg(ACCENT_GOLD).bold(),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(BORDER_ACCENT))
            .style(Style::new().bg(BG_PANEL));
        let skills_area = right_layout[0];
        skills_block.render(skills_area, buf);

        let inner = skills_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        let skills = [
            ("C++", 90u16, ACCENT_TEAL),
            ("Python", 85, ACCENT_GOLD),
            ("Rust", 80, ACCENT_VIOLET),
            ("JavaScript", 75, ACCENT_BLUE),
            ("Go", 65, ACCENT_GREEN),
            ("Terraform", 60, ACCENT_RED),
        ];

        for (i, (name, pct, accent)) in skills.iter().enumerate() {
            render_skill_gauge(
                buf,
                inner,
                name,
                *pct,
                *accent,
                (i as u16) * 2,
                self.screen_tick,
            );
        }

        let connect = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("◆ Hire me", Style::new().fg(ACCENT_GOLD).bold()),
                Span::styled(
                    " → Reach out for backend, AI, or IoT builds.",
                    Style::new().fg(FG_PRIMARY),
                ),
            ]),
            Line::from(vec![
                Span::styled("◇ Resume:", Style::new().fg(FG_SECONDARY)),
                Span::styled(
                    " Request via karneeshkar01@gmail.com",
                    Style::new().fg(FG_PRIMARY),
                ),
            ]),
            Line::from(Span::styled(
                "Available for collaborations, product engineering, and automation engagements.",
                Style::new().fg(FG_SECONDARY).italic(),
            )),
        ])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title(Span::styled(
                    "── Reach Out ──",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(BORDER_DIM))
                .padding(Padding::new(1, 1, 0, 0))
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        connect.render(right_layout[2], buf);
    }
}
