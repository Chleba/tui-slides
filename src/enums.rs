use ratatui::{layout::Rect, widgets::Paragraph};
use serde::{Deserialize, Serialize};
use tui_big_text::BigText;

pub enum ReturnSlideWidget<'a> {
    Paragraph(Paragraph<'a>),
    BigText(BigText<'a>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SlideContentType {
    Paragraph,
    BigText,
    // Image,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContentJson {
    #[serde(rename = "type")]
    pub type_: SlideContentType,
    pub content: String,
    pub rect: Rect,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SlideJson {
    pub title: String,
    pub content: ContentJson, 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoxSizeJson {
    pub percent_width: u16,
    pub percent_height: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SlidesJson {
    pub box_size: BoxSizeJson, 
    pub slides: Vec<SlideJson>,
}
