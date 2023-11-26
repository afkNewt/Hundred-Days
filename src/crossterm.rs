use crate::{
    app::{App, Table},
    hundred_days::item::ItemCategory,
    ui::draw,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

enum Inputs {
    Exit,
    Left,
    Right,
    Up,
    Down,
    Back,
    IncreaseActionActivation,
    DecreaseActionActivation,
    ActivateOrGoToActions,
    PassDay,
}

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

        let mut action = None;

        if let Event::Key(key) = event::read()? {
            action = match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(Inputs::Exit),
                KeyCode::Left | KeyCode::Char('a') => Some(Inputs::Left),
                KeyCode::Right | KeyCode::Char('d') => Some(Inputs::Right),
                KeyCode::Up | KeyCode::Char('w') => Some(Inputs::Up),
                KeyCode::Down | KeyCode::Char('s') => Some(Inputs::Down),
                KeyCode::Backspace => Some(Inputs::Back),
                KeyCode::Tab => Some(Inputs::IncreaseActionActivation),
                KeyCode::BackTab => Some(Inputs::DecreaseActionActivation),
                KeyCode::Enter => Some(Inputs::ActivateOrGoToActions),
                KeyCode::Char('c') => Some(Inputs::PassDay),
                _ => None,
            };

            if app.game_state.day < 0 {
                action = None;
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    action = Some(Inputs::Exit);
                }
            }
        }

        let Some(action) = action else {
            continue;
        };

        match action {
            Inputs::Exit => return Ok(()),
            Inputs::Left => app.navigate(crate::app::Direction::Left),
            Inputs::Right => app.navigate(crate::app::Direction::Right),
            Inputs::Up => app.navigate(crate::app::Direction::Up),
            Inputs::Down => app.navigate(crate::app::Direction::Down),
            Inputs::Back => {
                if let Some(item) = app.game_state.items.get(&app.selected_item) {
                    match item.category {
                        ItemCategory::Resource => app.change_tab(Table::Resources),
                        ItemCategory::Building => app.change_tab(Table::Buildings),
                    }
                }
            }
            Inputs::IncreaseActionActivation => match app.activation_amount {
                1 => app.activation_amount = 10,
                10 => app.activation_amount = 100,
                100 => app.activation_amount = 1,
                _ => app.activation_amount = 1,
            },
            Inputs::DecreaseActionActivation => match app.activation_amount {
                1 => app.activation_amount = 100,
                10 => app.activation_amount = 1,
                100 => app.activation_amount = 10,
                _ => app.activation_amount = 100,
            },
            Inputs::ActivateOrGoToActions => {
                if app.selected_table != Table::Actions {
                    app.change_tab(Table::Actions);
                } else {
                    app.call_selected_action();
                }
            }
            Inputs::PassDay => {
                app.game_state.pass_day(app.activation_amount);
            }
        }
    }
}
