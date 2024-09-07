use image::DynamicImage;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Sparkline},
};
use serde::{Deserialize, Serialize};
use tui_big_text::BigText;

// #[derive(Debug)]
pub enum ReturnSlideWidget<'a> {
    Paragraph(Paragraph<'a>),
    BigText(BigText<'a>),
    Line(Line<'a>),
    Image(DynamicImage),
    Block(Block<'a>),
    Sparkline(Sparkline<'a>),
    CodeHighlight(Paragraph<'a>),
    // CodeHighlight(Line<'a>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SlideContentType {
    Paragraph,
    BigText,
    Line,
    Image,
    Block,
    Sparkline,
    CodeHighlight,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContentJson {
    #[serde(rename = "type")]
    pub type_: SlideContentType,
    pub content: Option<String>,
    pub rect: Option<Rect>,
    pub color: Option<String>,
    pub data: Option<Vec<u64>>,
    pub max: Option<u64>,
}

impl Default for ContentJson {
    fn default() -> Self {
        Self {
            type_: SlideContentType::Line,
            content: None,
            rect: None,
            color: None,
            data: None,
            max: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SlideJson {
    pub title: Option<String>,
    pub content: Vec<ContentJson>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoxSizeJson {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SlidesJson {
    pub box_size: BoxSizeJson,
    pub slides: Vec<SlideJson>,
}
