use std::collections::HashMap;

use crate::application::input::keystroke::Keystroke;
use crate::application::Action;

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
  pub fn find(&self, keystroke: Keystroke) -> Option<&Action> {
    Action::iterator()
      .filter(|action| self.0.contains(action))
      .find(|action| action.keystrokes().contains(&keystroke))
  }

  pub fn actions(&self) -> &[Action] {
    self.0.as_slice()
  }
}

impl From<Vec<Action>> for Actions {
  fn from(actions: Vec<Action>) -> Self {
    let mut map: HashMap<Keystroke, Vec<Action>> = HashMap::new();
    for action in actions.iter() {
      for key in action.keystrokes().iter() {
        match map.get_mut(key) {
          Some(vec) => vec.push(*action),
          None => {
            map.insert(*key, vec![*action]);
          },
        }
      }
    }
    let conflicts = map
      .iter()
      .filter(|(_, actions)| actions.len() > 1)
      .map(|(key, actions)| {
        // At least two actions share same shortcut; panic!
        let actions = actions.iter().map(Action::to_string).collect::<Vec<_>>().join(", ");
        format!("Conflict key {} with actions {}", key, actions)
      })
      .collect::<Vec<_>>();
    if !conflicts.is_empty() {
      panic!("{}", conflicts.join("; "))
    }
    Self(actions)
  }
}
