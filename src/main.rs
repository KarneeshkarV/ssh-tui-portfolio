use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::distr::{Distribution, Uniform};
use ratatui::{DefaultTerminal, Frame, prelude::*};
mod screens;
use screens::intro_screen::ASCII_FRAMES;

enum ScreenWidget {
    Intro(screens::intro_screen::IntroScreenWidget),
    First(screens::first_screen::FirstScreenWidget),
    Second(screens::second_screen::SecondScreenWidget),
    Third(screens::third_screen::SparkWidget),
}

impl ratatui::widgets::Widget for ScreenWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ScreenWidget::Intro(widget) => widget.render(area, buf),
            ScreenWidget::First(widget) => widget.render(area, buf),
            ScreenWidget::Second(widget) => widget.render(area, buf),
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
    Second,
    Third,
}
/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    call_sign: String,
    screen: State,
    /// Data for third screen sparklines
    spark_data: [Vec<u64>; 3],
    /// Current frame index for intro ASCII animation
    intro_frame_index: usize,
    /// Tick accumulator to pace intro animation
    intro_tick: u8,
}
const PRIMARY_CALL_SIGN: &str = "Karneeshkar V";
const SECONDARY_CALL_SIGN: &str = "Veera";
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
        app.call_sign = PRIMARY_CALL_SIGN.to_string();
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
        if self.call_sign.is_empty() {
            self.call_sign = PRIMARY_CALL_SIGN.to_string();
        }

        let widget = match self.screen {
            State::Intro => ScreenWidget::Intro(screens::intro_screen::intro_screen(
                ASCII_FRAMES[self.intro_frame_index],
            )),
            State::First => {
                ScreenWidget::First(screens::first_screen::first_screen(&self.call_sign))
            }
            State::Second => {
                ScreenWidget::Second(screens::second_screen::second_screen(&self.call_sign))
            }
            State::Third => {
                ScreenWidget::Third(screens::third_screen::third_screen_from(&self.spark_data))
            }
        };

        frame.render_widget(widget, frame.area());
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
            (_, KeyCode::Char('[')) => self.set_call_sign(PRIMARY_CALL_SIGN),
            (_, KeyCode::Char(']')) => self.set_call_sign(SECONDARY_CALL_SIGN),
            (_, KeyCode::Char('n')) => self.next_screen(),
            (_, KeyCode::Char('p')) => self.previous_screen(),
            _ => {}
        }
    }

    fn next_screen(&mut self) {
        self.screen = match self.screen {
            State::Intro => State::First,
            State::First => State::Second,
            State::Second => State::Third,
            State::Third => State::First,
        };
    }

    fn previous_screen(&mut self) {
        self.screen = match self.screen {
            State::Intro => State::Third,
            State::First => State::Intro,
            State::Second => State::First,
            State::Third => State::Second,
        };
    }
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn set_call_sign(&mut self, next: &str) {
        self.call_sign = next.to_string();
    }

    /// Periodic tick to update dynamic data (e.g., sparklines)
    fn on_tick(&mut self) {
        self.update_spark_data();
        self.update_intro_animation();
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

    fn update_intro_animation(&mut self) {
        let frame_count = ASCII_FRAMES.len();
        if frame_count <= 1 {
            return;
        }

        if self.screen != State::Intro {
            self.intro_tick = 0;
            self.intro_frame_index %= frame_count;
            return;
        }

        self.intro_tick = self.intro_tick.saturating_add(1);
        if self.intro_tick >= 4 {
            self.intro_tick = 0;
            self.intro_frame_index = (self.intro_frame_index + 1) % frame_count;
        }
    }
}
