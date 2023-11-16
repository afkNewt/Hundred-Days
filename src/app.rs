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

#[derive(PartialEq, Copy, Clone)]
pub enum Table {
    Resources,
    Buildings,
    Actions,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SelectionMode {
    Table,
    Item,
}

pub struct App {
    pub resource_table: List,
    pub building_table: List,

    pub selection_index: usize,

    pub history: Vec<String>,

    // number of times to call an action
    // when an action is activated
    pub activation_amount: i32,

    // selected tab
    pub selected_table: Table,
    // last selected item
    pub selected_item: String,
    pub selection_mode: SelectionMode,

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
            selection_mode: SelectionMode::Item,
            game_state: game,
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

    pub fn navigate(&mut self, up: bool) {
        if self.selection_mode == SelectionMode::Table {
            return;
        }

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

        if up {
            if self.selection_index + 1 >= max {
                self.selection_index = 0;
            } else {
                self.selection_index += 1;
            }
        } else {
            if self.selection_index.checked_sub(1) == None {
                self.selection_index = max - 1;
            } else {
                self.selection_index -= 1;
            }
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

    pub fn alternate_selection_mode(&mut self) {
        match self.selection_mode {
            SelectionMode::Table => {
                self.selection_mode = SelectionMode::Item;
            }
            SelectionMode::Item => {
                self.selection_mode = SelectionMode::Table;
            }
        }
    }

    pub fn call_selected_action(&mut self) {
        let Some(action) = self.selected_action() else {
            self.history.push("Could not find action".to_string());
            return;
        };

        self.history.push(action.activate(
            self.selected_item.clone(),
            &mut self.game_state,
            self.activation_amount,
        ));
    }
}
