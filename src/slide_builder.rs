use std::{ops::Deref, path::Path, str::FromStr, sync::Arc};

use crate::enums::{ContentJson, ReturnSlideWidget, SlideContentType, SlideJson};
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::terminal::size;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::*,
    style::Stylize,
    text::Line,
    widgets::{
        block::{self, Title},
        Block, BorderType, Borders, Paragraph, Sparkline, WidgetRef,
    },
};
use ratatui_image::{picker::Picker, Image, Resize, StatefulImage};
use tui_big_text::BigText;

pub fn get_slide_content_string(slide: &ContentJson) -> String {
    let mut content_str = String::from("");
    if let Some(cv) = &slide.content {
        content_str = cv.to_string();
    }
    content_str
}

fn get_slide_content_color(slide: &ContentJson) -> String {
    if let Some(c) = &slide.color {
        return c.to_owned();
    }
    String::from("#FF0000")
}

fn get_slide_content_max(slide: &ContentJson) -> u64 {
    if let Some(c) = slide.max {
        return c;
    }
    10
}

// fn get_slide_content_data(slide: &ContentJson) -> Vec<u64> {
//     if let Some(c) = slide.data {
//         return c.to_vec();
//     }
//     vec![0]
// }

// -------------
// -- PARAGRAPH
// -------------
fn make_slide_paragraph<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
    let color = get_slide_content_color(&slide);
    ReturnSlideWidget::Paragraph(
        Paragraph::new(content).style(Style::default().fg(Color::from_str(&color).unwrap())),
    )
}

// -------------
// -- LINE
// -------------
fn make_slide_line<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
    let color = get_slide_content_color(&slide);
    ReturnSlideWidget::Line(
        Line::from(content)
            .style(Style::default().fg(Color::from_str(&color).unwrap_or(Color::Blue))),
    )
}

// -------------
// -- BIGTEXT
// -------------
fn make_slide_bigtext<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
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

// -------------
// -- IMAGE
// -------------
pub fn make_slide_image<'a>(slide: ContentJson, slide_path: String) -> ReturnSlideWidget<'a> {
    let f_path = Path::new(&slide_path);
    let img_path = f_path.parent().unwrap();
    let content = get_slide_content_string(&slide);
    let dyn_img = image::io::Reader::open(img_path.join(content))
        .unwrap()
        .decode()
        .unwrap();
    ReturnSlideWidget::Image(dyn_img)
}

// -------------
// -- BLOCK
// -------------
pub fn make_slide_block<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
    let color = get_slide_content_color(&slide);
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

// -------------
// -- SPARKLINE
// -------------
pub fn make_slide_sparkline<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
    let color = get_slide_content_color(&slide);
    let max = get_slide_content_max(&slide);
    // let data = get_slide_content_data(&slide).to_owned();

    ReturnSlideWidget::Sparkline(
        Sparkline::default()
            // .data(&data)
            // .data(data)
            .max(max)
            .style(Style::default().red().on_black()),
    )
}

// -------------
// -- CODE HIGHLIGHT
// -------------
pub fn make_slide_code_highlight<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(&slide);
    let color = get_slide_content_color(&slide);

    ReturnSlideWidget::CodeHighlight(Paragraph::new(content))
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
        SlideContentType::Sparkline => make_slide_sparkline(slide_content),
        SlideContentType::CodeHighlight => make_slide_code_highlight(slide_content),
    }
}
