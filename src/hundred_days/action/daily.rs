use super::{Deserialize, Game, HashMap, Information};

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

impl DailyAction {
    pub fn activate(&self, item: String, game: &mut Game, amount: i32) -> String {
        if !game.items.contains_key(&item) {
            return String::new();
        };

        match self {
            DailyAction::Produce { item_production } => {
                let mut output = format!("{} produced:", item);
                let item_amount = game.items.get(&item).unwrap().amount;

                for (item_name, prod) in item_production {
                    let Some(produced_item) = game.items.get_mut(item_name) else {
                        continue;
                    };

                    produced_item.amount += amount * prod * item_amount;
                    output = format!(
                        "{output}\n{}: {} ({})",
                        item_name,
                        amount * prod * item_amount,
                        produced_item.amount
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
