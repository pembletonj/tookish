
mod event;
mod tabs;
mod document;

use event::{
    Event,
    Events
};
use tabs::TabsState;
use document::{
    DocumentLine,
    LineType,
    Page
};

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
    tabs: TabsState,
    pages: Vec<Page>
}

fn main() {
    
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let events = Events::new();

    let mut app = App {
        tabs: TabsState::from_strs(vec!["1"]),
        pages: vec![Page::new()]
    };

    app.pages[0].lines.push(DocumentLine::new(LineType::Heading));
    app.pages[0].lines[0].attributes.insert(String::from("heading-level"), String::from("1"));
    app.pages[0].lines[0].text = String::from("The First Page (H1)");

    app.pages[0].lines.push(DocumentLine::new(LineType::Heading));
    app.pages[0].lines[1].attributes.insert(String::from("heading-level"), String::from("2"));
    app.pages[0].lines[1].text = String::from("The First Page (H2)");

    app.pages[0].lines.push(DocumentLine::new(LineType::Heading));
    app.pages[0].lines[2].attributes.insert(String::from("heading-level"), String::from("3"));
    app.pages[0].lines[2].text = String::from("The First Page (H3)");

    app.pages[0].lines.push(DocumentLine::new(LineType::Text));
    app.pages[0].lines[3].text = String::from("This is an extremely long line. It's hard to describe just how absurdly long this exceptionally long, long, LONG line is.");

    app.pages[0].lines.push(DocumentLine::new(LineType::UnorderedListItem));
    app.pages[0].lines[4].text = String::from("List item 1");

    app.pages[0].lines.push(DocumentLine::new(LineType::UnorderedListItem));
    app.pages[0].lines[5].text = String::from("This is list item number two. It is somehow a competitor for the title of longest line, but whether or not it will get it, I do not know.");

    app.pages[0].lines.push(DocumentLine::new(LineType::UnorderedListItem));
    app.pages[0].lines[6].text = String::from("It did!");

    app.pages[0].lines.push(DocumentLine::new(LineType::Link));
    app.pages[0].lines[7].text = String::from("This link is currently useless.");

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

            /*let page_title = String::from("Page ") + (app.tabs.index + 1).to_string().as_str();
            let page = Block::default()
                .title(page_title)
                .borders(Borders::ALL);

            f.render_widget(page, chunks[1]);*/

            app.pages[app.tabs.index].render(chunks[1], f);

            let status_line = Paragraph::new(Spans::from(vec![
                Span::styled("gemini://", Style::default().fg(Color::DarkGray)),
                Span::styled("midnight.pub", Style::default().fg(Color::White)),
                Span::styled("/", Style::default().fg(Color::DarkGray))
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
                    app.tabs.titles.push(tab_number.clone());

                    let mut page = Page::new();

                    page.lines.push(DocumentLine::new(LineType::Text));
                    page.lines[0].text = String::from("Hello from page ") + tab_number.as_str();

                    app.pages.push(page);
                }
                Key::Ctrl('r') => {
                    app.tabs.titles.pop();
                    app.pages.pop();
                    if app.tabs.index == 0 {
                        app.tabs.index = app.tabs.titles.len() - 1;
                    }
                    else {
                        app.tabs.index -= 1;
                    }
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Up => app.pages[app.tabs.index].change_scroll(-1),
                Key::Down => app.pages[app.tabs.index].change_scroll(1),
                _ => {}
            }
        }

        if app.tabs.titles.len() == 0 {
            running = false;
        }

    }

}
