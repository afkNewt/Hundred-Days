use super::{
    action::{daily::DailyAction, manual::ManualAction, Information},
    Deserialize,
};

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
    pub manual_actions: Vec<ManualAction>,
    pub daily_actions: Vec<DailyAction>,
}

impl Item {
    pub fn information(&self) -> String {
        let industries: String = self.industries.iter().map(|i| format!("\n{i}")).collect();

        let manual_action_descriptions: String = self
            .manual_actions
            .iter()
            .map(|a| format!("{}\n", a.description()))
            .collect();

        let daily_action_descriptions: String = self
            .daily_actions
            .iter()
            .map(|a| format!("{}\n", a.description()))
            .collect();

        return format!(
            "Name: {}\nAmount: {}\nIndustries: {industries}\n\n{}{}",
            self.name, self.amount, manual_action_descriptions, daily_action_descriptions
        );
    }
}
