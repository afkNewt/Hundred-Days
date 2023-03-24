use serde::Deserialize;
use std::collections::HashMap;

use super::{Action, GameState};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Active {
    Buy { buy_price: i32 },
    Sell { sell_price: i32 },
    Construct { build_cost: HashMap<String, i32> },
    Deconstruct { item_gain: HashMap<String, i32> },
}

impl Action for Active {
    fn name(&self) -> &str {
        match self {
            Active::Buy { buy_price: _ } => "Buy",
            Active::Sell { sell_price: _ } => "Sell",
            Active::Construct { build_cost: _ } => "Construct",
            Active::Deconstruct { item_gain: _ } => "Deconstruct",
        }
    }

    fn description(&self) -> String {
        match self {
            Active::Buy { buy_price } => format!("Buy Price: {buy_price}"),
            Active::Sell { sell_price } => format!("Sell Price: {sell_price}"),
            Active::Construct { build_cost } => {
                format!(
                    "Construction Cost:\n{}",
                    build_cost
                        .iter()
                        .map(|(name, amount)| format!("{name}: {amount}\n"))
                        .collect::<String>()
                )
            }
            Active::Deconstruct { item_gain } => {
                format!(
                    "Deconstruction Recouperation:\n{}",
                    item_gain
                        .iter()
                        .map(|(name, amount)| format!("{name}: {amount}\n"))
                        .collect::<String>()
                )
            }
        }
    }

    fn activate(&self, item_name: String, game: &mut GameState, amount: i32) -> String {
        if !game.items.contains_key(&item_name) {
            return "Action could not find associated item".to_string();
        };

        let max_activates = self.max_activate(item_name.clone(), game);
        if max_activates < amount {
            return format!("Can only be called {max_activates} more times");
        }

        match self {
            Active::Buy { buy_price } => {
                game.items.get_mut(&item_name).unwrap().amount += amount;
                game.currency -= buy_price * amount;

                return format!("Purchased {amount} {item_name} for {}", buy_price * amount);
            }
            Active::Sell { sell_price } => {
                game.items.get_mut(&item_name).unwrap().amount -= amount;
                game.currency += sell_price * amount;

                return format!("Sold {amount} {item_name} for {}", sell_price * amount);
            }
            Active::Construct { build_cost } => {
                let mut cost_string = String::new();
                for (name, cost) in build_cost {
                    game.items.get_mut(name).unwrap().amount -= cost * amount;
                    cost_string = format!("{cost_string}\n{name}: {cost}");
                }

                game.items.get_mut(&item_name).unwrap().amount += amount;

                return format!("Constructed {amount} {item_name} for:{cost_string}");
            }
            Active::Deconstruct { item_gain } => {
                let mut gain_string = String::new();
                for (name, gain) in item_gain {
                    game.items.get_mut(name).unwrap().amount += gain * amount;
                    gain_string = format!("{gain_string}\n{name}: {gain}");
                }

                game.items.get_mut(&item_name).unwrap().amount -= amount;

                return format!("Deconstructed {amount} {item_name} for:{gain_string}");
            }
        }
    }

    fn max_activate(&self, item_name: String, game: &mut GameState) -> i32 {
        if !game.items.contains_key(&item_name) {
            return 0;
        };

        match self {
            Active::Buy { buy_price } => {
                return game.currency / buy_price;
            }
            Active::Sell { sell_price: _ } => {
                return game.items.get(&item_name).unwrap().amount;
            }
            Active::Construct { build_cost } => {
                return build_cost
                    .iter()
                    .map(|(item_name, cost)| game.items.get(item_name).unwrap().amount / cost)
                    .min()
                    .unwrap_or(0);
            }
            Active::Deconstruct { item_gain: _ } => {
                return game.items.get(&item_name).unwrap().amount;
            }
        }
    }
}
