use crate::actions::Action;

pub trait Commandable {
  fn get_action(&self) -> Action;
}
