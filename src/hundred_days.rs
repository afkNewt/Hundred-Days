use serde::Deserialize;
use std::{collections::HashMap, fs};

pub trait Action: ToString {
    fn name(&self) -> &str;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RescourceAction {
    Buy { buy_price: i32 },
    Sell { sell_price: i32 },
}

impl ToString for RescourceAction {
    fn to_string(&self) -> String {
        match self {
            RescourceAction::Buy { buy_price: price } => format!("Buy Price: {price}"),
            RescourceAction::Sell { sell_price: price } => format!("Sell Price: {price}"),
        }
    }
}

impl Action for RescourceAction {
    fn name(&self) -> &str {
        match self {
            RescourceAction::Buy { buy_price: _ } => "Buy",
            RescourceAction::Sell { sell_price: _ } => "Sell",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub amount: f32,
    pub industries: Vec<String>,
    pub actions: Vec<RescourceAction>,
}

impl Resource {
    fn industry_as_string(&self) -> String {
        let mut output = String::new();

        for industry in &self.industries {
            output = format!("{output}\n{industry}");
        }

        format!("{output}\n");
        return output;
    }

    pub fn information(&self) -> String {
        let mut output = format!(
            "name: {}\namount: {}\n\nindustries: {}\n",
            self.name,
            self.amount,
            self.industry_as_string()
        );

        for action in &self.actions {
            output = format!("{output}\n{}", action.to_string());
        }

        return output;
    }
}

pub fn resource_action(game: &mut Game, resource_name: &String, action: RescourceAction) -> String {
    match action {
        RescourceAction::Buy { buy_price: price } => {
            let Some(resource) = game.resources.get_mut(resource_name) else {
                return String::new();
            };

            if game.currency < price {
                return format!("Need {} more currency", price - game.currency);
            }

            game.currency -= price;
            resource.amount += 1.0;
            return format!("Purchased one {} for {} currency", resource_name, price);
        }
        RescourceAction::Sell { sell_price: price } => {
            let Some(resource) = game.resources.get_mut(resource_name) else {
                return String::new();
            };

            if resource.amount < 1.0 {
                return "Need at least one of a resource to sell it".to_string();
            }

            resource.amount -= 1.0;
            game.currency += price;
            return format!("Sold one {} for {} currency", resource_name, price);
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum BuildingAction {
    Construct { build_cost: Vec<(String, f32)> },
}

impl ToString for BuildingAction {
    fn to_string(&self) -> String {
        match self {
            BuildingAction::Construct { build_cost } => {
                let mut output = format!("Build Cost:");

                for (name, amount) in build_cost {
                    output = format!("{output}\n{name}: {amount}");
                }

                format!("{output}\n");
                return output;
            }
        }
    }
}

impl Action for BuildingAction {
    fn name(&self) -> &str {
        match self {
            BuildingAction::Construct { build_cost: _ } => "Construct",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Building {
    pub name: String,
    pub amount: i32,
    pub industries: Vec<String>,
    pub actions: Vec<BuildingAction>,

    pub production: Vec<(String, f32)>,
}

impl Building {
    fn prod_as_string(&self) -> String {
        let mut output = String::new();

        for (name, amount) in &self.production {
            output = format!("{output}\n{name}: {amount}");
        }

        format!("{output}\n");
        return output;
    }

    fn industry_as_string(&self) -> String {
        let mut output = String::new();

        for industry in &self.industries {
            output = format!("{output}\n{industry}");
        }

        format!("{output}\n");
        return output;
    }

    pub fn information(&self) -> String {
        let mut output = format!(
            "name: {}\namount: {}\n\nindustry: {}\n\nproduction: {}\n",
            self.name,
            self.amount,
            self.industry_as_string(),
            self.prod_as_string(),
        );

        for action in &self.actions {
            output = format!("{output}\n{}", action.to_string());
        }

        return output;
    }
}

pub fn building_action(game: &mut Game, building_name: &str, action: BuildingAction) -> String {
    match action {
        BuildingAction::Construct { build_cost: cost } => {
            let Some(building) = game.buildings.get_mut(building_name) else {
                return String::new();
            };

            let mut can_build = true;
            let mut output = "Not enough resources, Need:".to_string();

            for (resource, amount) in &cost {
                if game.resources[resource].amount < *amount {
                    can_build = false;
                    output = format!(
                        "{output}\n{} more {}",
                        *amount - game.resources[resource].amount,
                        resource
                    );
                }
            }

            if !can_build {
                return output;
            }

            for (resource_name, amount) in &cost {
                let Some(resource) = game.resources.get_mut(resource_name) else {
                    continue;
                };
                resource.amount -= amount;
            }
            building.amount += 1;

            return format!("Constructed one {}", building_name);
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GlobalAction {
    Pass,
}

impl ToString for GlobalAction {
    fn to_string(&self) -> String {
        match self {
            GlobalAction::Pass => "Pass Day".to_string(),
        }
    }
}

impl Action for GlobalAction {
    fn name(&self) -> &str {
        match self {
            GlobalAction::Pass => "Pass Day",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub currency: i32,
    pub days: i32,

    pub industries: Vec<String>,

    pub global_actions: Vec<GlobalAction>,

    pub resources: HashMap<String, Resource>,
    pub buildings: HashMap<String, Building>,
}

pub fn global_action(game: &mut Game, action: GlobalAction) -> String {
    match action {
        GlobalAction::Pass => {
            let mut output = "Passed the day:".to_string();

            for (building_name, building) in &game.buildings {
                output = format!("{output}\n\n{}:", building_name);
                for (resource_name, amount) in building.production.iter() {
                    let amount = amount * building.amount as f32;

                    let Some(resource) = game.resources.get_mut(resource_name) else {
                        continue;
                    };

                    resource.amount += amount;
                    output = format!(
                        "{output}\n{} ({}) {}",
                        resource.amount, amount, resource_name
                    );
                }
            }

            game.days -= 1;

            return output;
        }
    }
}

impl Game {
    pub fn from_toml() -> Self {
        let file_path = "hundred_days.toml";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let game: Game = toml::from_str(&contents).unwrap();

        return game;
    }

    pub fn net_worth(&self) -> i32 {
        let mut net_worth = self.currency;

        for (_, resource) in &self.resources {
            for action in &resource.actions {
                match action {
                    RescourceAction::Sell { sell_price: price } => {
                        net_worth += price * resource.amount.floor() as i32;
                    }
                    _ => {}
                }
            }
        }

        return net_worth;
    }
}
