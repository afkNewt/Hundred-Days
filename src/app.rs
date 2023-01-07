use tui::widgets::ListState;

use crate::hundred_days::{action::{GameState, active::Active, Action}, item::ItemCategory};

#[derive(Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl StatefulList {
    fn reset_state(&mut self) {
        if self.items.is_empty() {
            self.state = ListState::default();
        } else {
            self.state.select(Some(0));
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Default for StatefulList {
    fn default() -> Self {
        let mut default_state = ListState::default();
        default_state.select(Some(0));

        return StatefulList {
            state: default_state.clone(),
            items: Vec::new(),
        };
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Table {
    Resources,
    Buildings,
    Industry,
    Actions,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SelectionMode {
    Table,
    Item,
}

pub struct App {
    pub resource_table: StatefulList,
    pub building_table: StatefulList,
    pub industry_table: StatefulList,
    pub action_table: StatefulList,

    pub info: String,
    pub extra_info: String,

    // number of times to call an action
    // when an action is activated
    pub activation_amount: i32,

    // selected industry
    pub industry: String,
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

        let mut app = App {
            resource_table: StatefulList::default(),
            building_table: StatefulList::default(),
            industry_table: {
                let mut ind = StatefulList::default();
                ind.items = game.industries.clone();
                ind
            },
            action_table: StatefulList::default(),
            info: String::new(),
            extra_info: String::new(),
            activation_amount: 1,
            industry: game.industries.first().unwrap().to_string(),
            selected_table: Table::Resources,
            selected_item: String::new(),
            selection_mode: SelectionMode::Item,
            game_state: game,
        };

        app.update_building_list();
        app.update_resources_list();
        app.update_actions_list();
        app.update_info_table();
        return app;
    }

    fn selected_action(&self) -> Option<Active> {
        let Some(action_index) = self.action_table.state.selected() else {
            return None;
        };

        let Some(item) = self.game_state.items.get(&self.selected_item) else {
            return None;
        };

        return Some(item.actions_active[action_index].clone());
    }

    fn update_resources_list(&mut self) {
        self.resource_table.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| {
                if item.category == ItemCategory::Resource
                    && item.industries.contains(&self.industry)
                {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();

        self.resource_table.reset_state();
    }

    fn update_building_list(&mut self) {
        self.building_table.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| {
                if item.category == ItemCategory::Building
                    && item.industries.contains(&self.industry)
                {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();

        self.building_table.reset_state();
    }

    fn update_actions_list(&mut self) {
        let Some(item) = self.game_state.items.get(&self.selected_item) else {
            return;
        };

        self.action_table.items = item
                .actions_active
                .iter()
                .map(|action| action.name().to_string())
                .collect::<Vec<String>>();

        self.action_table.reset_state();
    }

    fn update_info_table(&mut self) {
        let Some(item) = self.game_state.items.get(&self.selected_item) else {
            self.info = String::new();
            return;
        };

        self.info = item.information();
    }

    pub fn navigate(&mut self, up: bool) {
        if self.selection_mode == SelectionMode::Table {
            return;
        }

        match self.selected_table {
            Table::Resources => {
                if up {
                    self.resource_table.next();
                } else {
                    self.resource_table.previous();
                }

                self.selected_item = self.resource_table.items
                    [self.resource_table.state.selected().unwrap()]
                .clone();

                self.update_actions_list();
                self.update_info_table();
            }
            Table::Buildings => {
                if up {
                    self.building_table.next();
                } else {
                    self.building_table.previous();
                }

                self.selected_item = self.building_table.items
                    [self.building_table.state.selected().unwrap()]
                .clone();

                self.update_actions_list();
                self.update_info_table();
            }
            Table::Industry => {
                if up {
                    self.industry_table.next();
                } else {
                    self.industry_table.previous();
                }

                if let Some(selected) = self.industry_table.state.selected() {
                    self.industry = self.industry_table.items[selected].clone();
                }

                self.update_building_list();
                self.update_resources_list();
            }
            Table::Actions => {
                if up {
                    self.action_table.next();
                } else {
                    self.action_table.previous();
                }
            }
        }
    }

    pub fn change_tab(&mut self, new_table: Table) {
        self.selected_table = new_table;
    }

    pub fn alternate_selection_mode(&mut self) {
        match self.selection_mode {
            SelectionMode::Table => {
                self.selection_mode = SelectionMode::Item;

                match self.selected_table {
                    Table::Resources => {
                        self.selected_item = self.resource_table.items
                            [self.resource_table.state.selected().unwrap()]
                        .clone()
                    }
                    Table::Buildings => {
                        self.selected_item = self.building_table.items
                            [self.building_table.state.selected().unwrap()]
                        .clone()
                    }
                    _ => {}
                }

                self.update_info_table();
                self.update_actions_list();
            }
            SelectionMode::Item => {
                self.selection_mode = SelectionMode::Table;
            }
        }
    }

    pub fn call_selected_action(&mut self) {
        let Some(action) = self.selected_action() else {
            self.extra_info = "Could not find action".to_string();
            return;
        };

        self.extra_info = action.activate(
            self.selected_item.clone(),
            &mut self.game_state,
            self.activation_amount,
        );
        self.update_info_table();
    }
}
