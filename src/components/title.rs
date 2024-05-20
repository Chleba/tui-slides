use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, layout::get_title_layout};

#[derive(Default)]
pub struct Title {}

impl Title {
    pub fn new() -> Self {
        Self {}
    }

    fn make_title(&self) -> Paragraph {
        let version: &str = env!("CARGO_PKG_VERSION");
        let title = format!("tui-slides (v{})", version);

        Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
            )
            .alignment(Alignment::Left)
    }
}

impl Component for Title {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let rect = get_title_layout(area);
        let title = self.make_title();

        f.render_widget(title, rect);

        Ok(())
    }
}
