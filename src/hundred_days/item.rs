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
        let industries: String = self.industries.iter().map(|i| format!("\n{i}")).collect();

        let active_action_descriptions: String = self
            .actions_active
            .iter()
            .map(|a| format!("{}\n", a.description()))
            .collect();

        let passive_action_descriptions: String = self
            .actions_passive
            .iter()
            .map(|a| format!("{}\n", a.description()))
            .collect();

        return format!(
            "Name: {}\nAmount: {}\nIndustries: {industries}\n\n{}{}",
            self.name, self.amount, active_action_descriptions, passive_action_descriptions
        );
    }
}
