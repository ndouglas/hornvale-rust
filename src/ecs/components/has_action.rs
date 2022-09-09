use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::actions::Action;

#[derive(Component, Debug, Hash, PartialEq)]
pub struct HasAction(pub Action);
