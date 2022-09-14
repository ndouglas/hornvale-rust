#[macro_export]
macro_rules! get_tick {
  ($ecs: expr) => {{
    use crate::tick::TICK;
    *TICK.lock().unwrap()
  }};
}
