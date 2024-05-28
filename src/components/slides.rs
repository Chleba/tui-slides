use std::io::Read;

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{block::Title, *},
};
use ratatui_image::{protocol::StatefulProtocol, StatefulImage, Image};
use tokio::sync::mpsc::UnboundedSender;
use tui_big_text::{BigText, PixelSize};

use super::{Component, Frame};
use crate::{
    action::Action,
    enums::{ContentJson, ReturnSlideWidget, SlidesJson},
    layout::{get_slides_layout, CONTENT_PERCENT_HEIGHT, CONTENT_PERCENT_WIDTH},
    slide_builder::make_slide_content,
};

#[derive(Default)]
pub struct Slides {
    action_tx: Option<UnboundedSender<Action>>,
    slides: Option<SlidesJson>,
    slide_index: usize,
    slide_count: usize,
    image: Option<Box<dyn StatefulProtocol>>,
    images: Vec<Option<Box<dyn StatefulProtocol>>>,
}

impl Slides {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            slides: None,
            slide_index: 0,
            slide_count: 0,
            image: None,
            images: Vec::new(),
        }
    }

    fn get_json_slides(&mut self) {
        let mut f =
            std::fs::File::open(".data/slides.json5").expect("Failed to open slides json file");
        // let data_dir = crate::utils::get_data_dir();
        // let mut f = std::fs::File::open(data_dir.join("slides.json5")).expect("Failed to open slides json file");
        let mut f_content = String::new();
        f.read_to_string(&mut f_content)
            .expect("Failed to read json slides file");
        let slides: SlidesJson = serde_json::from_str(&f_content).unwrap();

        self.slides = Some(slides);
        if let Some(slides) = &self.slides {
            self.slide_count = slides.slides.len();
        }
    }

    fn get_slide_rect(&self, rect: Rect, item_rect: Option<Rect>) -> Rect {
        let mut slide_rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
        if let Some(slides) = &self.slides {
            if let Some(s_content_rect) = item_rect {
                slide_rect.x += s_content_rect.x;
                slide_rect.y += s_content_rect.y;
                slide_rect.width = s_content_rect.width;
                slide_rect.height = s_content_rect.height;
            }
        }
        slide_rect
    }

    fn next_slide(&mut self) {
        let mut s_index = self.slide_index + 1;
        s_index %= self.slide_count;
        self.slide_index = s_index;
    }

    fn previous_slide(&mut self) {
        let mut s_index = self.slide_index;
        if self.slide_index.checked_sub(1).is_none() {
            s_index = self.slide_count - 1;
        } else {
            s_index -= 1;
        }
        self.slide_index = s_index;
    }

    fn make_title(&self) -> BigText {
        let mut title_text = "__title__".to_string();
        if let Some(slides) = &self.slides {
            if let Some(t) = &slides.slides[self.slide_index].title {
                title_text = t.to_string();
            }
        }

        let big_title = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![title_text.red().into()])
            .alignment(Alignment::Center)
            .build();
        big_title.unwrap()
    }

    fn make_block(&self) -> Block {
        let s_index = self.slide_index + 1;
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 100, 100)))
            .border_type(BorderType::Double)
            .title(
                Title::from(Line::from(vec![
                    "|".yellow(),
                    s_index.to_string().green(),
                    "/".yellow(),
                    self.slide_count.to_string().green(),
                    "|".yellow(),
                ]))
                .alignment(Alignment::Right)
                .position(block::Position::Bottom),
            )
    }

    fn make_slide_items(&self) -> Vec<(ReturnSlideWidget<'_>, Option<Rect>)> {
        if let Some(slides) = &self.slides {
            let slide = slides.slides[self.slide_index].clone();
            let mut slide_items = vec![];
            for item in slide.content {
                slide_items.push((make_slide_content(item.clone()), item.rect));
            }
            return slide_items;
        }

        vec![(
            ReturnSlideWidget::Paragraph(Paragraph::new("__text__")),
            None,
        )]
    }
}

impl Component for Slides {
    fn init(&mut self, area: Rect) -> Result<()> {
        self.get_json_slides();
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Next => {
                self.next_slide();
            }
            Action::Previous => {
                self.previous_slide();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let mut box_width = CONTENT_PERCENT_WIDTH;
        let mut box_height = CONTENT_PERCENT_HEIGHT;
        if let Some(slides) = &self.slides {
            box_width = slides.box_size.percent_width;
            box_height = slides.box_size.percent_height;
        }

        let rect = get_slides_layout(area, box_width, box_height);
        let title_rect = Rect::new(
            rect.content.x,
            rect.content.y + 2,
            rect.content.width,
            rect.content.height,
        );

        let title = self.make_title();
        let block = self.make_block();
        let slide_items = self.make_slide_items();

        f.render_widget(title, title_rect);
        f.render_widget(block, rect.content);

        // -- render slide widgets
        for (slide, r) in slide_items {
            let slide_rect = self.get_slide_rect(rect.content, r);
            match slide {
                ReturnSlideWidget::Paragraph(s) => {
                    f.render_widget(s, slide_rect);
                }
                ReturnSlideWidget::Line(s) => {
                    f.render_widget(s, slide_rect);
                }
                ReturnSlideWidget::BigText(s) => {
                    f.render_widget(s, slide_rect);
                }
                ReturnSlideWidget::Image(s) => {
                    let img = Image::new(s.as_ref());
                    f.render_widget(img, slide_rect);
                }
                // _ => {}
            }
        }
        Ok(())
    }
} 
