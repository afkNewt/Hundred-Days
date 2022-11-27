use tui::widgets::ListState;

use crate::hundred_days::{use_global_action, use_manual_action, Game, ItemType};

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

#[derive(PartialEq, Copy, Clone)]
pub enum Tab {
    Resources,
    Buildings,
    Industry,
    Actions,
}

#[derive(PartialEq, Clone)]
pub enum Item {
    Resource,
    Building,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SelectionMode {
    Tabs,
    Items,
}

#[derive(Clone)]
pub struct States {
    pub resource: StatefulList,
    pub building: StatefulList,
    pub industry: StatefulList,
    pub action: StatefulList,
}

pub struct App {
    pub table_states: States,
    pub info: String,
    pub extra_info: String,

    // selected industry
    pub industry: String,
    // selected tab
    pub selected_tab: Tab,
    // whether items or building was selected last
    pub selected_item: Item,
    pub selection_mode: SelectionMode,

    pub game_state: Game,
}

impl App {
    pub fn new() -> App {
        let mut default_state = ListState::default();
        default_state.select(Some(0));

        let state = StatefulList {
            state: default_state.clone(),
            items: Vec::new(),
        };

        let game = Game::generate_from_json();

        let industries = StatefulList {
            state: default_state.clone(),
            items: game.industries.clone(),
        };

        let mut app = App {
            table_states: States {
                resource: state.clone(),
                building: state.clone(),
                industry: industries,
                action: state.clone(),
            },
            info: String::new(),
            extra_info: String::new(),
            industry: game.industries.first().unwrap().to_string(),
            selected_tab: Tab::Resources,
            selection_mode: SelectionMode::Items,
            game_state: game,
            selected_item: Item::Resource,
        };
        app.update_building_list();
        app.update_resources_list();
        app.update_actions_list();
        app.update_info_table();
        return app;
    }

    fn selected_building(&self) -> &str {
        // gets index of selected building
        let selected = self
            .table_states
            .building
            .state
            .selected()
            .unwrap_or_default();
        // gets name of selected building
        let building_name = &self.table_states.building.items[selected];

        return building_name;
    }

    fn selected_resource(&self) -> &str {
        // gets index of selected resource
        let selected = self
            .table_states
            .resource
            .state
            .selected()
            .unwrap_or_default();
        // gets name of selected resource
        let resource_name = &self.table_states.resource.items[selected];

        return resource_name;
    }

    fn update_resources_list(&mut self) {
        self.table_states.resource.items = Vec::new();

        for (item_name, item) in &self.game_state.items {
            if item.r#type != ItemType::Resource {
                continue;
            }
            if item.industries.contains(&self.industry) {
                self.table_states.resource.items.push(item_name.to_string());
            }
        }

        self.table_states.resource.reset_state();
    }

    fn update_building_list(&mut self) {
        self.table_states.building.items = Vec::new();

        for (item_name, item) in &self.game_state.items {
            if item.r#type != ItemType::Building {
                continue;
            }
            if item.industries.contains(&self.industry) {
                self.table_states.building.items.push(item_name.to_string());
            }
        }

        self.table_states.building.reset_state();
    }

    fn update_actions_list(&mut self) {
        self.table_states.action.items = Vec::new();

        for action in &self.game_state.global_actions {
            self.table_states.action.items.push(action.name());
        }

        let selected_item;

        match self.selected_item {
            Item::Resource => {
                let Some(item) = self.game_state.items.get(self.selected_resource()) else {
                    return;
                };

                selected_item = item;
            }
            Item::Building => {
                let Some(item) = self.game_state.items.get(self.selected_building()) else {
                    return;
                };

                selected_item = item;
            }
        }

        for action in &selected_item.manual_actions {
            self.table_states.action.items.push(action.name());
        }

        self.table_states.action.reset_state();
    }

    fn update_info_table(&mut self) {
        self.info = String::new();

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.items.get(self.selected_resource()) else {
                    return;
                };

                self.info = resource.information();
            }
            Item::Building => {
                let Some(building) = self.game_state.items.get(self.selected_building()) else {
                    return;
                };

                self.info = building.information();
            }
        }
    }

    pub fn navigate(&mut self, up: bool) {
        if self.selection_mode == SelectionMode::Tabs {
            return;
        }

        match self.selected_tab {
            Tab::Resources => {
                if up {
                    self.table_states.resource.next();
                } else {
                    self.table_states.resource.previous();
                }

                self.update_actions_list();
                self.update_info_table();
            }
            Tab::Buildings => {
                if up {
                    self.table_states.building.next();
                } else {
                    self.table_states.building.previous();
                }

                self.update_actions_list();
                self.update_info_table();
            }
            Tab::Industry => {
                if up {
                    self.table_states.industry.next();
                } else {
                    self.table_states.industry.previous();
                }

                if let Some(selected) = self.table_states.industry.state.selected() {
                    self.industry = self.table_states.industry.items[selected].clone();
                }

                self.update_building_list();
                self.update_resources_list();
            }
            Tab::Actions => {
                if up {
                    self.table_states.action.next();
                } else {
                    self.table_states.action.previous();
                }

                self.extra_info = self.get_selected_action_info();
            }
        }
    }

    fn get_selected_action_info(&self) -> String {
        let Some(mut action_index) = self.table_states.action.state.selected() else {
            return String::new();
        };

        if action_index < self.game_state.global_actions.len() {
            return self.game_state.global_actions[action_index].to_string();
        }
        action_index -= self.game_state.global_actions.len();

        let item;
        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.items.get(self.selected_resource()) else {
                    return String::new();
                };

                item = resource;
            }
            Item::Building => {
                let Some(building) = self.game_state.items.get(self.selected_building()) else {
                    return String::new();
                };

                item = building;
            }
        }

        return item.manual_actions[action_index].to_string();
    }

    pub fn change_tab(&mut self, new_tab: Tab) {
        self.selected_tab = new_tab;
    }

    pub fn alternate_selection_mode(&mut self) {
        match self.selection_mode {
            SelectionMode::Tabs => {
                self.selection_mode = SelectionMode::Items;

                if self.selected_tab == Tab::Buildings {
                    self.selected_item = Item::Building;
                    self.extra_info = String::new();
                }
                if self.selected_tab == Tab::Resources {
                    self.selected_item = Item::Resource;
                    self.extra_info = String::new();
                }

                self.update_info_table();
                self.update_actions_list();
            }
            SelectionMode::Items => {
                self.selection_mode = SelectionMode::Tabs;
            }
        }
    }

    pub fn call_selected_action(&mut self) {
        let action_index = self.table_states.action.state.selected();
        let Some(mut action_index) = action_index else {
            return;
        };

        if action_index < self.game_state.global_actions.len() {
            let action = self.game_state.global_actions[action_index].clone();

            let info = use_global_action(&mut self.game_state, &action);
            self.extra_info = info;
            self.update_info_table();
            return;
        }
        action_index -= self.game_state.global_actions.len();

        let item;

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.items.get(self.selected_resource()) else {
                    return;
                };

                item = resource;
            }
            Item::Building => {
                let Some(building) = self.game_state.items.get(self.selected_building()) else {
                    return;
                };

                item = building;
            }
        }

        let action = item.manual_actions[action_index].clone();
        let item_name = item.name.clone();

        let info = use_manual_action(item_name, &mut self.game_state, &action, 1);
        self.extra_info = info;
        self.update_info_table();
    }
}
