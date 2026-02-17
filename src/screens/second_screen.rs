use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use super::theme::*;

pub struct SecondScreenWidget {
    call_sign: String,
    page: usize,
    total: usize,
}

pub fn second_screen(call_sign: &str, page: usize, total: usize) -> SecondScreenWidget {
    SecondScreenWidget {
        call_sign: call_sign.to_string(),
        page,
        total,
    }
}

impl Widget for SecondScreenWidget {
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
                Constraint::Length(6),
                Constraint::Min(11),
                Constraint::Length(4),
            ])
            .split(content);

        let header = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("Experience & Projects · {}", self.call_sign),
                Style::new().fg(ACCENT_GOLD).bold(),
            )),
            Line::from(Span::styled(
                "Shipping resilient software across automotive, cloud, and AI ecosystems.",
                Style::new().fg(FG_PRIMARY),
            )),
            Line::from(Span::styled(
                "Latest role: Tech Lead @ 2Cents Capital (July 2025 - Present).",
                Style::new().fg(FG_SECONDARY),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(
                    "── Mission Log ──",
                    Style::new().fg(ACCENT_VIOLET).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(BORDER_DIM))
                .style(Style::new().bg(BG_HERO)),
        )
        .wrap(Wrap { trim: true });
        header.render(sections[0], buf);

        buf.set_style(sections[1], Style::new().bg(BG_SECTION));
        let main_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(59),
                Constraint::Length(1), // spacer
                Constraint::Percentage(41),
            ])
            .split(sections[1]);

        // Timeline-style experience entries
        let experience_data: Vec<(&str, &str, &str, &str, &str, Color)> = vec![
            (
                "2Cents Capital",
                "Tech Lead",
                "July 2025 - Present",
                "Led and scaled cross-functional Mobile and AI teams (10+ engineers) to deliver high-impact features with 95% on-time release rate.",
                "Published a fully responsive investment application on both iOS App Store and Google Play, achieving <0.5s cold-start time.",
                ACCENT_GOLD,
            ),
            (
                "Visteon",
                "Embedded Systems Developer",
                "Jan 2025 - Present",
                "Optimized real-time automobile dashboards, cutting processing latency by 25%.",
                "Engineered fault-tolerant microcontroller code and 95% accurate diagnostics.",
                ACCENT_GOLD,
            ),
            (
                "UpWork",
                "Freelance Developer",
                "Nov 2024 - Jan 2025",
                "Delivered AI-powered RAG legal solutions with 92% accuracy and IoT firmware boosts.",
                "Completed 5+ engagements maintaining a 4.9/5 satisfaction score.",
                ACCENT_TEAL,
            ),
            (
                "Procter & Gamble",
                "IoT Developer & Automation Intern",
                "Dec 2024 - Jan 2025",
                "Rolled out IIoT monitoring across two lines capturing 200K+ daily data points.",
                "Automated quality checks trimming manual inspection time by 15%.",
                ACCENT_BLUE,
            ),
            (
                "Intellect Design Arena Ltd",
                "Software Intern",
                "Sept 2024 - Dec 2024",
                "Built PHP-based financial sites with 10% faster loads and managed AWS uptime to 99.9%.",
                "Crafted analytics dashboards for real-time engagement insights.",
                ACCENT_VIOLET,
            ),
            (
                "TYNATECH Ingenious Pvt Ltd",
                "IoT & LoRaWAN Developer",
                "May 2024 - Jun 2024",
                "Integrated 10+ LoRaWAN devices via REST APIs and MQTT visualizations handling 20 msg/s.",
                "Boosted UART-LoRaWAN throughput by 10% through buffer tuning.",
                ACCENT_GOLD,
            ),
            (
                "Hindustan Aeronautics Limited",
                "R&D Intern",
                "July 2024 - Aug 2024",
                "Optimized autopilot algorithms reducing computational overhead by 8%.",
                "Delivered C++ tooling validating 40+ control system scenarios.",
                ACCENT_TEAL,
            ),
        ];

        let entry_count = experience_data.len();
        let mut exp_lines: Vec<Line> = Vec::new();

        for (i, (company, role, date, desc1, desc2, accent)) in experience_data.iter().enumerate() {
            let is_last = i == entry_count - 1;
            let marker = if is_last { "◆──" } else { "●──" };
            let cont = if is_last { "   " } else { "│  " };

            // Company + role line
            exp_lines.push(Line::from(vec![
                Span::styled(marker, Style::new().fg(*accent)),
                Span::styled(format!(" {}", company), Style::new().fg(*accent).bold()),
                Span::styled(format!(" · {}", role), Style::new().fg(FG_PRIMARY).bold()),
            ]));
            // Date line
            exp_lines.push(Line::from(vec![
                Span::styled(cont, Style::new().fg(FG_DIM)),
                Span::styled(format!(" {}", date), Style::new().fg(ACCENT_BLUE)),
            ]));
            // Description lines
            exp_lines.push(Line::from(vec![
                Span::styled(cont, Style::new().fg(FG_DIM)),
                Span::styled(format!(" {}", desc1), Style::new().fg(FG_PRIMARY)),
            ]));
            exp_lines.push(Line::from(vec![
                Span::styled(cont, Style::new().fg(FG_DIM)),
                Span::styled(format!(" {}", desc2), Style::new().fg(FG_SECONDARY)),
            ]));
            // Blank separator line between entries
            if !is_last {
                exp_lines.push(Line::from(Span::styled("│", Style::new().fg(FG_DIM))));
            }
        }

        let experience = Paragraph::new(exp_lines)
            .block(
                Block::default()
                    .title(Span::styled(
                        "── Professional Experience ──",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_DIM))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });
        experience.render(main_split[0], buf);

        let right_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(78),
                Constraint::Length(1), // spacer
                Constraint::Percentage(22),
            ])
            .split(main_split[2]);

        // Project cards with separators
        let project_data: Vec<(&str, &str, &str, &str, Color)> = vec![
            (
                "Fine-Tuning LLaMA 3.1 8B for Code Reasoning",
                "Apr 2025",
                "Applied GRPO reinforcement learning with Gemma3 rewards to elevate code reasoning; released model & dataset.",
                "LLaMA 3.1 8B · Gemma3 · Python · Hugging Face · DigitalOcean",
                ACCENT_GOLD,
            ),
            (
                "Multipurpose VR Gloves with Gesture Recognition",
                "Sept 2024",
                "Captured 15+ gestures with 92% accuracy and sub-20ms wireless latency for immersive control.",
                "Unity · C# · Python · ESP32 · Flex Sensors · IMU",
                ACCENT_TEAL,
            ),
            (
                "Instagram Automation Bot",
                "Dec 2024",
                "Handled 1000+ daily interactions with 99.5% uptime, using LLM sentiment analysis at 88% accuracy.",
                "Python · Meta Graph API · FastAPI · Redis · GPT Models · AWS",
                ACCENT_VIOLET,
            ),
            (
                "Smart Rhesus Macaque Deterrent System",
                "Mar 2024",
                "Deployed edge vision alerts with 94% detection accuracy, cutting campus intrusions by 90%.",
                "Raspberry Pi · Jetson Nano · PyTorch · Computer Vision · Ultrasonic Emitters",
                ACCENT_BLUE,
            ),
            (
                "Wearable Jump Height Measurement Device",
                "Mar 2024",
                "Achieved ±1 cm accuracy with Kalman-filtered motion data and actionable training analytics.",
                "ESP32 · MPU6050 · Flask · SQLite",
                ACCENT_GOLD,
            ),
        ];

        let mut proj_lines: Vec<Line> = Vec::new();
        let proj_count = project_data.len();

        for (i, (name, date, desc, stack, accent)) in project_data.iter().enumerate() {
            proj_lines.push(Line::from(vec![
                Span::styled("◆ ", Style::new().fg(*accent)),
                Span::styled(*name, Style::new().fg(*accent).bold()),
                Span::styled(format!("  · {}", date), Style::new().fg(FG_SECONDARY)),
            ]));
            proj_lines.push(Line::from(Span::styled(*desc, Style::new().fg(FG_PRIMARY))));
            proj_lines.push(Line::from(vec![
                Span::styled("╰ ", Style::new().fg(FG_DIM)),
                Span::styled(*stack, Style::new().fg(FG_MUTED).italic()),
            ]));
            if i < proj_count - 1 {
                proj_lines.push(Line::from(Span::styled("───", Style::new().fg(FG_DIM))));
            }
        }

        let projects = Paragraph::new(proj_lines)
            .block(
                Block::default()
                    .title(Span::styled(
                        "── Highlighted Projects ──",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(BORDER_DIM))
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });
        projects.render(right_column[0], buf);

        let contact = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("◆ Let's collaborate", Style::new().fg(ACCENT_TEAL).bold()),
                Span::styled(" — karneeshkar01@gmail.com", Style::new().fg(FG_PRIMARY)),
            ]),
            Line::from(Span::styled(
                "Open to backend, AI, and embedded engagements. Resume available on request.",
                Style::new().fg(FG_SECONDARY).italic(),
            )),
        ])
        .block(
            Block::default()
                .title(Span::styled(
                    "── Contact ──",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(BORDER_DIM))
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        contact.render(right_column[2], buf);

        render_footer(
            sections[2],
            buf,
            self.page,
            self.total,
            "Email karneeshkar01@gmail.com for resume and collaborations.",
        );
    }
}
