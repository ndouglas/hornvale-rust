use std::io::{stdin, stdout, Write};

#[named]
pub fn read_input() {
  trace_enter!();
  let mut s = String::new();
  print!("> ");
  let _ = stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n') = s.chars().next_back() {
    s.pop();
  }
  if let Some('\r') = s.chars().next_back() {
    s.pop();
  }
  trace_exit!();
}
