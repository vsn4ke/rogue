use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
};

use crate::{
    app::App,
    colors::{YELLOW2, c},
};

use super::Drawable;

pub struct TopBar {
    pub tab_list: Vec<String>,
    pub selected_tab: usize,
}

impl Drawable for TopBar {
    fn draw(&self, frame: &mut ratatui::Frame, rect: ratatui::prelude::Rect, _app: &App) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(rect);
        let mut spans = Vec::new();
        for (i, tab_name) in self.tab_list.iter().enumerate() {
            let i = i + 1;
            let span = Span::default()
                .content(format!("[F{i}] {tab_name}     "))
                .style(if i == self.selected_tab {
                    Style::default().fg(c(YELLOW2))
                } else {
                    Style::default()
                });

            spans.push(span);
        }

        frame.render_widget(Text::from(Line::from(spans)), layout[1]);
    }
}

pub struct BottomBar;
impl Drawable for BottomBar {
    fn draw(&self, frame: &mut ratatui::Frame, rect: ratatui::prelude::Rect, _app: &App) {
        let spans = vec![Span::default().content(" [←↑↓→] Move ")];
        //todo add other shortcut
        frame.render_widget(Text::from(Line::from(spans)), rect);
    }
}
