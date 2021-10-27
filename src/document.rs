
use std::collections::HashMap;
use tui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, Wrap}
};



pub enum LineType {
    Text,
    Link,
    Heading,
    UnorderedListItem,
    Blockquote,
    PreformattedText,
    Custom(String)
}



pub struct DocumentLine {
    line_type: LineType,
    pub attributes: HashMap<String, String>,
    pub text: String
}

impl DocumentLine {

    pub fn new(line_type: LineType) -> DocumentLine {
        DocumentLine {
            line_type,
            attributes: HashMap::new(),
            text: String::new()
        }
    }

    pub fn generate_spans(&self, highlight: bool) -> Spans {
        match &self.line_type {
            LineType::Text => {
                let mut style = Style::default();
                if highlight {
                    style = style.add_modifier(Modifier::REVERSED);
                }
                Spans::from(Span::styled(self.text.clone(), style))
            },
            LineType::Link => {
                let mut style = Style::default().fg(Color::LightBlue);
                if highlight {
                    style = style.add_modifier(Modifier::REVERSED);
                }
                Spans::from(Span::styled(self.text.clone(), style))
            },
            LineType::Heading => {

                let heading_level = self.attributes.get("heading-level");
                let heading_level = match heading_level {
                    Some(heading_level) => heading_level,
                    None => panic!("No heading level specified.")
                };

                let mut style = match heading_level.as_str() {
                    "1" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    "2" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    "3" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    _ => Style::default().add_modifier(Modifier::BOLD)
                };

                if highlight {
                    style = style.add_modifier(Modifier::REVERSED);
                }

                Spans::from(Span::styled(self.text.clone(), style))

            },
            LineType::UnorderedListItem => {
                let bullet_style = Style::default();
                let mut text_style = Style::default();
                if highlight {
                    text_style = text_style.add_modifier(Modifier::REVERSED);
                }
                Spans::from(vec![
                    Span::styled("  * ", bullet_style),
                    Span::styled(self.text.clone(), text_style)
                ])
            },
            _ => unimplemented!()
        }
    }

}



pub struct Page {
    pub lines: Vec<DocumentLine>,
    scroll: u16
}

impl Page {

    pub fn new() -> Page {
        Page {
            lines: Vec::new(),
            scroll: 0
        }
    }

    pub fn render<B: Backend>(&self, area: Rect, f: &mut Frame<B>) {

        let mut text = Text::default();

        for l in self.lines.iter() {
            text.extend(Text::from(l.generate_spans(false)));
        }

        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll, 0));
        
        f.render_widget(paragraph, area);

    }

    pub fn change_scroll(&mut self, amount: i32) {
        if (self.scroll as i32) >= amount * -1 {
            if amount >= 0 {
                self.scroll += amount as u16;
            }
            else {
                self.scroll -= (amount * -1) as u16;
            }
        }
    }

    pub fn set_scroll(&mut self, scroll: u16) {
        self.scroll = scroll;
    }

    pub fn get_scroll(&self) -> u16 {
        self.scroll
    }

}