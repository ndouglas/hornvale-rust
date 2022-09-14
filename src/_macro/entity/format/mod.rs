#[macro_export]
macro_rules! format_room {
  ($room: expr) => {{
    use colored::*;
    let mut string = String::new();
    if let Some(name) = get_room_name!($room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_room_description!($room) {
      string.push_str(format!("{}\n", description.green()).as_str());
    }
    if let Some(exits) = get_exits!($room) {
      string.push_str(format!("{}\n", format!("{}", exits).bright_green()).as_str());
    }
    format!("{}", string)
  }};
}
