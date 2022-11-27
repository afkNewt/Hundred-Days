use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ItemType {
    Resource,
    Building,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ManualAction {
    Buy { buy_price: i32 },
    Sell { sell_price: i32 },
    Construct { build_cost: HashMap<String, i32> },
    Deconstruct { item_gain: HashMap<String, i32> },
}

impl ToString for ManualAction {
    fn to_string(&self) -> String {
        match self {
            ManualAction::Buy { buy_price: _ } => return "Buy".to_string(),
            ManualAction::Sell { sell_price: _ } => return "Sell".to_string(),
            ManualAction::Construct { build_cost: _ } => return "Construct".to_string(),
            ManualAction::Deconstruct { item_gain: _ } => return "Deconstruct".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum DailyAction {
    Produce {
        item_production: HashMap<String, i32>,
    },
    Reduction {
        item_reduction: HashMap<String, i32>,
    },
}

impl DailyAction {
    pub fn description(&self) -> String {
        match self {
            DailyAction::Produce { item_production } => {
                let mut output = "Produces daily:".to_string();

                for (item_name, amount) in item_production {
                    output = format!("{output}\n{item_name}: {amount}")
                }

                return output;
            },
            DailyAction::Reduction { item_reduction } => {
                let mut output = "Reduces daily:".to_string();

                for (item_name, amount) in item_reduction {
                    output = format!("{output}\n{item_name}: {amount}")
                }

                return output;
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GlobalAction {
    PassDay,
}

impl ToString for GlobalAction {
    fn to_string(&self) -> String {
        match self {
            GlobalAction::PassDay => return "Pass Day".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub amount: i32,
    pub r#type: ItemType,
    pub industries: Vec<String>,
    pub manual_actions: Vec<ManualAction>,
    pub daily_actions: Vec<DailyAction>,
}

impl Item {
    fn industries_to_string(&self) -> String {
        let output = "Industries:".to_string();

        for industry in &self.industries {
            format!("{output}\n{industry}");
        }

        return output;
    }

    pub fn information(&self) -> String {
        let mut output = format!("Name: {}\nAmount: {}\nIndustries: {}\n",
            self.name, self.amount, self.industries_to_string()
        );

        for action in &self.daily_actions {
            output = format!("{output}\n{}", action.description());
        }

        return output;
    }
}

pub fn use_manual_action(
    item_name: String,
    game: &mut Game,
    action: &ManualAction,
    activation_amount: i32,
) -> String {
    // make sure key is valid
    if !game.items.contains_key(&item_name) {
        return String::new();
    };

    match action {
        ManualAction::Buy { buy_price: price } => {
            let Some(item) = game.items.get_mut(&item_name) else {
                return String::new();
            };

            if game.currency < price * activation_amount {
                return format!(
                    "Can only buy {} {item_name}",
                    price * activation_amount - game.currency
                );
            }

            game.currency -= price * activation_amount;
            item.amount += activation_amount;
            return format!(
                "Bought {activation_amount} {item_name} for {} currency",
                price * activation_amount
            );
        }
        ManualAction::Sell { sell_price: price } => {
            let Some(item) = game.items.get_mut(&item_name) else {
                return String::new();
            };

            if item.amount < activation_amount {
                return format!("Not enough {item_name} to sell {activation_amount} {item_name}");
            }

            item.amount -= activation_amount;
            game.currency += activation_amount * price;

            return format!(
                "Sold {activation_amount} {item_name} for {} currency",
                activation_amount * price
            );
        }
        ManualAction::Construct { build_cost: price } => {
            let mut output = format!("Couldnt purchase {activation_amount} {item_name}, need:");
            let mut amount_can_construct = 0;

            // see how many we can construct
            for (item_name, amount) in price {
                let Some(item) = game.items.get(item_name) else {
                continue;
                };

                if item.amount - amount * activation_amount < 0 {
                    output = format!(
                        "{output}\n{} more {item_name}",
                        amount * activation_amount - item.amount
                    );
                } else {
                    amount_can_construct += 1;
                }
            }

            // if we cant construct enough, return
            if !amount_can_construct == activation_amount {
                output = format!("{output}\nCan only construct {amount_can_construct} {item_name}");
                return output;
            }

            // action is possible
            output = format!("Constructed {activation_amount} {item_name} for:");

            for (item_name, amount) in price {
                let Some(item) = game.items.get_mut(item_name) else {
                continue;
                };

                item.amount -= amount * activation_amount;
                output = format!("{output}{item_name}: {}", amount * activation_amount);
            }

            // we can use unwarp because we make sure
            // the key is valid at the start of the function
            game.items.get_mut(&item_name).unwrap().amount += activation_amount;
            return output;
        }
        ManualAction::Deconstruct { item_gain: price } => {
            let Some(item) = game.items.get_mut(&item_name) else {
                return String::new();
            };

            if item.amount < activation_amount {
                return format!(
                    "Not enough {item_name} to deconstruct {activation_amount} {item_name}"
                );
            }

            let mut output = format!("Deconstructed {activation_amount} {item_name} and recouped:");
            for (item_name, amount) in price {
                let Some(item) = game.items.get_mut(item_name) else {
                    continue;
                };

                output = format!("{output}\n{item_name}: {amount}");
                item.amount -= amount;
            }

            return output;
        }
    }
}

pub fn use_passive_action(item_name: String, game: &mut Game, action: &DailyAction) -> String {
    // make sure key is valid
    if !game.items.contains_key(&item_name) {
        return String::new();
    };

    match action {
        DailyAction::Produce {
            item_production: prod,
        } => {
            let mut output = format!("{item_name} produced:");
            for (item_name, amount) in prod {
                let Some(item) = game.items.get_mut(item_name) else {
                    continue;
                };

                output = format!("{item_name}: {amount}");
                item.amount += amount;
            }

            return output;
        }
        DailyAction::Reduction {
            item_reduction: cost,
        } => {
            let mut output = format!("{item_name} took:");

            for (item_name, amount) in cost {
                let Some(item) = game.items.get_mut(item_name) else {
                    continue;
                };

                let mut amount_reduced = 0;
                item.amount -= amount;

                if item.amount < 0 {
                    amount_reduced += item.amount.abs();
                    item.amount += item.amount.abs();
                }

                output = format!("{output}\n{item_name}: {amount_reduced}");
            }

            return output;
        }
    }
}

pub fn use_global_action(game: &mut Game, action: &GlobalAction) -> String {
    match action {
        GlobalAction::PassDay => {
            for (item_name, item) in game.items.clone() {
                for action in &item.daily_actions {
                    use_passive_action(item_name.to_string(), game, action);
                }
            }

            game.current_day -= 1;

            return format!("It is the dawn of the {} Day", game.current_day);
        }
    }
}

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
    pub fn generate_from_toml() -> Self {
        let file_path = "hundred_days.toml";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let game: Game = toml::from_str(&contents).unwrap();

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
