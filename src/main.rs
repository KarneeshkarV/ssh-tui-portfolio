use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::distr::{Distribution, Uniform};
use ratatui::prelude::*;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};
use tui_big_text::BigText;
mod screens;

enum ScreenWidget {
    Intro(screens::intro_screen::IntroScreenWidget),
    First(BigText<'static>),
    Third(screens::third_screen::SparkWidget),
}

impl ratatui::widgets::Widget for ScreenWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ScreenWidget::Intro(widget) => widget.render(area, buf),
            ScreenWidget::First(widget) => widget.render(area, buf),
            ScreenWidget::Third(widget) => widget.render(area, buf),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default, PartialEq)]
enum State {
    #[default]
    Intro,
    First,
    Third,
}
/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    text: String,
    screen: State,
    /// Data for third screen sparklines
    spark_data: [Vec<u64>; 3],
}
const TEXT1: &str = "Karneeshkar V";
const TEXT2: &str = "Veera";
impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        // Initialize sparkline data so third screen has something to show
        let mut rng = rand::rng();
        let dist = Uniform::new(0, 100).unwrap();
        app.spark_data = [
            (0..100).map(|_| dist.sample(&mut rng)).collect(),
            (0..100).map(|_| dist.sample(&mut rng)).collect(),
            (0..100).map(|_| dist.sample(&mut rng)).collect(),
        ];
        app
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        let tick_rate = std::time::Duration::from_millis(200);
        while self.running {
            if event::poll(tick_rate)? {
                self.handle_crossterm_events()?;
            } else {
                // No input within tick_rate; advance app state
                self.on_tick();
            }
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        if self.text.is_empty() {
            self.text = TEXT1.to_string();
        }
        let title = Line::from("Karneeshkar V - SSH Portfolio")
            .bold()
            .green()
            .centered();
        let text = format!(
            "Hello, Hey!\n\n\
            My name is {}\n\n\
            Press [ or ] to change the text.\n\n\
            Press p or n to change the Bottom Layout.\n\n\
             Press `Esc`, `Ctrl-C` or `q` to stop running.",
            self.text
        );

        let active_screen = match self.screen {
            State::Intro => ScreenWidget::Intro(screens::intro_screen::intro_screen()),
            State::First => ScreenWidget::First(screens::first_screen::first_screen()),
            State::Third => {
                ScreenWidget::Third(screens::third_screen::third_screen_from(&self.spark_data))
            }
        };

        // Intro screen takes full screen, other screens split in half
        if self.screen == State::Intro {
            frame.render_widget(active_screen, frame.area());
        } else {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.area());
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer_layout[0]);
            frame.render_widget(active_screen, layout[1]);
            frame.render_widget(
                Paragraph::new(text)
                    .block(Block::bordered().title(title))
                    .centered()
                    .red(),
                layout[0],
            )
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('[')) => self.text = TEXT1.to_string(),
            (_, KeyCode::Char(']')) => self.text = TEXT2.to_string(),
            (_, KeyCode::Char('n')) => self.next_screen(),
            (_, KeyCode::Char('p')) => self.previous_screen(),
            _ => {}
        }
    }

    fn next_screen(&mut self) {
        self.screen = match self.screen {
            State::Intro => State::First,
            State::First => State::Third,
            State::Third => State::First,
        };
    }

    fn previous_screen(&mut self) {
        self.screen = match self.screen {
            State::Intro => State::Third,
            State::First => State::Intro,
            State::Third => State::First,
        };
    }
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Periodic tick to update dynamic data (e.g., sparklines)
    fn on_tick(&mut self) {
        self.update_spark_data();
    }

    /// Append a new sample to each sparkline and keep a fixed history length
    fn update_spark_data(&mut self) {
        let mut rng = rand::rng();
        let dist = Uniform::new(0, 100).unwrap();
        for series in &mut self.spark_data {
            series.push(dist.sample(&mut rng));
            if series.len() > 100 {
                series.remove(0);
            }
        }
    }
}
