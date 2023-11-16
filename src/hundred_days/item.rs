use serde::Deserialize;

use super::action::{active::Active, passive::Passive, Action};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ItemCategory {
    Resource,
    Building,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub amount: i32,
    pub category: ItemCategory,
    pub industries: Vec<String>,
    pub actions_active: Vec<Active>,
    pub actions_passive: Vec<Passive>,
}

impl Item {
    pub fn information(&self) -> String {
        format!("Name: {}\nAmount: {}\n\n", self.name, self.amount)
    }
}
