#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use clap::Parser;
use eyre::Result;
use log::LevelFilter;
use specs::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tui_logger::{init_logger, set_default_level};

use application::io::handler::IoHandler;
use application::io::IoEvent;
use application::start_ui;

use hornvale::*;

#[named]
#[tokio::main]
async fn main() -> Result<()> {
  let _args = cli::Arguments::parse();

  // Create channel for I/O.
  let (sync_io_tx, mut sync_io_rx) = channel::<IoEvent>(100);

  // Configure logs.
  init_logger(LevelFilter::Debug).unwrap();
  set_default_level(log::LevelFilter::Debug);

  // We need to share the App between threads.
  let app = Arc::new(Mutex::new(application::Application::new(sync_io_tx.clone())));
  let app_ui = Arc::clone(&app);

  // Spawn a distinct thread for I/O.
  tokio::spawn(async move {
    let mut handler = IoHandler::new(app);
    // Process incoming I/O.
    while let Some(io_event) = sync_io_rx.recv().await {
      handler.handle_io_event(io_event).await;
    }
  });

  /*
  // Spawn a distinct thread for the ECS.
  tokio::spawn(async move {
    let mut state = state::State::new();
    state.run();
  });
  */

  // Start the user interface.
  application::start_ui(&app_ui).await?;
  Ok(())
}
