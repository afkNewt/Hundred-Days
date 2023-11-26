use std::{collections::HashMap, fs};

use serde::Deserialize;

use super::{
    action::{active::Active, Action},
    item::Item,
};

#[derive(Deserialize, Clone, PartialEq)]
pub struct GameState {
    pub day: i32,
    pub currency: i32,
    pub items: HashMap<String, Item>,
}

impl GameState {
    pub fn generate_from_json() -> Self {
        let file_path = "hundred_days.json";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let game: GameState = serde_json::from_str(&contents).unwrap();

        return game;
    }

    pub fn pass_day(&mut self, amount: i32) {
        self.day -= amount;

        self.clone().items.values().for_each(|i| {
            i.actions_passive.iter().for_each(|p| {
                p.activate(i.name.clone(), self, amount);
            })
        });
    }

    pub fn net_worth(&self) -> i32 {
        // all items that can be will be turned into
        // currency in this copy
        let mut cash_game = self.clone();

        for (item_name, item) in cash_game.items.clone() {
            for action in item.actions_active.clone() {
                match action {
                    Active::Sell { sell_price: _ } => {
                        while cash_game.items.get(&item_name).unwrap().amount > 0 {
                            action.activate(item_name.clone(), &mut cash_game, 1);
                        }
                    }
                    Active::Deconstruct { item_gain: _ } => {
                        while cash_game.items.get(&item_name).unwrap().amount > 0 {
                            action.activate(item_name.clone(), &mut cash_game, 1);
                        }
                    }
                    _ => continue,
                }
            }
        }

        return cash_game.currency;
    }
}
