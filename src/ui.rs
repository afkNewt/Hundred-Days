use crate::app::{App, SelectionMode, Tab};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

const DEFAULT_STYLE: Style = Style {
    fg: None,
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

const HIGHLIGHT_STYLE: Style = Style {
    fg: Some(Color::Rgb(255, 105, 180)),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::UNDERLINED,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Surrounding Block
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title(format!(" In {} days", app.game_state.days))
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

    // Second Column
    let second_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Percentage(100)].as_ref())
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

fn draw_cash<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let cash_block = Paragraph::new(format!("$ {}", app.game_state.currency)).block(block);

    f.render_widget(cash_block, area);
}

fn draw_info<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .style(DEFAULT_STYLE)
        .borders(Borders::ALL)
        .title(" Information ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let text = format!("{}\n\n{}", app.info, app.extra_info);

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_resources<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_tab == Tab::Resources && app.selection_mode == SelectionMode::Tabs {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Resources ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let resources: Vec<ListItem> = app
        .table_states
        .resource
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.clone()))]))
        .collect();

    let resources = List::new(resources)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Items && app.selected_tab == Tab::Resources {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(resources, area, &mut app.table_states.resource.state);
}

fn draw_industries<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_tab == Tab::Industry && app.selection_mode == SelectionMode::Tabs {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Industries ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let industries: Vec<ListItem> = app
        .table_states
        .industry
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.clone()))]))
        .collect();

    let industries = List::new(industries)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Items && app.selected_tab == Tab::Industry {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(industries, area, &mut app.table_states.industry.state);
}

fn draw_buildings<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_tab == Tab::Buildings && app.selection_mode == SelectionMode::Tabs {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Industries ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let buildings: Vec<ListItem> = app
        .table_states
        .building
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.clone()))]))
        .collect();

    let buildings = List::new(buildings)
        .block(block)
        .highlight_style(
            if app.selection_mode == SelectionMode::Items && app.selected_tab == Tab::Buildings {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(buildings, area, &mut app.table_states.building.state);
}

fn draw_actions<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            if app.selected_tab == Tab::Actions && app.selection_mode == SelectionMode::Tabs {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .title(" Actions ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let actions: Vec<ListItem> = app
        .table_states
        .action
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.clone()))]))
        .collect();

    let actions = List::new(actions)
        .block(block)
        .style(DEFAULT_STYLE)
        .highlight_style(
            if app.selection_mode == SelectionMode::Items && app.selected_tab == Tab::Actions {
                HIGHLIGHT_STYLE
            } else {
                DEFAULT_STYLE
            },
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(actions, area, &mut app.table_states.action.state);
}
