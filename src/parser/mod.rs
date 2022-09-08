use crate::State;

#[named]
pub fn parse(string: String, state: &mut State) {
  trace_enter!();
  let words: Vec<&str> = string
    .split_whitespace()
    .collect();
  let first : String = words.get(0).unwrap_or(&"").to_string();
  trace_var!(first);
  let second : String = words.get(1).unwrap_or(&"").to_string();
  trace_var!(second);
  match first.as_str() {
    "n" | "north" => {},
    "ne" | "northeast" => {},
    "nw" | "northwest" => {},
    "e" | "east" => {},
    "w" | "west" => {},
    "s" | "south" => {},
    "se" | "southeast" => {},
    "sw" | "southwest" => {},
    "l" | "look" => {},
    &_ => todo!(),
  }
  trace_exit!();
}
