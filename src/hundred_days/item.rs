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
        format!(
            "Name: {}\nAmount: {}\nIndustries: {}\n\n{}{}",
            self.name,
            self.amount,
            self.industries
                .iter()
                .map(|i| format!("\n{i}"))
                .collect::<String>(),
            self.actions_active
                .iter()
                .map(|a| format!("{}\n", a.description()))
                .collect::<String>(),
            self.actions_passive
                .iter()
                .map(|a| format!("{}\n", a.description()))
                .collect::<String>(),
        )
    }
}
