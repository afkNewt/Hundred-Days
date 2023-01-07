pub use super::game::GameState;

pub mod active;
pub mod passive;

pub trait Action {
    fn name(&self) -> &str;
    fn description(&self) -> String;
    fn activate(&self, item_name: String, game: &mut GameState, amount: i32) -> String;
    fn max_activate(&self, item_name: String, game: &mut GameState) -> i32;
}
