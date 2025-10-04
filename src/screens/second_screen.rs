use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use super::theme::*;

pub struct SecondScreenWidget {
    call_sign: String,
}

pub fn second_screen(call_sign: &str) -> SecondScreenWidget {
    SecondScreenWidget {
        call_sign: call_sign.to_string(),
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
                "Latest role: Embedded Systems Developer @ Visteon (Jan 2025 - Present).",
                Style::new().fg(FG_SECONDARY),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(
                    "Mission Log",
                    Style::new().fg(ACCENT_VIOLET).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_HERO)),
        )
        .wrap(Wrap { trim: true });
        header.render(sections[0], buf);

        buf.set_style(sections[1], Style::new().bg(BG_SECTION));
        let main_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(sections[1]);

        let experience_items = vec![
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("Visteon", Style::new().fg(ACCENT_GOLD).bold()),
                    Span::styled(
                        " · Embedded Systems Developer",
                        Style::new().fg(FG_PRIMARY).bold(),
                    ),
                    Span::styled("  (Jan 2025 - Present)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Optimized real-time automobile dashboards, cutting processing latency by 25%.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Engineered fault-tolerant microcontroller code and 95% accurate diagnostics.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("UpWork", Style::new().fg(ACCENT_TEAL).bold()),
                    Span::styled(" · Freelance Developer", Style::new().fg(FG_PRIMARY).bold()),
                    Span::styled("  (Nov 2024 - Jan 2025)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Delivered AI-powered RAG legal solutions with 92% accuracy and IoT firmware boosts.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Completed 5+ engagements maintaining a 4.9/5 satisfaction score.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("Procter & Gamble", Style::new().fg(ACCENT_BLUE).bold()),
                    Span::styled(
                        " · IoT Developer & Automation Intern",
                        Style::new().fg(FG_PRIMARY).bold(),
                    ),
                    Span::styled("  (Dec 2024 - Jan 2025)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Rolled out IIoT monitoring across two lines capturing 200K+ daily data points.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Automated quality checks trimming manual inspection time by 15%.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Intellect Design Arena Ltd",
                        Style::new().fg(ACCENT_VIOLET).bold(),
                    ),
                    Span::styled(" · Software Intern", Style::new().fg(FG_PRIMARY).bold()),
                    Span::styled("  (Sept 2024 - Dec 2024)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Built PHP-based financial sites with 10% faster loads and managed AWS uptime to 99.9%.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Crafted analytics dashboards for real-time engagement insights.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "TYNATECH Ingenious Pvt Ltd",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ),
                    Span::styled(
                        " · IoT & LoRaWAN Developer",
                        Style::new().fg(FG_PRIMARY).bold(),
                    ),
                    Span::styled("  (May 2024 - Jun 2024)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Integrated 10+ LoRaWAN devices via REST APIs and MQTT visualizations handling 20 msg/s.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Boosted UART-LoRaWAN throughput by 10% through buffer tuning.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Hindustan Aeronautics Limited",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ),
                    Span::styled(" · R&D Intern", Style::new().fg(FG_PRIMARY).bold()),
                    Span::styled("  (July 2024 - Aug 2024)", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Optimized autopilot algorithms reducing computational overhead by 8%.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Delivered C++ tooling validating 40+ control system scenarios.",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
        ];

        let experience = List::new(experience_items)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Professional Experience",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .highlight_style(Style::new().fg(ACCENT_GOLD));
        ratatui::widgets::Widget::render(experience, main_split[0], buf);

        let right_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(78), Constraint::Percentage(22)])
            .split(main_split[1]);

        let project_items = vec![
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Fine-Tuning LLaMA 3.1 8B for Code Reasoning",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ),
                    Span::styled("  · Apr 2025", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Applied GRPO reinforcement learning with Gemma3 rewards to elevate code reasoning; released model & dataset.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Stack: LLaMA 3.1 8B · Gemma3 · Python · Hugging Face · DigitalOcean",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Multipurpose VR Gloves with Gesture Recognition",
                        Style::new().fg(ACCENT_TEAL).bold(),
                    ),
                    Span::styled("  · Sept 2024", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Captured 15+ gestures with 92% accuracy and sub-20ms wireless latency for immersive control.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Stack: Unity · C# · Python · ESP32 · Flex Sensors · IMU",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Instagram Automation Bot",
                        Style::new().fg(ACCENT_VIOLET).bold(),
                    ),
                    Span::styled("  · Dec 2024", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Handled 1000+ daily interactions with 99.5% uptime, using LLM sentiment analysis at 88% accuracy.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Stack: Python · Meta Graph API · FastAPI · Redis · GPT Models · AWS",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Smart Rhesus Macaque Deterrent System",
                        Style::new().fg(ACCENT_BLUE).bold(),
                    ),
                    Span::styled("  · Mar 2024", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Deployed edge vision alerts with 94% detection accuracy, cutting campus intrusions by 90%.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Stack: Raspberry Pi · Jetson Nano · PyTorch · Computer Vision · Ultrasonic Emitters",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        "Wearable Jump Height Measurement Device",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ),
                    Span::styled("  · Mar 2024", Style::new().fg(FG_SECONDARY)),
                ]),
                Line::from(Span::styled(
                    "Achieved ±1 cm accuracy with Kalman-filtered motion data and actionable training analytics.",
                    Style::new().fg(FG_PRIMARY),
                )),
                Line::from(Span::styled(
                    "Stack: ESP32 · MPU6050 · Flask · SQLite",
                    Style::new().fg(FG_SECONDARY),
                )),
            ]),
        ];

        let projects = List::new(project_items)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Highlighted Projects",
                        Style::new().fg(ACCENT_GOLD).bold(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .highlight_style(Style::new().fg(ACCENT_TEAL));
        ratatui::widgets::Widget::render(projects, right_column[0], buf);

        let contact = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Let's collaborate", Style::new().fg(ACCENT_TEAL).bold()),
                Span::styled(" — karneeshkar01@gmail.com", Style::new().fg(FG_PRIMARY)),
            ]),
            Line::from(Span::styled(
                "Open to backend, AI, and embedded engagements. Resume available on request.",
                Style::new().fg(FG_SECONDARY),
            )),
        ])
        .block(
            Block::default()
                .title(Span::styled("Contact", Style::new().fg(ACCENT_TEAL).bold()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL)),
        )
        .wrap(Wrap { trim: true });
        contact.render(right_column[1], buf);

        let footer = Paragraph::new(vec![
            Line::from(vec![
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
                "Email karneeshkar01@gmail.com for resume, collaborations, and project briefs.",
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
