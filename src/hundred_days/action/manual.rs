use crate::hundred_days::game::Game;
pub use serde::Deserialize;
pub use std::{collections::HashMap, fs};

use super::Information;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ManualAction {
    Buy { buy_price: i32 },
    Sell { sell_price: i32 },
    Construct { build_cost: HashMap<String, i32> },
    Deconstruct { item_gain: HashMap<String, i32> },
}

impl Information for ManualAction {
    fn name(&self) -> &str {
        match self {
            ManualAction::Buy { buy_price: _ } => return "Buy",
            ManualAction::Sell { sell_price: _ } => return "Sell",
            ManualAction::Construct { build_cost: _ } => return "Construct",
            ManualAction::Deconstruct { item_gain: _ } => return "Deconstruct",
        }
    }

    fn description(&self) -> String {
        match self {
            ManualAction::Buy { buy_price } => return format!("Buy Price: {buy_price}"),
            ManualAction::Sell { sell_price } => return format!("Sell Price: {sell_price}"),
            ManualAction::Construct { build_cost } => {
                let cost: String = build_cost
                    .iter()
                    .map(|(name, amount)| format!("{name}: {amount}\n"))
                    .collect();

                return format!("Contsruction Cost:\n{}", cost);
            }
            ManualAction::Deconstruct { item_gain } => {
                let gain: String = item_gain
                    .iter()
                    .map(|(name, amount)| format!("{name}: {amount}\n"))
                    .collect();

                return format!("Deconstruction Recouperation:\n{}", gain);
            }
        }
    }
}

impl ManualAction {
    pub fn activate(
        &self,
        item: String,
        game: &mut Game,
        amount: i32,
    ) -> String {
        if !game.items.contains_key(&item) {
            return String::new();
        };

        match self {
            ManualAction::Buy { buy_price } => {
                let item = game.items.get_mut(&item).unwrap();

                if game.currency < buy_price * amount {
                    return format!(
                        "Can only buy {} {}",
                        game.currency / buy_price,
                        item.name
                    );
                }

                game.currency -= buy_price * amount;
                item.amount += amount;
                return format!(
                    "Bought {amount} {} for {} currency",
                    item.name,
                    buy_price * amount
                );
            }
            ManualAction::Sell { sell_price } => {
                let item = game.items.get_mut(&item).unwrap();

                if item.amount < amount {
                    return format!("Not enough {0} to sell {amount} {0}", item.name);
                }

                item.amount -= amount;
                game.currency += amount * sell_price;

                return format!(
                    "Sold {amount} {} for {} currency",
                    item.name,
                    amount * sell_price,
                );
            }
            ManualAction::Construct { build_cost } => {
                let mut output = format!("Could not construct {amount} {}, need:", item);
                let mut can_construct = true;

                for (item_name, cost) in build_cost {
                    let Some(cost_item) = game.items.get(item_name) else {
                        continue;
                    };

                    if (cost_item.amount - cost * amount).is_negative() {
                        can_construct = false;
                        output = format!("{output}\n{} more {item_name}", cost * amount - cost_item.amount);
                    }
                }

                if !can_construct {
                    return output;
                }

                output = format!("Constructed {amount} {} for:", item);

                for (item_name, cost) in build_cost {
                    let Some(cost_item) = game.items.get_mut(item_name) else {
                        continue;
                    };

                    cost_item.amount -= cost * amount;
                    output = format!("{output}\n{item_name}: {}", cost * amount);
                }

                let item = game.items.get_mut(&item).unwrap();

                item.amount += amount;
                return output;
            }
            ManualAction::Deconstruct { item_gain } => {
                let item = game.items.get_mut(&item).unwrap();
                
                if item.amount < amount {
                    return format!(
                        "Not enough {0} to deconstruct {amount} {0}", item.name
                    );
                }

                let mut output =
                    format!("Deconstructed {amount} {} and recouped:", item.name);

                item.amount -= amount;
                for (item_name, amount) in item_gain {
                    let Some(item) = game.items.get_mut(item_name) else {
                        continue;
                    };

                    output = format!("{output}\n{item_name}: {amount}");
                    item.amount += amount;
                }

                return output;
            }
        }
    }
}
