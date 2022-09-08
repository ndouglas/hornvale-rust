use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct IsAPlayer {}
