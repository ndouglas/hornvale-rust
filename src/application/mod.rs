use crossterm::event::{read as read_event, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use eyre::Result;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::Terminal;
use tui_textarea::{Input, Key as TextAreaKey, TextArea};

use crate::state::State as GameState;

pub mod action;
pub use action::*;
pub mod input;
pub use input::*;
pub mod io;
pub use io::*;
pub mod run_mode;
pub use run_mode::*;
pub mod state;
pub use state::*;
pub mod ui;
pub use ui::*;

// The main application.
pub struct Application<'a> {
  pub io_tx: Sender<IoEvent>,
  pub cli_textarea: TextArea<'a>,
  pub input_mode: InputMode,
  pub run_mode: RunMode,
  pub actions: Actions,
  pub state: ApplicationState,
  pub is_busy: bool,
}

impl Application<'_> {
  #[named]
  pub fn new(io_tx: Sender<IoEvent>) -> Self {
    let actions = vec![Action::Quit].into();
    let state = ApplicationState::default();
    let run_mode = RunMode::Continue;
    let input_mode = InputMode::Cli;
    let mut cli_textarea = TextArea::default();
    cli_textarea.set_cursor_line_style(Style::default());
    let is_busy = true;
    Self {
      io_tx,
      cli_textarea,
      run_mode,
      input_mode,
      actions,
      state,
      is_busy,
    }
  }

  pub async fn handle_key(&mut self, key: Key) -> RunMode {
    if let Some(action) = self.actions.find(key) {
      match action {
        Action::Quit => RunMode::Exit,
        Action::Sleep => {
          if let Some(duration) = self.state.duration().cloned() {
            // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
            self.dispatch(IoEvent::Sleep(duration)).await
          }
          RunMode::Continue
        },
      }
    } else if self.input_mode == InputMode::Cli {
      if key == Key::Enter && validate(&mut self.cli_textarea) {
        return RunMode::Exit;
      }
      if let Key::Char(char) = key {
        self.cli_textarea.input(Input::from(crossterm::event::KeyEvent {
          code: crossterm::event::KeyCode::Char(char),
          modifiers: crossterm::event::KeyModifiers::empty(),
        }));
      }
      RunMode::Continue
    } else {
      RunMode::Continue
    }
  }

  pub async fn tick(&mut self) -> RunMode {
    self.state.increment_tick();
    RunMode::Continue
  }

  pub fn should_continue(&mut self) -> bool {
    self.run_mode != RunMode::Exit
  }

  pub async fn dispatch(&mut self, action: IoEvent) {
    self.is_busy = true;
    if let Err(e) = self.io_tx.send(action).await {
      self.is_busy = false;
      error!("Error from dispatch {}", e);
    };
  }

  pub fn did_initialize(&mut self) {
    self.actions = vec![Action::Quit, Action::Sleep].into();
    self.state = ApplicationState::initialized();
  }

  pub fn did_load(&mut self) {
    self.is_busy = false;
  }

  pub fn did_sleep(&mut self) {
    self.state.increment_sleep();
  }
}

#[named]
pub async fn start_ui(app: &Arc<Mutex<Application<'_>>>) -> Result<()> {
  // Setup.
  let stdout = stdout();
  let mut stdout = stdout.lock();
  enable_raw_mode()?;
  crossterm::execute!(stdout, EnterAlternateScreen)?; //, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  terminal.hide_cursor()?;

  {
    let mut app = app.lock().await;
    // Here we assume the the first load is a long task
    app.dispatch(IoEvent::Initialize).await;
  }

  // Start processing events.
  let tick_rate = Duration::from_millis(200);
  let mut input_event_reader = InputEventReader::new(tick_rate);
  let mut game_state = GameState::new();
  // Main application loop.
  loop {
    let mut app = app.lock().await;
    terminal.draw(|rect| {
      ui::draw(rect, &mut app);
    })?;
    match input_event_reader.next().await {
      InputEvent::Input(key) => {
        app.run_mode = app.handle_key(key).await;
      },
      InputEvent::Tick => {
        game_state.tick();
        app.run_mode = app.tick().await;
      },
    }
    if !app.should_continue() {
      input_event_reader.close();
      break;
    }
  }

  // Reset terminal and quit.
  terminal.clear()?;
  terminal.show_cursor()?;
  disable_raw_mode()?;
  crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  Ok(())
}
