use rand::distr::{Distribution, Uniform};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    widgets::{Block, Borders, Sparkline},
};

pub struct SparkWidget {
    data: [Vec<u64>; 3],
}

impl ratatui::widgets::Widget for SparkWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data1"),
            )
            .data(&self.data[0])
            .style(Style::default().fg(Color::Yellow));
        sparkline.render(chunks[0], buf);

        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data2"),
            )
            .data(&self.data[1])
            .style(Style::default().bg(Color::Green));
        sparkline.render(chunks[1], buf);

        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data3"),
            )
            .data(&self.data[2])
            .style(Style::default().fg(Color::Red));
        sparkline.render(chunks[2], buf);
    }
}

pub fn third_screen() -> SparkWidget {
    let mut rng = rand::rng();
    let dist = Uniform::new(0, 100).unwrap();

    // Generate fresh data each call
    let data1: Vec<u64> = (0..100).map(|_| dist.sample(&mut rng)).collect();
    let data2: Vec<u64> = (0..100).map(|_| dist.sample(&mut rng)).collect();
    let data3: Vec<u64> = (0..100).map(|_| dist.sample(&mut rng)).collect();

    SparkWidget {
        data: [data1, data2, data3],
    }
}

/// Build the spark widget from existing data (keeps animation state in App)
pub fn third_screen_from(data: &[Vec<u64>; 3]) -> SparkWidget {
    SparkWidget {
        data: [data[0].clone(), data[1].clone(), data[2].clone()],
    }
}
