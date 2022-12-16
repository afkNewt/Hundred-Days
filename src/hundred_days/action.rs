use super::game::Game;

pub mod daily;
pub mod global;
pub mod manual;

pub trait Information {
    fn name(&self) -> &str;
    fn description(&self) -> String;
}