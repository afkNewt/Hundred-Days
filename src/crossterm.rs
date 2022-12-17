use crate::{
    app::{App, SelectionMode, Table},
    hundred_days::item::ItemType,
    ui::draw,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if app.game_state.current_day < 0 {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Left => {
                    if app.selection_mode == SelectionMode::Table {
                        match app.selected_table {
                            Table::Actions => app.change_tab(Table::Buildings),
                            Table::Buildings | Table::Industry => app.change_tab(Table::Resources),
                            _ => {}
                        }
                    }
                }
                KeyCode::Right => {
                    if app.selection_mode == SelectionMode::Table {
                        match app.selected_table {
                            Table::Buildings | Table::Industry => app.change_tab(Table::Actions),
                            Table::Resources => app.change_tab(Table::Buildings),
                            _ => {}
                        }
                    }
                }
                KeyCode::Up => {
                    if app.selection_mode == SelectionMode::Table {
                        match app.selected_table {
                            Table::Buildings | Table::Resources | Table::Actions => {
                                app.change_tab(Table::Industry)
                            }
                            _ => {}
                        }
                    } else {
                        app.navigate(false);
                    }
                }
                KeyCode::Down => {
                    if app.selection_mode == SelectionMode::Table {
                        match app.selected_table {
                            Table::Industry | Table::Actions | Table::Resources => {
                                app.change_tab(Table::Buildings)
                            }
                            _ => {}
                        }
                    } else {
                        app.navigate(true);
                    }
                }
                KeyCode::Backspace => {
                    if app.selection_mode == SelectionMode::Table {
                        app.alternate_selection_mode();
                    }

                    if let Some(item) = app.game_state.items.get(&app.selected_item) {
                        match item.r#type {
                            ItemType::Resource => app.change_tab(Table::Resources),
                            ItemType::Building => app.change_tab(Table::Buildings),
                        }
                    }
                }
                KeyCode::Char(' ') => {
                    app.alternate_selection_mode();
                }
                KeyCode::Tab => match app.activation_amount {
                    1 => app.activation_amount = 10,
                    10 => app.activation_amount = 100,
                    100 => app.activation_amount = 1,
                    _ => app.activation_amount = 1,
                },
                KeyCode::BackTab => match app.activation_amount {
                    1 => app.activation_amount = 100,
                    10 => app.activation_amount = 1,
                    100 => app.activation_amount = 10,
                    _ => app.activation_amount = 100,
                },
                KeyCode::Enter => {
                    if app.selected_table != Table::Actions {
                        app.change_tab(Table::Actions);

                        if app.selection_mode == SelectionMode::Table {
                            app.alternate_selection_mode();
                        }
                    } else {
                        app.call_selected_action();
                    }
                }
                _ => {}
            }
        }
    }
}
