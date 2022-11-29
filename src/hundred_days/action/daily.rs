use crate::hundred_days::game::Game;
pub use serde::Deserialize;
pub use std::{collections::HashMap, fs};

use super::{Information, ItemAction};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum DailyAction {
    Produce {
        item_production: HashMap<String, i32>,
    },
    Reduction {
        item_reduction: HashMap<String, i32>,
    },
}

impl Information for DailyAction {
    fn name(&self) -> &str {
        match self {
            DailyAction::Produce { item_production: _ } => "Produce",
            DailyAction::Reduction { item_reduction: _ } => "Reduce",
        }
    }

    fn description(&self) -> String {
        match self {
            DailyAction::Produce { item_production } => {
                let production: String = item_production
                    .iter()
                    .map(|(name, amount)| format!("{name}: {amount}\n"))
                    .collect();

                return format!("Produces daily:\n{}", production);
            }
            DailyAction::Reduction { item_reduction } => {
                let production: String = item_reduction
                    .iter()
                    .map(|(name, amount)| format!("{name}: {amount}\n"))
                    .collect();

                return format!("Produces daily:\n{}", production);
            }
        }
    }
}

impl ItemAction for DailyAction {
    fn activate(&self, item: String, game: &mut Game, amount: i32) -> String {
        if !game.items.contains_key(&item) {
            return String::new();
        };

        match self {
            DailyAction::Produce { item_production } => {
                let mut output = format!("{} produced:", item);

                for (item_name, prod) in item_production {
                    let Some(item) = game.items.get_mut(item_name) else {
                        continue;
                    };

                    item.amount += amount * prod;
                    output = format!(
                        "{output}\n{}: {} ({})",
                        item_name,
                        amount * prod,
                        item.amount
                    );
                }

                return output;
            }
            DailyAction::Reduction { item_reduction } => {
                let mut output = format!("{} took:", item);

                for (item_name, cost) in item_reduction {
                    let Some(item) = game.items.get_mut(item_name) else {
                        continue;
                    };

                    let mut amount_to_reduce = amount * cost;
                    if (item.amount - amount_to_reduce).is_negative() {
                        amount_to_reduce += (item.amount - amount_to_reduce).abs()
                    }

                    output = format!("{output}\n{item_name}: {}", amount_to_reduce);
                    item.amount -= amount_to_reduce;
                }

                return output;
            }
        }
    }
}
