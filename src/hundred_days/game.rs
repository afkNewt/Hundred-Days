use std::{collections::HashMap, fs};

use serde::Deserialize;

use super::{action::{global::GlobalAction, manual::ManualAction, ItemAction}, item::Item};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Game {
    pub starting_day: i32,
    pub current_day: i32,

    pub industries: Vec<String>,
    pub global_actions: Vec<GlobalAction>,

    pub currency: i32,
    pub items: HashMap<String, Item>,
}

impl Game {
    pub fn generate_from_json() -> Self {
        let file_path = "hundred_days.json";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let game: Game = serde_json::from_str(&contents).unwrap();

        return game;
    }

    pub fn net_worth(&self) -> i32 {
        // all items that can be will be turned into
        // currency in this copy
        let mut cash_game = self.clone();

        for (item_name, item) in cash_game.items.clone() {
            for action in item.manual_actions.clone() {
                match action {
                    ManualAction::Sell { sell_price: _ } => {
                        while cash_game.items.get(&item_name).unwrap().amount > 0 {
                            action.activate(item_name.clone(), &mut cash_game, 1);
                        }
                    },
                    ManualAction::Deconstruct { item_gain: _ } => {
                        while cash_game.items.get(&item_name).unwrap().amount > 0 {
                            action.activate(item_name.clone(), &mut cash_game, 1);
                        }
                    },
                    _ => continue,
                }
            }
        }

        return cash_game.currency;
    }
}
