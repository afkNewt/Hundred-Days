use crate::{app::{App, SelectionMode, Tab}, ui::draw};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
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
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Left => {
                    if app.selection_mode == SelectionMode::Tabs {
                        match app.selected_tab {
                            Tab::Actions => app.change_selected_tab(Tab::Buildings),
                            Tab::Buildings | Tab::Industry => app.change_selected_tab(Tab::Resources),
                            _ => {}
                        }
                    }
                }
                KeyCode::Right => {
                    if app.selection_mode == SelectionMode::Tabs {
                        match app.selected_tab {
                            Tab::Buildings | Tab::Industry => app.change_selected_tab(Tab::Actions),
                            Tab::Resources => app.change_selected_tab(Tab::Buildings),
                            _ => {}
                        }
                    }
                }
                KeyCode::Up => {
                    if app.selection_mode == SelectionMode::Tabs {
                        match app.selected_tab {
                            Tab::Buildings | Tab::Resources | Tab::Actions => app.change_selected_tab(Tab::Industry),
                            _ => {}
                        }
                    } else {
                        app.navigate(false);
                    }
                }
                KeyCode::Down => {
                    if app.selection_mode == SelectionMode::Tabs {
                        match app.selected_tab {
                            Tab::Industry | Tab::Actions | Tab::Resources => app.change_selected_tab(Tab::Buildings),
                            _ => {}
                        }
                    } else {
                        app.navigate(true);
                    }
                }
                KeyCode::Char(' ') => {
                    app.swap_selection_mode();
                }
                KeyCode::Enter => {
                    if app.selected_tab != Tab::Actions {
                        app.change_selected_tab(Tab::Actions);

                        if app.selection_mode == SelectionMode::Tabs {
                            app.swap_selection_mode();
                        }
                    } else {
                        app.call_current_action();
                    }
                }
                _ => {}
            }
        }
    }
}