use specs::prelude::*;
use specs_derive::*;

#[derive(Clone, Component, Debug, Hash, PartialEq)]
pub struct HasDescription(pub String);
