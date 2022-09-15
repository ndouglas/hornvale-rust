use std::collections::VecDeque;
use std::sync::Mutex;

lazy_static! {
  pub static ref MESSAGE_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_message(message: String) {
  MESSAGE_QUEUE.lock().unwrap().push_back(message);
}

#[named]
pub fn get_messages() -> Vec<String> {
  MESSAGE_QUEUE.lock().unwrap().drain(..).collect::<Vec<String>>()
}

#[named]
pub fn start_message_spammer() {
  use rand::{thread_rng, Rng};
  use std::thread::{sleep, spawn};
  use std::time::Duration;
  spawn(move || {
    let mut rng = thread_rng();
    let mut i = 0usize;
    sleep(Duration::from_secs(2));
    loop {
      enq_message!(format!("External message #{}", i));
      let wait_ms = rng.gen_range(1000..20000);
      sleep(Duration::from_millis(wait_ms));
      i += 1;
    }
  });
}

// This is a nice little effect, but why would I want to make the game seem slower?
// Perhaps if we're doing something that _should_ take some time, etc.
// Unfortunately, it can end up interfering with other messages that get interleaved.
#[named]
pub fn show_spinner(length: usize) {
  use std::thread::spawn;

  spawn(move || {
    let frames = vec![
      "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
    ];
    enq_message!(format!(""));
    for i in 0..length {
      enq_message!(format!("\x1b[1A{}", frames[i % frames.len()]));
    }
    enq_message!(format!("\x1b[2A"));
  });
}
