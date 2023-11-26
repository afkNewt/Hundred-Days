use serde::Deserialize;
use std::collections::HashMap;

use super::{Action, GameState};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Passive {
    Produce {
        item_production: HashMap<String, i32>,
    },
    Reduce {
        item_reduction: HashMap<String, i32>,
    },
}

impl Action for Passive {
    fn name(&self) -> &str {
        match self {
            Passive::Produce { item_production: _ } => "Produce",
            Passive::Reduce { item_reduction: _ } => "Reduce",
        }
    }

    fn description(&self) -> String {
        match self {
            Passive::Produce { item_production } => {
                format!(
                    "Produces daily:\n{}",
                    item_production
                        .iter()
                        .map(|(name, amount)| format!("{name}: {amount}\n"))
                        .collect::<String>()
                )
            }
            Passive::Reduce { item_reduction } => {
                format!(
                    "Reduces daily:\n{}",
                    item_reduction
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
        };

        match self {
            Passive::Produce { item_production } => {
                for (name, production) in item_production {
                    game.items.get_mut(name).unwrap().amount += production * amount;
                }

                return format!(
                    "Produced {{ {}}}",
                    item_production
                        .iter()
                        .map(|(s, i)| { format!("{s}: {i} ") })
                        .collect::<String>()
                );
            }
            Passive::Reduce { item_reduction } => {
                for (name, reduction) in item_reduction {
                    game.items.get_mut(name).unwrap().amount -= reduction * amount;
                }

                return format!(
                    "Reduced {{ {}}}",
                    item_reduction
                        .iter()
                        .map(|(s, i)| { format!("{s}: {i} ") })
                        .collect::<String>()
                );
            }
        }
    }

    fn max_activate(&self, item_name: String, game: &mut GameState) -> i32 {
        if !game.items.contains_key(&item_name) {
            return 0;
        };

        match self {
            Passive::Produce { item_production: _ } => {
                return i32::MAX;
            }
            Passive::Reduce { item_reduction } => {
                return item_reduction
                    .iter()
                    .map(|(item_name, cost)| game.items.get(item_name).unwrap().amount / cost)
                    .min()
                    .unwrap_or(0);
            }
        }
    }
}
