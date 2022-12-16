pub use serde::Deserialize;
pub use std::{collections::HashMap, fs};
use crate::hundred_days::game::Game;

use super::Information;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GlobalAction {
    PassDay,
}

impl Information for GlobalAction {
    fn name(&self) -> &str {
        match self {
            GlobalAction::PassDay => return "Pass Day",
        }
    }

    fn description(&self) -> String {
        match self {
            GlobalAction::PassDay => return "Pass Day".to_string(),
        }
    }
}

impl GlobalAction {
    pub fn activate(&self, game: &mut Game, mut amount: i32) -> String {
        match self {
            GlobalAction::PassDay => {
                let mut output = "".to_string();

                if game.current_day - amount < -1 {
                    amount -= (game.current_day - amount).abs();
                }

                // we can use clone, because we just want the action
                for (item_name, item) in game.items.clone() {
                    for action in &item.daily_actions {
                        output = format!("{output}{}\n\n", action.activate(item_name.clone(), game, amount));
                    }
                }
    
                game.current_day -= amount;
    
                return output;
            }
        }
    }
}