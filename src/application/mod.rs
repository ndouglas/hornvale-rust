use crossterm::event::{read as read_event, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use eyre::Result;
use std::io::stdout;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

pub mod io;
pub use io::*;
pub mod ui;
pub use ui::*;

// The main application.
pub struct Application<'a> {
  // We could dispatch an IO event
  pub io_tx: Sender<IoEvent>,
  pub cli_textarea: TextArea<'a>,
  pub should_continue: bool,
  // actions: Actions,
  // is_loading: bool,
  // state: AppState,
}

impl Application<'_> {
  #[named]
  pub fn new(io_tx: Sender<IoEvent>) -> Self {
    // let actions = vec![Action::Quit].into();
    // let is_loading = false;
    // let state = AppState::default();
    let mut cli_textarea = TextArea::default();
    cli_textarea.set_cursor_line_style(Style::default());
    Self {
      io_tx,
      cli_textarea,
      should_continue: true,
      // actions,
      // is_loading,
      // state,
    }
  }

  /*
  pub async fn handle_key(&mut self, key: Key) -> AppReturn {
    if let Some(action) = self.actions.find(key) {
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
  */

  /*
  /// We could update the app or dispatch event on tick
  pub async fn update_on_tick(&mut self) -> AppReturn {
    // here we just increment a counter
    self.state.incr_tick();
    AppReturn::Continue
  }
  */

  /// Send a network event to the IO thread
  pub async fn dispatch(&mut self, action: IoEvent) {
    // `is_loading` will be set to false again after the async action has finished in io/handler.rs
    /*
    self.is_loading = true;
    if let Err(e) = self.io_tx.send(action).await {
      self.is_loading = false;
      error!("Error from dispatch {}", e);
    };
    */
  }

  pub fn did_initialize(&mut self) {
    /*
    self.actions = vec![
      Action::Quit,
      Action::Sleep,
      Action::IncrementDelay,
      Action::DecrementDelay,
    ]
    .into();
    self.state = AppState::initialized()
    */
  }

  pub fn did_load(&mut self) {
    // self.is_loading = false;
  }

  pub fn did_sleep(&mut self) {
    // self.state.incr_sleep();
  }
}

#[named]
pub async fn start_ui(app: &Arc<Mutex<Application<'_>>>) -> Result<()> {
  // Setup.
  let stdout = stdout();
  let mut stdout = stdout.lock();
  enable_raw_mode()?;
  crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  terminal.hide_cursor()?;

  // Main application loop.
  loop {
    let mut app = app.lock().await;
    terminal.draw(|rect| {
      ui::draw(rect, &mut app);
    })?;
    if !app.should_continue {
      break;
    }
  }

  /*
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
  */

  // Reset terminal and quit.
  terminal.clear()?;
  terminal.show_cursor()?;
  disable_raw_mode()?;
  crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  Ok(())
}
