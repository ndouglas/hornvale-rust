use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use eyre::Result;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::style::Style;
use tui::Terminal;
use tui_textarea::{Input, TextArea};

pub mod hotkey_action;
pub use hotkey_action::*;
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
  pub actions: HotkeyActions,
  pub state: State<'a>,
  pub is_busy: bool,
}

impl<'a> Application<'a> {
  #[named]
  pub fn new(io_tx: Sender<IoEvent>) -> Self {
    let actions = vec![HotkeyAction::Quit].into();
    let state = State::new();
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

  pub async fn handle_keystroke(&mut self, keystroke: Keystroke) -> RunMode {
    if let Some(action) = self.actions.find(keystroke) {
      use HotkeyAction::*;
      match action {
        Quit => RunMode::Exit,
        Undo => {
          self.cli_textarea.undo();
          RunMode::Continue
        },
        Redo => {
          self.cli_textarea.redo();
          RunMode::Continue
        },
      }
    } else if self.input_mode == InputMode::Cli {
      if keystroke == Keystroke::Enter && validate(&mut self.cli_textarea) {
        send_input(&mut self.cli_textarea, &mut self.state);
      } else {
        self.cli_textarea.input(Input::from(keystroke));
      }
      RunMode::Continue
    } else {
      RunMode::Continue
    }
  }

  pub async fn tick(&mut self) -> RunMode {
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
    use HotkeyAction::*;
    self.actions = vec![Quit, Undo, Redo].into();
  }

  pub fn did_load(&mut self) {
    self.is_busy = false;
  }
}

#[named]
pub async fn start_ui<'a>(app: &Arc<Mutex<Application<'a>>>) -> Result<()> {
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
  let tick_rate = Duration::from_millis(400);
  let mut input_event_reader = InputEventReader::new(tick_rate);
  // Main application loop.
  loop {
    let mut app = app.lock().await;
    terminal.draw(|rect| {
      ui::draw(rect, &mut app);
    })?;
    match input_event_reader.next().await {
      InputEvent::Keystroke(keystroke) => {
        app.run_mode = app.handle_keystroke(keystroke).await;
      },
      InputEvent::Tick => {
        app.state.tick().await;
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
