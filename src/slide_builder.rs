use std::{path::Path, str::FromStr};

use crate::enums::{ContentJson, ReturnSlideWidget, SlideContentType, SlideJson};
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::terminal::size;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::*,
    style::Stylize,
    text::Line,
    widgets::{
        block::{self, Title}, Block, BorderType, Borders, Paragraph, WidgetRef
    },
};
use ratatui_image::{picker::Picker, Image, Resize, StatefulImage};
use tui_big_text::BigText;

pub fn get_slide_content_string(slide: ContentJson) -> String {
    let mut content_str = String::from("");
    if let Some(cv) = slide.content {
        content_str = cv;
    }
    content_str
}

fn get_slide_content_color(slide: ContentJson) -> String {
    if let Some(c) = slide.color {
        return c;
    }
    String::from("#FF0000")
}

fn make_slide_paragraph<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide.clone());
    let color = get_slide_content_color(slide);
    ReturnSlideWidget::Paragraph(Paragraph::new(content).style(Style::default().fg(Color::from_str(&color).unwrap())))
}

fn make_slide_line<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide.clone());
    let color = get_slide_content_color(slide);
    ReturnSlideWidget::Line(
        Line::from(content)
            .style(Style::default().fg(Color::from_str(&color).unwrap_or(Color::Blue))),
    )
}

fn make_slide_bigtext<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide);
    let lines: Vec<Line> = content
        .split('\n')
        .map(|s| Line::from(s.to_string()))
        .collect();
    ReturnSlideWidget::BigText(
        BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Sextant)
            .lines(lines)
            .alignment(ratatui::layout::Alignment::Center)
            .build()
            .unwrap(),
    )
}

pub fn make_slide_image<'a>(slide: ContentJson, slide_path: String) -> ReturnSlideWidget<'a> {
    let f_path = Path::new(&slide_path);
    let img_path = f_path.parent().unwrap();
    let content = get_slide_content_string(slide);
    let dyn_img = image::io::Reader::open(img_path.join(content))
        .unwrap()
        .decode()
        .unwrap();
    ReturnSlideWidget::Image(dyn_img)
}

pub fn make_slide_block<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide.clone());
    let color = get_slide_content_color(slide);
    ReturnSlideWidget::Block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::from_str(&color).unwrap()))
            .title(
                Title::from(Line::from(vec![content.yellow()]))
                    .alignment(Alignment::Right)
                    .position(block::Position::Bottom),
            ),
    )
}

pub fn make_slide_content<'a>(
    slide_content: ContentJson,
    slide_path: String,
) -> ReturnSlideWidget<'a> {
    match slide_content.type_ {
        SlideContentType::Paragraph => make_slide_paragraph(slide_content),
        SlideContentType::BigText => make_slide_bigtext(slide_content),
        SlideContentType::Line => make_slide_line(slide_content),
        SlideContentType::Image => make_slide_image(slide_content, slide_path),
        SlideContentType::Block => make_slide_block(slide_content),
    }
}
