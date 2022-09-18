use std::collections::HashMap;

use crate::application::input::keystroke::Keystroke;
use crate::application::HotkeyAction;

#[derive(Default, Debug, Clone)]
pub struct HotkeyActions(Vec<HotkeyAction>);

impl HotkeyActions {
  pub fn find(&self, keystroke: Keystroke) -> Option<&HotkeyAction> {
    HotkeyAction::iterator()
      .filter(|action| self.0.contains(action))
      .find(|action| action.keystrokes().contains(&keystroke))
  }

  pub fn actions(&self) -> &[HotkeyAction] {
    self.0.as_slice()
  }
}

impl From<Vec<HotkeyAction>> for HotkeyActions {
  fn from(actions: Vec<HotkeyAction>) -> Self {
    let mut map: HashMap<Keystroke, Vec<HotkeyAction>> = HashMap::new();
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
        let actions = actions
          .iter()
          .map(HotkeyAction::to_string)
          .collect::<Vec<_>>()
          .join(", ");
        format!("Conflict key {} with hotkey actions {}", key, actions)
      })
      .collect::<Vec<_>>();
    if !conflicts.is_empty() {
      panic!("{}", conflicts.join("; "))
    }
    Self(actions)
  }
}
