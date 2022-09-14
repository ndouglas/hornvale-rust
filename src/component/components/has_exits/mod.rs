use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HasExits {
  pub exits: Exits,
}
