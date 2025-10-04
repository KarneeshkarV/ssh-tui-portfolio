use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline, Wrap},
};

use super::theme::*;

pub struct SparkWidget {
    data: [Vec<u64>; 3],
}

#[derive(Clone, Copy, Debug)]
struct SeriesStats {
    last: u64,
    max: u64,
    min: u64,
    avg: f64,
    delta: i64,
}

impl SeriesStats {
    fn from_series(series: &[u64]) -> Self {
        if series.is_empty() {
            return Self {
                last: 0,
                max: 0,
                min: 0,
                avg: 0.0,
                delta: 0,
            };
        }

        let mut min = series[0];
        let mut max = series[0];
        let mut sum = 0u64;
        for &value in series {
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
            sum += value;
        }

        let last = *series.last().unwrap();
        let first = series[0];
        let avg = sum as f64 / series.len() as f64;
        let delta = last as i64 - first as i64;

        Self {
            last,
            max,
            min,
            avg,
            delta,
        }
    }

    fn trend_symbol(&self) -> &'static str {
        match self.delta.cmp(&0) {
            std::cmp::Ordering::Greater => "↑",
            std::cmp::Ordering::Less => "↓",
            std::cmp::Ordering::Equal => "→",
        }
    }

    fn trend_magnitude(&self) -> u64 {
        if self.delta < 0 {
            (-self.delta) as u64
        } else {
            self.delta as u64
        }
    }
}

impl ratatui::widgets::Widget for SparkWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(BG_CANVAS));

        let content = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        if content.width < 48 || content.height < 15 {
            return;
        }

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(9),
                Constraint::Length(3),
            ])
            .split(content);

        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(5)])
            .split(sections[0]);

        buf.set_style(header_chunks[0], Style::new().bg(BG_HERO));
        buf.set_style(header_chunks[1], Style::new().bg(BG_SECTION));
        buf.set_style(sections[1], Style::new().bg(BG_SECTION));
        buf.set_style(sections[2], Style::new().bg(BG_FOOTER));

        let data = self.data;

        let series_stats: Vec<SeriesStats> = data
            .iter()
            .map(|series| SeriesStats::from_series(series))
            .collect();

        let mut total_samples = 0usize;
        let mut latest_sum = 0u64;
        let mut latest_count = 0u64;
        let mut min_value = u64::MAX;
        let mut max_value = u64::MIN;
        let mut has_value = false;

        for series in &data {
            total_samples += series.len();
            if let Some(last) = series.last() {
                latest_sum += *last;
                latest_count += 1;
            }

            for &value in series {
                has_value = true;
                if value < min_value {
                    min_value = value;
                }
                if value > max_value {
                    max_value = value;
                }
            }
        }

        let composite_signal = if latest_count > 0 {
            latest_sum as f64 / latest_count as f64
        } else {
            0.0
        };

        let spread = if has_value { max_value - min_value } else { 0 };

        let header = Paragraph::new(vec![
            Line::from(Span::styled(
                "Operations Telemetry",
                Style::new().fg(ACCENT_GOLD).bold(),
            )),
            Line::from(Span::styled(
                "Live signal metrics refreshing every 200ms across three service clusters.",
                Style::new().fg(FG_PRIMARY),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Span::styled(
                    "Systems Pulse",
                    Style::new().fg(ACCENT_TEAL).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_HERO)),
        )
        .wrap(Wrap { trim: true });
        header.render(header_chunks[0], buf);

        let metrics_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(header_chunks[1]);

        let metric_cards = [
            (
                format!("{}", total_samples),
                "Active Samples",
                "Points retained across all telemetry streams.",
                ACCENT_TEAL,
            ),
            (
                format!("{:.1}%", composite_signal),
                "Composite Signal",
                "Latest blended health score (0-100).",
                ACCENT_BLUE,
            ),
            (
                format!("{}", spread),
                "Signal Spread",
                "Range between min and max readings this session.",
                ACCENT_GOLD,
            ),
        ];

        for (column, (value, title, description, accent)) in
            metrics_columns.iter().zip(metric_cards.iter())
        {
            let accent = *accent;
            let card = Paragraph::new(vec![
                Line::from(Span::styled(value.as_str(), Style::new().fg(accent).bold())),
                Line::from(Span::styled(*title, Style::new().fg(FG_PRIMARY))),
                Line::from(Span::styled(*description, Style::new().fg(FG_MUTED))),
            ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::new().bg(BG_PANEL)),
            )
            .wrap(Wrap { trim: true });
            card.render(*column, buf);
        }

        let spark_area = sections[1];
        let spark_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(spark_area);

        let series_meta = [
            ("Signal Alpha", ACCENT_TEAL),
            ("Signal Beta", ACCENT_VIOLET),
            ("Signal Gamma", ACCENT_GOLD),
        ];

        for (idx, column) in spark_columns.iter().enumerate() {
            let (label, accent) = series_meta[idx];
            let stats = series_stats[idx];
            let series = &data[idx];

            let block = Block::default()
                .title(Line::from(vec![
                    Span::styled(label, Style::new().fg(accent).bold()),
                    Span::raw("  ·  "),
                    Span::styled("Telemetry", Style::new().fg(FG_SECONDARY)),
                ]))
                .title_bottom(Line::from(vec![
                    Span::styled(
                        format!("now {:>3}", stats.last),
                        Style::new().fg(FG_PRIMARY).bold(),
                    ),
                    Span::raw("  "),
                    Span::styled("avg", Style::new().fg(FG_MUTED)),
                    Span::styled(format!(" {:>5.1}", stats.avg), Style::new().fg(FG_PRIMARY)),
                    Span::raw("  "),
                    Span::styled("min", Style::new().fg(FG_MUTED)),
                    Span::styled(format!(" {:>3}", stats.min), Style::new().fg(FG_PRIMARY)),
                    Span::raw("  "),
                    Span::styled("max", Style::new().fg(FG_MUTED)),
                    Span::styled(format!(" {:>3}", stats.max), Style::new().fg(FG_PRIMARY)),
                    Span::raw("  "),
                    Span::styled("trend", Style::new().fg(FG_MUTED)),
                    Span::styled(
                        format!(" {}{:>2}", stats.trend_symbol(), stats.trend_magnitude()),
                        Style::new().fg(accent).bold(),
                    ),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::new().bg(BG_PANEL));

            let sparkline = Sparkline::default()
                .block(block)
                .data(series)
                .max(stats.max.max(1))
                .style(Style::new().fg(accent));

            sparkline.render(*column, buf);
        }

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
                "Signals refresh every 200ms · data resets per launch.",
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

/// Build the spark widget from existing data (keeps animation state in App)
pub fn third_screen_from(data: &[Vec<u64>; 3]) -> SparkWidget {
    SparkWidget {
        data: [data[0].clone(), data[1].clone(), data[2].clone()],
    }
}
