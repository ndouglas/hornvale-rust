#[macro_export]
macro_rules! format_room {
  ($ecs: expr, $room: expr) => {
    {
      use colored::*;
      let mut string = String::new();
      if let Some(name) = get_name!($ecs, $room) {
        string.push_str(format!("{}\n", name.magenta()).as_str());
      }
      if let Some(description) = get_description!($ecs, $room) {
        string.push_str(format!("{}\n", description.green()).as_str());
      }
      if let Some(exits) = get_exits!($ecs, $room) {
        string.push_str(format!("{}\n", format!("{}", exits).bright_green()).as_str());
      }
      format!("{}", string)
    }
  };
}
