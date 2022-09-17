use crossterm::event::Event::Key as EventKey;
use crossterm::event::{poll, read};
use log::error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::key::Key;
use super::InputEvent;

pub struct InputEventReader {
  rx: Receiver<InputEvent>,
  // Needs to be retained to prevent disposing of the sender.
  _tx: Sender<InputEvent>,
  // To stop the loop.
  stop_capture: Arc<AtomicBool>,
}

impl InputEventReader {
  pub fn new(tick_rate: Duration) -> InputEventReader {
    let (tx, rx) = channel(100);
    let stop_capture = Arc::new(AtomicBool::new(false));
    let event_tx = tx.clone();
    let event_stop_capture = stop_capture.clone();
    tokio::spawn(async move {
      loop {
        // Poll for tick rate duration.
        // If we do not receive another event, send a tick event.
        if poll(tick_rate).unwrap() {
          if let EventKey(key) = read().unwrap() {
            let key = Key::from(key);
            if let Err(err) = event_tx.send(InputEvent::Input(key)).await {
              error!("Encountered an error transmitting an input event: {}", err);
            }
          }
        }
        if let Err(err) = event_tx.send(InputEvent::Tick).await {
          error!("Encountered error transmitting a tick: {}", err);
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

  pub async fn next(&mut self) -> InputEvent {
    self.rx.recv().await.unwrap_or(InputEvent::Tick)
  }

  pub fn close(&mut self) {
    self.stop_capture.store(true, Ordering::Relaxed)
  }
}
