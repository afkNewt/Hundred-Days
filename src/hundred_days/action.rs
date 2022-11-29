use super::game::Game;

pub mod daily;
pub mod global;
pub mod manual;

pub trait Information {
    fn name(&self) -> &str;
    fn description(&self) -> String;
}

pub trait ItemAction {
    fn activate(&self, item: String, game: &mut Game, amount: i32) -> String;
}

pub trait ItemlessAction {
    fn activate(&self, game: &mut Game, amount: i32) -> String;
}
