use ratatui::widgets::ListState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::hundred_days::item::ItemCategory;
use crate::{
    app::{App, SelectionMode, Table},
    hundred_days::action::Action,
};

const DEFAULT_STYLE: Style = Style {
    fg: None,
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
    underline_color: None,
};

const HIGHLIGHT_STYLE: Style = Style {
    fg: Some(Color::Rgb(255, 105, 180)),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::UNDERLINED,
    underline_color: None,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    if app.game_state.day < 0 {
        draw_end_screen(f, app);
    } else {
        draw_game_screen(f, app);
    }
}

pub fn draw_end_screen(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Surrounding Block
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title(format!(" In {} days ", app.game_state.day))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);
    f.render_widget(block, size);

    draw_game_ended_stats(f, app, size);
}

pub fn draw_game_screen(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Surrounding Block
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title(format!(" In {} days ", app.game_state.day))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);
    f.render_widget(block, size);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let rows = Layout::default()
        .direction(Direction::Vertical)
        // .margin(1)
        .constraints([
            Constraint::Min(3),
            Constraint::default(),
            Constraint::Min(5),
        ])
        .split(columns[0]);

    let cash_char_count = (app.game_state.currency.checked_ilog10().unwrap_or(0) + 6) as u16;
    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::default(), Constraint::Min(cash_char_count)])
        .split(rows[0]);

    let middle_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    draw_tabs(f, app, top_row[0]);
    draw_cash(f, app, top_row[1]);

    draw_resources(f, app, middle_row[0]);
    draw_buildings(f, app, middle_row[1]);

    draw_history(f, app, rows[2]);

    draw_actions(f, app, columns[1]);
}

fn draw_game_ended_stats(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let text = vec![
        Line::from(Span::raw("Congratulations!")),
        Line::from(Span::raw(format!(
            "You earned {} points!",
            app.game_state.net_worth(),
        ))),
        Line::from(Span::raw("Press q to exit")),
    ];

    let stats_block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(stats_block, area);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Main Game", "Night Market"];

    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let tabs = Tabs::new(titles)
        .block(block)
        .highlight_style(HIGHLIGHT_STYLE)
        .select(0);

    f.render_widget(tabs, area);
}

fn draw_cash(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let cash_block = Paragraph::new(format!("$ {}", app.game_state.currency)).block(block);

    f.render_widget(cash_block, area);
}

fn draw_actions(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .border_style(
            if app.selected_table == Table::Actions && app.selection_mode == SelectionMode::Table {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .borders(Borders::ALL)
        .title(" Actions ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let action_block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    f.render_widget(block, area);
    let selected_item_name = app.selected_item.clone();
    let Some(selected_item) = app.game_state.items.get(&selected_item_name) else {
        return;
    };

    let mut constraints = selected_item
        .actions_active
        .iter()
        .map(|a| Constraint::Min(a.description().lines().count() as u16 + 2))
        .collect::<Vec<Constraint>>();

    // dummy constraint active and passive actions
    // are split up
    constraints.push(Constraint::default());

    constraints.append(
        &mut selected_item
            .actions_passive
            .iter()
            .map(|p| Constraint::Min(p.description().lines().count() as u16 + 2))
            .collect::<Vec<Constraint>>(),
    );

    let action_blocks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .margin(1)
        .split(area);

    // use app.selected_item to change
    // the block to one with highlight
    for (i, active) in selected_item.actions_active.iter().enumerate() {
        let desc = active.description();
        let block = action_block.to_owned().border_style(
            if app.selected_table == Table::Actions
                && app.selection_mode == SelectionMode::Item
                && app.selection_index == i
            {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        );
        let action = Paragraph::new(desc.as_str())
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        f.render_widget(action, action_blocks[i]);
    }

    let max_len = action_blocks.len() - 1;
    for (i, passive) in selected_item.actions_passive.iter().enumerate() {
        let desc = passive.description();
        let action = Paragraph::new(desc.as_str())
            .block(action_block.clone().title(passive.name()))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        f.render_widget(action, action_blocks[max_len - i]);
    }
}

fn draw_resources(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_table == Table::Resources && app.selection_mode == SelectionMode::Table
            {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Resources ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let resources: Vec<ListItem> = app
        .resource_table
        .items
        .iter()
        .map(|res_name| {
            let res_amount = app.game_state.items.get(res_name).unwrap().amount;
            let char_count = res_name.chars().count();
            let lines = vec![Line::from(format!(
                "{res_name}{:>1$.2}",
                res_amount,
                (area.width as usize)
                    .checked_sub(char_count)
                    .unwrap_or(0)
                    .checked_sub(5)
                    .unwrap_or(0)
            ))];
            ListItem::new(lines)
        })
        .collect();

    let resources = List::new(resources)
        .block(block)
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol("> ");

    let selected_item_name = app.selected_item.clone();
    if let Some(item) = app.game_state.items.get(&selected_item_name) {
        if item.category == ItemCategory::Resource {
            let index = &app
                .resource_table
                .items
                .iter()
                .position(|s| *s == item.name);

            if let Some(index) = index {
                f.render_stateful_widget(
                    resources,
                    area,
                    &mut ListState::default().with_selected(Some(*index)),
                );
                return;
            };
        }
    };

    f.render_widget(resources, area);
}

fn draw_buildings(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_table == Table::Buildings && app.selection_mode == SelectionMode::Table
            {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Buildings ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let buildings: Vec<ListItem> = app
        .building_table
        .items
        .iter()
        .map(|build_name| {
            let build_amount = app.game_state.items.get(build_name).unwrap().amount;
            let char_count = build_name.chars().count();
            let lines = vec![Line::from(format!(
                "{build_name}{:>1$.2}",
                build_amount,
                (area.width as usize)
                    .checked_sub(char_count)
                    .unwrap_or(0)
                    .checked_sub(5)
                    .unwrap_or(0)
            ))];
            ListItem::new(lines)
        })
        .collect();

    let buildings = List::new(buildings)
        .block(block)
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol("> ");

    let selected_item_name = app.selected_item.clone();
    if let Some(item) = app.game_state.items.get(&selected_item_name) {
        if item.category == ItemCategory::Building {
            let index = &app
                .building_table
                .items
                .iter()
                .position(|s| *s == item.name);

            if let Some(index) = index {
                f.render_stateful_widget(
                    buildings,
                    area,
                    &mut ListState::default().with_selected(Some(*index)),
                );
                return;
            };
        }
    };

    f.render_widget(buildings, area);
}

fn draw_history(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(DEFAULT_STYLE)
        .title(" History ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let history_items: Vec<ListItem> = vec![
        ListItem::new("Test content meant to test things that are kinda long just to make sure there is no wrapping and that it doesn't look too weird"),
        ListItem::new("An additional list item of a more standard length"),
        ListItem::new("Now we want to make sure that too many list items isnt weird"),
        ListItem::new("This should be a good number of items")
    ];

    let history = List::new(history_items).block(block);
    f.render_widget(history, area)
}
