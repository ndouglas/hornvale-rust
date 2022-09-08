use serde::*;
use specs::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player(pub Entity);
