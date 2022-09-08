use std::io::{stdin, stdout, Write};

use crate::parse;
use crate::State;

#[named]
pub fn read_input(state: &mut State) {
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
  parse(s, state);
  trace_exit!();
}
