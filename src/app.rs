use crate::hundred_days::{
    action::{active::Active, Action, GameState},
    item::ItemCategory,
};

#[derive(Clone)]
pub struct List {
    pub items: Vec<String>,
}

impl Default for List {
    fn default() -> Self {
        return List { items: Vec::new() };
    }
}

#[derive(PartialEq, Clone)]
pub struct HistoryItem {
    pub description: String,
    pub amount: i32,
}

impl HistoryItem {
    pub fn new(description: String, amount: i32) -> Self {
        Self {
            description,
            amount,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Table {
    Resources,
    Buildings,
    Actions,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct App {
    pub resource_table: List,
    pub building_table: List,

    pub selection_index: usize,

    pub history_limit: usize,
    pub history: Vec<HistoryItem>,

    // number of times to call an action
    // when an action is activated
    pub activation_amount: i32,

    // selected tab
    pub selected_table: Table,
    // last selected item
    pub selected_item: String,

    pub game_state: GameState,
}

impl App {
    pub fn new() -> App {
        let game = GameState::generate_from_json();
        let first_item = game
            .items
            .iter()
            .find(|(_, i)| i.category == ItemCategory::Resource)
            .and_then(|(s, _)| Some(s.to_owned()));

        let mut app = App {
            resource_table: List::default(),
            building_table: List::default(),
            selection_index: 0,
            activation_amount: 1,
            selected_table: Table::Resources,
            selected_item: first_item.unwrap_or_default(),
            game_state: game,
            history_limit: 3,
            history: Vec::new(),
        };

        app.update_building_list();
        app.update_resources_list();
        return app;
    }

    fn selected_action(&self) -> Option<Active> {
        let Some(item) = self.game_state.items.get(&self.selected_item) else {
            return None;
        };

        return Some(item.actions_active[self.selection_index].clone());
    }

    fn add_history_item(&mut self, history_item: HistoryItem) {
        let first_item = self.history.get(0);
        let Some(first_item) = first_item else {
            self.history.push(history_item);
            return;
        };

        if first_item.description == history_item.description {
            self.history[0].amount += history_item.amount;
        } else {
            self.history.insert(0, history_item);
        }

        while self.history.len() > self.history_limit {
            self.history.pop();
        }
    }

    fn update_resources_list(&mut self) {
        self.resource_table.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| item.category == ItemCategory::Resource)
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();
    }

    fn update_building_list(&mut self) {
        self.building_table.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| item.category == ItemCategory::Building)
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();
    }

    pub fn currently_selected_item_name(&self) -> Option<String> {
        match self.selected_table {
            Table::Resources => self.resource_table.items.get(self.selection_index).cloned(),
            Table::Buildings => self.building_table.items.get(self.selection_index).cloned(),
            _ => None,
        }
    }

    pub fn navigate(&mut self, direction: Direction) {
        let mut selection_index_wrapping_add = |amount: i32| {
            let max = match self.selected_table {
                Table::Resources => self.resource_table.items.len(),
                Table::Buildings => self.building_table.items.len(),
                Table::Actions => {
                    let selected_item = self.selected_item.clone();
                    let Some(item) = self.game_state.items.get(&selected_item) else {
                        return;
                    };

                    item.actions_active.len()
                }
            };

            let added = self.selection_index as i32 + amount;
            self.selection_index = if (0..max).contains(&(added as usize)) {
                added as usize
            } else {
                if added > 0 {
                    0
                } else {
                    max - 1
                }
            };
        };

        match direction {
            Direction::Up => selection_index_wrapping_add(-1),
            Direction::Down => selection_index_wrapping_add(1),
            Direction::Left => match self.selected_table {
                Table::Actions => self.change_tab(Table::Buildings),
                Table::Buildings => self.change_tab(Table::Resources),
                Table::Resources => self.change_tab(Table::Actions),
            },
            Direction::Right => match self.selected_table {
                Table::Buildings => self.change_tab(Table::Actions),
                Table::Resources => self.change_tab(Table::Buildings),
                Table::Actions => self.change_tab(Table::Resources),
            },
        }

        let Some(selected_item_name) = self.currently_selected_item_name() else {
            return;
        };

        self.selected_item = selected_item_name;
    }

    pub fn change_tab(&mut self, new_table: Table) {
        self.selected_table = new_table;
        self.selection_index = 0;

        if self.selected_table == Table::Resources || self.selected_table == Table::Buildings {
            if let Some(selected_item_name) = self.currently_selected_item_name() {
                self.selected_item = selected_item_name;
            }
        }
    }

    pub fn call_selected_action(&mut self) {
        let Some(action) = self.selected_action() else {
            self.add_history_item(HistoryItem::new("Could not find action".to_string(), 1));
            return;
        };

        let action_desc = action.activate(
            self.selected_item.clone(),
            &mut self.game_state,
            self.activation_amount,
        );

        self.add_history_item(HistoryItem::new(action_desc, self.activation_amount));
    }
}
