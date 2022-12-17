use tui::widgets::ListState;

use crate::hundred_days::{
    action::{global::GlobalAction, manual::ManualAction, Information},
    game::Game,
    item::ItemType,
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

    // number of times to call an action
    // when an action is activated
    pub activation_amount: i32,

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
            activation_amount: 1,
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

    pub fn selected_building(&self) -> String {
        return self
            .table_states
            .building
            .items
            .get(
                self.table_states
                    .resource
                    .state
                    .selected()
                    .unwrap_or_default(),
            )
            .unwrap_or(&String::new())
            .to_owned();
    }

    pub fn selected_resource(&self) -> String {
        return self
            .table_states
            .resource
            .items
            .get(
                self.table_states
                    .resource
                    .state
                    .selected()
                    .unwrap_or_default(),
            )
            .unwrap_or(&String::new())
            .to_owned();
    }

    fn selected_action(&self) -> (Option<GlobalAction>, Option<ManualAction>) {
        let action_index = self.table_states.action.state.selected();
        let Some(mut action_index) = action_index else {
            return (None, None);
        };

        // if its a global action
        if action_index < self.game_state.global_actions.len() {
            return (
                Some(self.game_state.global_actions[action_index].clone()),
                None,
            );
        }
        // remove global actions
        action_index -= self.game_state.global_actions.len();

        let item;
        // get the selected item
        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.items.get(&self.selected_resource()) else {
                    return (None, None);
                };

                item = resource;
            }
            Item::Building => {
                let Some(building) = self.game_state.items.get(&self.selected_building()) else {
                    return (None, None);
                };

                item = building;
            }
        }

        return (None, Some(item.manual_actions[action_index].clone()));
    }

    fn update_resources_list(&mut self) {
        self.table_states.resource.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| {
                if item.r#type == ItemType::Resource && item.industries.contains(&self.industry) {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();

        self.table_states.resource.reset_state();
    }

    fn update_building_list(&mut self) {
        self.table_states.building.items = self
            .game_state
            .items
            .iter()
            .filter(|(_item_name, item)| {
                if item.r#type == ItemType::Building && item.industries.contains(&self.industry) {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|(item_name, _)| {
                return item_name.to_string();
            })
            .collect();

        self.table_states.building.reset_state();
    }

    fn update_actions_list(&mut self) {
        self.table_states.action.items = self
            .game_state
            .global_actions
            .iter()
            .map(|action| return action.name().to_string())
            .collect();

        let selected_item;
        match self.selected_item {
            Item::Resource => {
                let Some(item) = self.game_state.items.get(&self.selected_resource()) else {
                    return;
                };

                selected_item = item;
            }
            Item::Building => {
                let Some(item) = self.game_state.items.get(&self.selected_building()) else {
                    return;
                };

                selected_item = item;
            }
        }

        self.table_states.action.items.append(
            &mut selected_item
                .manual_actions
                .iter()
                .map(|action| action.name().to_string())
                .collect::<Vec<String>>(),
        );

        self.table_states.action.reset_state();
    }

    fn update_info_table(&mut self) {
        self.info = String::new();

        match self.selected_item {
            Item::Resource => {
                let Some(resource) = self.game_state.items.get(&self.selected_resource()) else {
                    return;
                };

                self.info = resource.information();
            }
            Item::Building => {
                let Some(building) = self.game_state.items.get(&self.selected_building()) else {
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
            }
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
        let action = self.selected_action();
        match action {
            (None, Some(action)) => {
                let item;
                match self.selected_item {
                    Item::Resource => {
                        item = self.selected_resource();
                    }
                    Item::Building => {
                        item = self.selected_building();
                    }
                }

                let info = action.activate(
                    item.to_string(),
                    &mut self.game_state,
                    self.activation_amount,
                );
                self.extra_info = info;
                self.update_info_table();
            }
            (Some(action), None) => {
                let info = action.activate(&mut self.game_state, self.activation_amount);
                self.extra_info = info;
                self.update_info_table();
            }
            _ => return,
        }
    }
}
