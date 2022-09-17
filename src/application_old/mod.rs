use log::{debug, error, warn};

use self::actions::Action;
use self::actions::Actions;
use self::state::AppState;
use eyre::Result;
use inputs::key::Key;
use tokio::sync::mpsc::Sender;

use specs::prelude::*;

use std::io::stdout;

use std::sync::Arc;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use inputs::events::Events;
use inputs::InputEvent;

use io::IoEvent;

pub mod actions;
pub mod inputs;
pub mod io;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
  Exit,
  Continue,
}

/// The main application, containing the state
pub struct App {
  /// We could dispatch an IO event
  io_tx: Sender<IoEvent>,
  /// Contextual actions
  actions: Actions,
  /// State
  is_loading: bool,
  state: AppState,
}

impl App {
  pub fn new(io_tx: Sender<IoEvent>) -> Self {
    let actions = vec![Action::Quit].into();
    let is_loading = false;
    let state = AppState::default();

    Self {
      io_tx,
      actions,
      is_loading,
      state,
    }
  }

  /// Handle a user action
  pub async fn do_action(&mut self, key: Key) -> AppReturn {
    if let Some(action) = self.actions.find(key) {
      debug!("Run action [{:?}]", action);
      match action {
        Action::Quit => AppReturn::Exit,
        Action::Sleep => {
          if let Some(duration) = self.state.duration().cloned() {
            // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
            self.dispatch(IoEvent::Sleep(duration)).await
          }
          AppReturn::Continue
        },
        // IncrementDelay and DecrementDelay is handled in the UI thread
        Action::IncrementDelay => {
          self.state.increment_delay();
          AppReturn::Continue
        },
        // Note, that we clamp the duration, so we stay >= 0
        Action::DecrementDelay => {
          self.state.decrement_delay();
          AppReturn::Continue
        },
      }
    } else {
      warn!("No action accociated to {}", key);
      AppReturn::Continue
    }
  }

  /// We could update the app or dispatch event on tick
  pub async fn update_on_tick(&mut self) -> AppReturn {
    // here we just increment a counter
    self.state.incr_tick();
    AppReturn::Continue
  }

  /// Send a network event to the IO thread
  pub async fn dispatch(&mut self, action: IoEvent) {
    // `is_loading` will be set to false again after the async action has finished in io/handler.rs
    self.is_loading = true;
    if let Err(e) = self.io_tx.send(action).await {
      self.is_loading = false;
      error!("Error from dispatch {}", e);
    };
  }

  pub fn actions(&self) -> &Actions {
    &self.actions
  }
  pub fn state(&self) -> &AppState {
    &self.state
  }

  pub fn is_loading(&self) -> bool {
    self.is_loading
  }

  pub fn initialized(&mut self) {
    // Update contextual actions
    self.actions = vec![
      Action::Quit,
      Action::Sleep,
      Action::IncrementDelay,
      Action::DecrementDelay,
    ]
    .into();
    self.state = AppState::initialized()
  }

  pub fn loaded(&mut self) {
    self.is_loading = false;
  }

  pub fn slept(&mut self) {
    self.state.incr_sleep();
  }
}

#[named]
pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
  // Configure Crossterm backend for tui
  let stdout = stdout();
  crossterm::terminal::enable_raw_mode()?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  terminal.hide_cursor()?;

  // User event handler
  let tick_rate = Duration::from_millis(200);
  let mut events = Events::new(tick_rate);

  // Trigger state change from Init to Initialized
  {
    let mut app = app.lock().await;
    // Here we assume the the first load is a long task
    app.dispatch(IoEvent::Initialize).await;
  }

  loop {
    let mut app = app.lock().await;

    // Render
    terminal.draw(|rect| ui::draw(rect, &app))?;

    // Handle inputs
    let result = match events.next().await {
      InputEvent::Input(key) => app.do_action(key).await,
      InputEvent::Tick => app.update_on_tick().await,
    };
    // Check if we should exit
    if result == AppReturn::Exit {
      events.close();
      break;
    }
  }

  // Restore the terminal and close application
  terminal.clear()?;
  terminal.show_cursor()?;
  crossterm::terminal::disable_raw_mode()?;

  Ok(())
}


fn main() -> io::Result<()> {
  let stdout = io::stdout();
  let mut stdout = stdout.lock();

  enable_raw_mode()?;
  crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut term = Terminal::new(backend)?;

  let mut textarea = TextArea::default();
  textarea.set_cursor_line_style(Style::default());
  let layout =
      Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
  let mut is_valid = validate(&mut textarea);

  loop {
      term.draw(|f| {
          let chunks = layout.split(f.size());
          let widget = textarea.widget();
          f.render_widget(widget, chunks[0]);
      })?;

      match crossterm::event::read()?.into() {
          Input { key: Key::Esc, .. } => break,
          Input {
              key: Key::Enter, ..
          } if is_valid => break,
          Input {
              key: Key::Char('m'),
              ctrl: true,
              ..
          }
          | Input {
              key: Key::Enter, ..
          } => {}
          input => {
              // TextArea::input returns if the input modified its text
              if textarea.input(input) {
                  is_valid = validate(&mut textarea);
              }
          }
      }
  }

  disable_raw_mode()?;
  crossterm::execute!(
      term.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
  )?;
  term.show_cursor()?;

  println!("Input: {:?}", textarea.lines()[0]);
  Ok(())
}
