use crate::hundred_days::game::Game;
pub use std::{collections::HashMap, fs};
pub use serde::Deserialize;

pub mod daily;
pub mod global;
pub mod manual;

pub trait Information {
    fn name(&self) -> &str;
    fn description(&self) -> String;
}