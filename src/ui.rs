use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

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

    // Horizontal Chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(f.size());

    // First Column
    let first_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    draw_cash(f, &app, first_column[0]);
    draw_resources(f, app, first_column[1]);
    // First Column

    let industry_len: u16 = app
        .game_state
        .industries
        .len()
        .try_into()
        .unwrap_or_default();
    // Second Column
    let second_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(industry_len + 2),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    draw_industries(f, app, second_column[0]);
    draw_buildings(f, app, second_column[1]);
    // Second Column

    // Third Column
    draw_actions(f, app, chunks[2]);
    // Third Column

    // Fourth Column
    draw_info(f, &app, chunks[3]);
    // Fourth Column
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

fn draw_cash(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let cash_block = Paragraph::new(format!("$ {}", app.game_state.currency)).block(block);

    f.render_widget(cash_block, area);
}

fn draw_info(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title(" Information ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let text = format!("{}\n\n{}", app.info, app.extra_info);
    let mut paragraph = Paragraph::new(text)
        .block(block.clone())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    let mut action_description = String::new();
    if app.selected_table == Table::Actions && app.selection_mode == SelectionMode::Item {
        let action_index = app.action_table.state.selected();
        if let Some(action_index) = action_index {
            if let Some(item) = app.game_state.items.get(&app.selected_item) {
                action_description = item.actions_active[action_index].description()
            }
        };
        // this should only have two parts
        let split: Vec<&str> = app.info.split(&action_description).collect();
        if split.len() == 2 {
            let mut lines = Vec::new();

            for line in split[0].split("\n") {
                lines.push(Line::from(Span::raw(line)));
            }
            lines.pop();
            for line in action_description.split("\n") {
                lines.push(Line::from(Span::styled(line, HIGHLIGHT_STYLE)));
            }
            for line in split[1].split("\n") {
                lines.push(Line::from(Span::raw(line)));
            }
            lines.remove(lines.len() - split[1].split("\n").count());

            for line in app.extra_info.split("\n") {
                lines.push(Line::from(Span::raw(line)));
            }

            paragraph = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Left);
        }
    }

    f.render_widget(paragraph, area);
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
                area.width as usize - char_count - 5
            ))];
            ListItem::new(lines)
        })
        .collect();

    let resources = List::new(resources)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Item && app.selected_table == Table::Resources {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(resources, area, &mut app.resource_table.state);
}

fn draw_industries(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_table == Table::Industry && app.selection_mode == SelectionMode::Table {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Industries ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let industries: Vec<ListItem> = app
        .industry_table
        .items
        .iter()
        .map(|i| ListItem::new(vec![Line::from(Span::raw(i.clone()))]))
        .collect();

    let industries = List::new(industries)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Item && app.selected_table == Table::Industry {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(industries, area, &mut app.industry_table.state);
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
                area.width as usize - char_count - 5
            ))];
            ListItem::new(lines)
        })
        .collect();

    let buildings = List::new(buildings)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Item && app.selected_table == Table::Buildings {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(buildings, area, &mut app.building_table.state);
}

fn draw_actions(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_table == Table::Actions && app.selection_mode == SelectionMode::Table {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(format!(" Actions ({}) ", app.activation_amount))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let actions: Vec<ListItem> = app
        .action_table
        .items
        .iter()
        .map(|i| ListItem::new(vec![Line::from(Span::raw(i.clone()))]))
        .collect();

    let actions = List::new(actions)
        .block(block)
        .style(DEFAULT_STYLE)
        .highlight_style(
            if app.selection_mode == SelectionMode::Item && app.selected_table == Table::Actions {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(actions, area, &mut app.action_table.state);
}
