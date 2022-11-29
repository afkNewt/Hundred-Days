use std::{collections::HashMap, fs};

use serde::Deserialize;

use super::{action::{global::GlobalAction, manual::ManualAction}, item::Item};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Game {
    pub starting_day: i32,
    pub current_day: i32,

    pub black_market_days: Vec<i32>,

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
        let mut net_worth = self.currency;

        for (_, item) in self.items.iter() {
            let deconstruct: Vec<&HashMap<String, i32>> = item
                .manual_actions
                .iter()
                .filter_map(|a| match a {
                    ManualAction::Deconstruct { item_gain: price } => Some(price),
                    _ => None,
                })
                .collect();

            // Item can be deconstructed
            if let Some(deconstruct) = deconstruct.first() {
                for (item_name, amount) in *deconstruct {
                    let Some(item) = self.items.get(item_name) else {
                        continue;
                    };

                    let sell: Vec<&i32> = item
                        .manual_actions
                        .iter()
                        .filter_map(|a| match a {
                            ManualAction::Sell { sell_price: price } => Some(price),
                            _ => None,
                        })
                        .collect();

                    // Item recieved from deconstruction
                    // can be sold
                    if let Some(price) = sell.first() {
                        net_worth += *price * amount;
                    }
                }
            }

            let sell: Vec<&i32> = item
                .manual_actions
                .iter()
                .filter_map(|a| match a {
                    ManualAction::Sell { sell_price: price } => Some(price),
                    _ => None,
                })
                .collect();

            // Item can be sold
            if let Some(price) = sell.first() {
                net_worth += *price * item.amount;
            }
        }

        return net_worth;
    }
}
