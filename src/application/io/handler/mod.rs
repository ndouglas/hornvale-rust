use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::application::Application;

pub struct IoHandler<'a> {
  application: Arc<Mutex<Application<'a>>>,
}

impl<'a> IoHandler<'a> {
  pub fn new(application: Arc<Mutex<Application<'a>>>) -> Self {
    Self { application }
  }

  pub async fn handle_io_event(&mut self, io_event: IoEvent) {
    use IoEvent::*;
    let outcome = match io_event {
      Initialize => self.initialize().await,
      Sleep(duration) => self.sleep(duration).await,
    };
    if let Err(err) = outcome {
      error!("Oops, something wrong happened: {:?}", err);
    }
    let mut application = self.application.lock().await;
    application.did_load();
  }

  async fn initialize(&mut self) -> Result<()> {
    info!("🚀 Initialize the application");
    let mut application = self.application.lock().await;
    sleep(Duration::from_secs(1)).await;
    application.did_initialize();
    info!("👍 Application initialized");
    Ok(())
  }

  async fn sleep(&mut self, duration: Duration) -> Result<()> {
    info!("😴 Go sleeping for {:?}...", duration);
    sleep(duration).await;
    info!("⏰ Wake up !");
    let mut application = self.application.lock().await;
    application.did_sleep();
    Ok(())
  }
}
