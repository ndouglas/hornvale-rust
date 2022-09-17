use crossterm::event::Event::Key as EventKey;
use crossterm::event::{poll, read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use log::error;

use super::key::Key;
use super::InputEvent;

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct InputEventReader {
  rx: Receiver<InputEvent>,
  // Need to be kept around to prevent disposing the sender side.
  _tx: Sender<InputEvent>,
  // To stop the loop
  stop_capture: Arc<AtomicBool>,
}

impl InputEventReader {
  /// Constructs an new instance of `InputEventReader` with the default config.
  pub fn new(tick_rate: Duration) -> InputEventReader {
    let (tx, rx) = channel(100);
    let stop_capture = Arc::new(AtomicBool::new(false));

    let event_tx = tx.clone();
    let event_stop_capture = stop_capture.clone();
    tokio::spawn(async move {
      loop {
        // poll for tick rate duration, if no event, sent tick event.
        if poll(tick_rate).unwrap() {
          if let EventKey(key) = read().unwrap() {
            let key = Key::from(key);
            if let Err(err) = event_tx.send(InputEvent::Input(key)).await {
              error!("Oops!, {}", err);
            }
          }
        }
        if let Err(err) = event_tx.send(InputEvent::Tick).await {
          error!("Oops!, {}", err);
        }
        if event_stop_capture.load(Ordering::Relaxed) {
          break;
        }
      }
    });

    InputEventReader {
      rx,
      _tx: tx,
      stop_capture,
    }
  }

  /// Attempts to read an event.
  pub async fn next(&mut self) -> InputEvent {
    self.rx.recv().await.unwrap_or(InputEvent::Tick)
  }

  /// Close
  pub fn close(&mut self) {
    self.stop_capture.store(true, Ordering::Relaxed)
  }
}
