
mod event;
mod tabs;

use crate::event::{Event, Events};
use tabs::TabsState;

use std::io;
use termion::{
    event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Paragraph},
    Terminal
};

struct App {
    tabs: TabsState
}

fn main() {
    
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let events = Events::new();

    let mut app = App {
        tabs: TabsState::from_strs(vec!["1"])
    };

    let mut running = true;

    while running {

        terminal.draw(|f| {

            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1)
                ].as_ref())
                .split(size);
            
            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    Spans::from(Span::styled(t.clone(), Style::default().fg(Color::Yellow)))
                })
                .collect();
            
            let tabs = Tabs::new(titles)
                .select(app.tabs.index)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Green)
                );
            
            f.render_widget(tabs, chunks[0]);

            let page_title = String::from("Page ") + (app.tabs.index + 1).to_string().as_str();
            let page = Block::default()
                .title(page_title)
                .borders(Borders::ALL);

            f.render_widget(page, chunks[1]);

            let status_line = Paragraph::new(Spans::from(vec![
                Span::styled("gemini://", Style::default().fg(Color::DarkGray)),
                Span::styled("midnight.pub", Style::default().fg(Color::White)),
                Span::styled("/really/fookin/long/so_that/we-need-to.php#no/fookin/cloo/why/it/is/such/a/long/line.gmi", Style::default().fg(Color::DarkGray))
            ]));
            f.render_widget(status_line, chunks[2]);

        }).unwrap();

        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Char('q') => {
                    running = false;
                },
                Key::Ctrl('t') => {
                    let tab_number = (app.tabs.titles.len() + 1).to_string();
                    app.tabs.titles.push(tab_number);
                }
                Key::Ctrl('r') => {
                    app.tabs.titles.pop();
                    if app.tabs.index == 0 {
                        app.tabs.index = app.tabs.titles.len() - 1;
                    }
                    else {
                        app.tabs.index -= 1;
                    }
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            }
        }

        if app.tabs.titles.len() == 0 {
            running = false;
        }

    }

}
