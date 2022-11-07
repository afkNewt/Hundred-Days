use tui::widgets::ListState;

use crate::hundred_days::{building_action, global_action, resource_action, Game, Industry};

#[derive(Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl StatefulList {
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
    pub industry: Option<Industry>,
    // selected tab
    pub selected_tab: Tab,
    pub selection_mode: SelectionMode,

    pub game_state: Game,
}

impl App {
    pub fn new() -> App {
        let mut default_state = ListState::default();
        default_state.select(Some(0));

        let state = StatefulList {
            state: default_state,
            items: Vec::new(),
        };

        let mut app = App {
            table_states: States {
                resource: state.clone(),
                building: state.clone(),
                industry: state.clone(),
                action: state.clone(),
            },
            info: String::new(),
            extra_info: String::new(),
            industry: None,
            selected_tab: Tab::Resources,
            selection_mode: SelectionMode::Items,
            game_state: Game::from_toml(),
        };
        app.generate_lists();
        app.update_info();
        return app;
    }

    pub fn update_info(&mut self) {
        match self.selected_tab {
            Tab::Resources => {
                self.info = self.game_state.resources[self.selected_resource()].information();
            }
            Tab::Buildings => {
                self.info = self.game_state.buildings[self.selected_building()].information();
            }
            _ => {}
        }
    }

    pub fn swap_selection_mode(&mut self) {
        match self.selection_mode {
            SelectionMode::Tabs => {
                self.selection_mode = SelectionMode::Items;

                if self.selected_tab == Tab::Resources {
                    self.generate_lists();
                }
                if self.selected_tab == Tab::Buildings {
                    self.generate_lists();
                }

                self.update_info();
            },
            SelectionMode::Items => {
                self.selection_mode = SelectionMode::Tabs;
            },
        }
    }

    pub fn change_selected_tab(&mut self, new_box: Tab) {
        self.selected_tab = new_box;
    }

    fn reset_table_states(&mut self) {
        let mut state = ListState::default();
        state.select(Some(0));

        self.table_states.resource.state = state.clone();
        self.table_states.building.state = state.clone();
        self.table_states.action.state = state.clone();
    }

    pub fn selected_building(&self) -> usize {
        let index = self.table_states.building.state.selected();
        let Some(selected) = index else {
            return 0;
        };

        let Some(industry) = self.industry else {
            return selected;
        };

        let mut counter = 0;
        for (index, building) in self.game_state.buildings.iter().enumerate() {
            if building.industry == industry {
                if counter == selected {
                    return index;
                }
                counter += 1;
            }
        }

        return 0;
    }

    pub fn selected_resource(&self) -> usize {
        let index = self.table_states.resource.state.selected();
        let Some(selected) = index else {
            return 0;
        };

        let Some(industry) = self.industry else {
            return selected;
        };

        let mut counter = 0;
        for (index, resource) in self.game_state.resources.iter().enumerate() {
            if resource.industry == industry {
                if counter == selected {
                    return index;
                }
                counter += 1;
            }
        }

        return 0;
    }

    pub fn selected_action(&self) -> String {
        let index = self.table_states.action.state.selected();
        let Some(index) = index else {
            return String::new();
        };

        let action = self.table_states.action.items[index].clone();
        return action;
    }

    pub fn navigate(&mut self, move_down: bool) {
        match self.selected_tab {
            Tab::Resources => {
                if move_down {
                    self.table_states.resource.next();
                } else {
                    self.table_states.resource.previous();
                }
            }
            Tab::Buildings => {
                if move_down {
                    self.table_states.building.next();
                } else {
                    self.table_states.building.previous();
                }
            }
            Tab::Industry => {
                if move_down {
                    self.table_states.industry.next();
                } else {
                    self.table_states.industry.previous();
                }

                if let Some(selected) = self.table_states.industry.state.selected() {
                    match selected {
                        0 => self.industry = Some(Industry::Mining),
                        1 => self.industry = Some(Industry::Logging),
                        2 => self.industry = Some(Industry::Farming),
                        _ => self.industry = None,
                    }
                }
                self.generate_lists();
                self.reset_table_states();
            }
            Tab::Actions => {
                if move_down {
                    self.table_states.action.next();
                } else {
                    self.table_states.action.previous();
                }
            }
        }
        self.update_info();
    }

    pub fn generate_lists(&mut self) {
        self.table_states.resource.items = Vec::new();
        self.table_states.building.items = Vec::new();

        if let Some(industry) = self.industry {
            for resource in &self.game_state.resources {
                if resource.industry == industry {
                    self.table_states
                        .resource
                        .items
                        .push(format!("{}", resource.name));
                }
            }

            for building in &self.game_state.buildings {
                if building.industry == industry {
                    self.table_states
                        .building
                        .items
                        .push(format!("{}", building.name));
                }
            }
        } else {
            for resource in &self.game_state.resources {
                self.table_states
                    .resource
                    .items
                    .push(format!("{}", resource.name));
            }

            for building in &self.game_state.buildings {
                self.table_states
                    .building
                    .items
                    .push(format!("{}", building.name));
            }
        }

        self.table_states.industry.items = vec![
            "Mining".to_string(),
            "Logging".to_string(),
            "Farming".to_string(),
            "None".to_string(),
        ];

        self.table_states.action.items = Vec::new();
        for action in &self.game_state.actions {
            self.table_states.action.items.push(format!("{}", action));
        }

        match self.selected_tab {
            Tab::Resources => {
                for action in &self.game_state.resources[self.selected_resource()].actions {
                    self.table_states.action.items.push(format!("{}", action));
                }
            }
            Tab::Buildings => {
                for action in &self.game_state.buildings[self.selected_building()].actions {
                    self.table_states.action.items.push(format!("{}", action));
                }
            }
            _ => {}
        }
    }

    pub fn call_current_action(&mut self) {
        let global_actions = self.game_state.actions.clone();
        let building_actions = self.game_state.buildings[self.selected_building()]
            .actions
            .clone();
        let resource_actions = self.game_state.resources[self.selected_resource()]
            .actions
            .clone();

        if global_actions.contains(&self.selected_action()) {
            let action = global_action(&self.selected_action());
            let info = (action)(&mut self.game_state);

            self.info = String::new();
            self.extra_info = info;
        }
        if building_actions.contains(&self.selected_action()) {
            let action = building_action(&self.selected_action());
            let selected_building = self.selected_building();
            let info = (action)(&mut self.game_state, selected_building);

            self.info = self.game_state.buildings[selected_building].information();
            self.extra_info = info;
        }
        if resource_actions.contains(&self.selected_action()) {
            let action = resource_action(&self.selected_action());
            let selected_resource = self.selected_resource();
            let info = (action)(&mut self.game_state, selected_resource);

            self.info = self.game_state.resources[selected_resource].information();
            self.extra_info = info;
        }

        self.update_info();
    }
}
