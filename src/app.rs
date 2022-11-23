use tui::widgets::ListState;

use crate::hundred_days::{
    building_action, global_action, resource_action, Action, Game,
};

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

        let game = Game::from_toml();

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
            industry: game.industries[0].clone(),
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

        for (resource_name, resource) in &self.game_state.resources {
            if resource.industries.contains(&self.industry) {
                self.table_states
                    .resource
                    .items
                    .push(resource_name.to_string());
            }
        }

        self.table_states.resource.reset_state();
    }

    fn update_building_list(&mut self) {
        self.table_states.building.items = Vec::new();

        for (building_name, building) in &self.game_state.buildings {
            if building.industries.contains(&self.industry) {
                self.table_states
                    .building
                    .items
                    .push(building_name.to_string());
            }
        }

        self.table_states.building.reset_state();
    }

    fn update_actions_list(&mut self) {
        self.table_states.action.items = Vec::new();

        for action in &self.game_state.global_actions {
            self.table_states
                .action
                .items
                .push(action.name().to_string());
        }

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.resources.get(self.selected_resource()) else {
                    return;
                };
    
                for action in &resource.actions {
                    self.table_states
                        .action
                        .items
                        .push(action.name().to_string());
                }
            },
            Item::Building => {
                let Some(building) = self.game_state.buildings.get(self.selected_building()) else {
                    return;
                };
    
                for action in &building.actions {
                    self.table_states
                        .action
                        .items
                        .push(action.name().to_string());
                }
            },
        }

        self.table_states.action.reset_state();
    }

    fn update_info_table(&mut self) {
        self.info = String::new();

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.resources.get(self.selected_resource()) else {
                    return;
                };
    
                self.info = resource.information();
            },
            Item::Building => {
                let Some(building) = self.game_state.buildings.get(self.selected_building()) else {
                    return;
                };
    
                self.info = building.information();
            },
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
            },
            Tab::Buildings => {
                if up {
                    self.table_states.building.next();
                } else {
                    self.table_states.building.previous();
                }
                
                self.update_actions_list();
                self.update_info_table();
            },
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
            },
            Tab::Actions => {
                if up {
                    self.table_states.action.next();
                } else {
                    self.table_states.action.previous();
                }
            },
        }
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
                }
                if self.selected_tab == Tab::Resources {
                    self.selected_item = Item::Resource;
                }

                self.update_info_table();
                self.update_actions_list();
            },
            SelectionMode::Items => {
                self.selection_mode = SelectionMode::Tabs;
            },
        }
    }

    pub fn call_selected_action(&mut self) {
        let action_index = self.table_states.action.state.selected();
        let Some(mut action_index) = action_index else {
            return;
        };

        if action_index < self.game_state.global_actions.len() {
            let action = self.game_state.global_actions[action_index].clone();

            let info = global_action(&mut self.game_state, action);
            self.extra_info = info;
            self.update_info_table();
            return;
        }
        action_index -= self.game_state.global_actions.len();

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.resources.get(self.selected_resource()) else {
                    return;
                };
    
                let action = resource.actions[action_index].clone();
                let resource_name = resource.name.clone();
    
                let info = resource_action(&mut self.game_state, &resource_name, action);
                self.extra_info = info;
                self.update_info_table();
            },
            Item::Building => {
                let Some(building) = self.game_state.buildings.get(self.selected_building()) else {
                    return;
                };
    
                let action = building.actions[action_index].clone();
                let building_name = building.name.clone();
    
                let info = building_action(&mut self.game_state, &building_name, action);
                self.extra_info = info;
                self.update_info_table();
            },
        }
    }
}
