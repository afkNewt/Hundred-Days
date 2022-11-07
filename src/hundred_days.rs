use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Industry {
    Mining,
    Logging,
    Farming,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub amount: f32,
    pub industry: Industry,
    pub actions: Vec<String>,

    pub price: i32,
}

impl Resource {
    pub fn information(&self) -> String {
        return format!(
            "name: {}\namount: {}\nindustry: {:?}\nprice: {}",
            self.name, self.amount, self.industry, self.price
        );
    }
}

pub fn resource_action(name: &str) -> fn(&mut Game, usize) -> String {
    match name {
        "buy" => |game, resource| {
            if game.currency < game.resources[resource].price {
                return format!(
                    "Need {} more currency",
                    game.resources[resource].price - game.currency
                );
            }

            game.currency -= game.resources[resource].price;
            game.resources[resource].amount += 1.0;
            return format!(
                "Purchased one {} for {} currency",
                game.resources[resource].name, game.resources[resource].price
            );
        },
        "sell" => |game, resource| {
            if game.resources[resource].amount < 1.0 {
                return "Need at least one of a resource to sell it".to_string();
            }

            game.resources[resource].amount -= 1.0;
            game.currency += game.resources[resource].price;
            return format!(
                "Sold one {} for {} currency",
                game.resources[resource].name, game.resources[resource].price
            );
        },
        _ => |_, _| return String::new(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Building {
    pub name: String,
    pub amount: i32,
    pub industry: Industry,
    pub actions: Vec<String>,

    pub production: Vec<(String, f32)>,
    pub cost: Vec<(String, f32)>,
}

impl Building {
    fn vec_to_string(vec: Vec<(String, f32)>) -> String {
        let mut output = String::new();

        for (name, amount) in vec {
            output = format!("{output}\n{name}: {amount}");
        }

        return output;
    }

    pub fn information(&self) -> String {
        return format!(
            "name: {}\namount: {}\nindustry: {:?}\n\nproduction: {}\n\ncost: {}",
            self.name,
            self.amount,
            self.industry,
            Self::vec_to_string(self.production.clone()),
            Self::vec_to_string(self.cost.clone())
        );
    }
}

pub fn building_action(name: &str) -> fn(&mut Game, usize) -> String {
    match name {
        "construct" => |game, building| {
            let mut can_build = true;
            let mut output = "Not enough resources, Need:".to_string();

            for (index, (resource, amount)) in game.buildings[building].cost.iter().enumerate() {
                if game.resources[index].amount < *amount {
                    can_build = false;
                    output = format!(
                        "{output}\n{} more {}",
                        *amount - game.resources[index].amount,
                        resource
                    );
                }
            }

            if !can_build {
                return output;
            }

            for (index, (_, amount)) in game.buildings[building].cost.iter().enumerate() {
                game.resources[index].amount -= amount;
            }
            game.buildings[building].amount += 1;

            return format!("Constructed one {}", game.buildings[building].name);
        },
        _ => |_, _| return String::new(),
    }
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub currency: i32,
    pub days: i32,

    pub actions: Vec<String>,

    pub resources: Vec<Resource>,
    pub buildings: Vec<Building>,
}

pub fn global_action(name: &str) -> fn(&mut Game) -> String {
    match name {
        "pass" => |game| -> String {
            let mut output = "Passed the day:".to_string();

            for building in &game.buildings {
                output = format!("{output}\n\n{}:", building.name);
                for (resource, amount) in building.production.iter() {
                    let amount = amount * building.amount as f32;

                    let res = game.resources.iter_mut().find(|r| r.name == *resource);
                    let Some(res) = res else {
                        continue;
                    };

                    res.amount += amount;
                    output = format!("{output}\n{} ({}) {}", res.amount, amount, resource);
                }
            }

            game.days -= 1;

            return output;
        },
        _ => |_| return String::new(),
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
}
